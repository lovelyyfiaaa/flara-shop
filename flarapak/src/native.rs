///
/// Native implementation of the
///
/// This requires a Linux-like environment in the program that has Flatpak installed in it. So WebAssembly is not supported by this one. You can try the flathub-beta-web backend however!!
pub struct NativeBackend {}

use std::fs::File;

use std::io::Read;

use std::path::PathBuf;

use std::process::{Command, ExitStatus};

use crate::schemas::appstream::Components;
use crate::schemas::remote::{deserialize, Remote};
use crate::{prelude, schemas};
pub use NativeBackend as Backend;

#[derive(Debug, Clone)]
pub struct NativeRepository {
    remote: Remote,
    path: PathBuf,
}
pub use NativeRepository as Repository;
pub use Repository as Repo;
impl<'a> prelude::Repository<App> for Repository {
    fn get_apps(&self) -> Vec<App> {
        let path = self.path.join("appstream").join(&self.remote.name);

        if !path.exists() {
            Vec::new()
        } else {
            let arch_folder = path.join(std::env::consts::ARCH).join("active");

            let arch = arch_folder.join("appstream.xml");
            let arch_gz = arch_folder.join("appstream.xml.gz");
            let mut xml = String::new();
            if arch.exists() {
                match File::open(&arch) {
                    Ok(mut file) => file.read_to_string(&mut xml).unwrap(),
                    Err(err) => panic!("{:?} failed: {err:#?}", &arch),
                };
            } else if arch_gz.exists() {
                let mut decoder = flate2::read::GzDecoder::new(File::open(arch_gz).unwrap());

                decoder.read_to_string(&mut xml).unwrap();
            }

            if xml == "" {
                return Vec::new();
            }

            let collection = match parse_from_str(&xml) {
                Ok(c) => c,
                Err(err) => panic!("{:?} failed: {err:#?}", &arch),
            };


            collection
                .components
                .iter()
                .map(|app| NativeApp { app: app.clone() })
                .collect()
        }
    }

    fn name(&self) -> Option<&Self::StringRet> {
        if let Some(xa) = &self.remote.xa {
            xa.title.as_ref()
        } else {
            None
        }
    }

    type StringRet = String;
}

#[derive(Debug, Clone)]
pub struct NativeApp {
    app: schemas::appstream::App,
}
use async_trait::async_trait;
#[async_trait]
impl prelude::App for NativeApp {
    type StringRet = String;
    fn id(&self) -> &Self::StringRet {
        &self.app.id
    }
    fn title(&self) -> &Self::StringRet {
        &self.app.name
    }
    async fn install(&self) -> std::io::Result<ExitStatus> {
        Self::install_id(&self.app.id).await
    }

    async fn install_id(id: &str) -> std::io::Result<ExitStatus> {
        let mut flatpak = Command::new("flatpak");
        flatpak.args(["install", id]);

        flatpak.status()
    }
    fn author(&self) -> Option<&Self::StringRet> {
        self.app.developer_name.as_ref()
    }

    fn description(&self) -> Option<&Self::StringRet> {
        self.app.description.as_ref()
    }

    fn images(&self) -> Vec<&Self::StringRet> {
        let mut images = Vec::new();

        for screenshots in &self.app.screenshots.screenshots {
            images.extend(&screenshots.images);
        }

        images.iter().map(|image| &image.url).collect()
    }
}
use configparser::ini::Ini;
pub use App as Application;
pub use App as NativeApplication;
pub use NativeApp as App;

impl Backend {
    pub const SCAN_FOLDERS: [&'static str; 2] = ["/var/lib/flatpak", "~/local/share/flatpak"];
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use configparser::ini::Ini;

    use crate::native::Repo;
    use crate::prelude::*;
    use crate::schemas::remote::deserialize;
    #[test]
    pub fn test_repo() {
        let mut ini = Ini::new();
        ini.read(include_str!("./schemas/flatpak_repoconfig.ini").to_string())
            .unwrap();
        for remote in deserialize(ini) {
            let repo = Repo {
                remote,
                path: PathBuf::new(),
            };
            assert_eq!(repo.name(), Some(&"Fedora Flathub Selection".to_string()));
        }
    }
}

impl prelude::Backend<App, Repo> for Backend {
    fn get_repositories() -> Vec<Repo> {
        let mut repositories = Vec::new();
        for str_folder in Self::SCAN_FOLDERS {
            let folder;
            if str_folder.starts_with("~/") {
                folder = dirs::home_dir()
                    .unwrap()
                    .join(str_folder.strip_prefix("~/").unwrap())
            } else {
                folder = PathBuf::from(str_folder)
            }
            if let Ok(path) = Self::repo_config(&folder) {
                let mut ini = Ini::new();
                ini.load(path).unwrap();

                let remotes = deserialize(ini);
                repositories.extend(remotes.iter().map(|remote| Repo {
                    remote: remote.clone(),
                    path: folder.clone(),
                }));
            };
        }

        repositories
    }

    fn get_apps(repos: &Vec<Repo>) -> Vec<App> {
        let mut apps = Vec::new();

        for repo in repos {
            apps.extend(<NativeRepository as prelude::Repository<App>>::get_apps(
                repo,
            ))
        }

        apps
    }
}

impl NativeBackend {
    pub fn repo_config(folder: &PathBuf) -> std::io::Result<PathBuf> {
        folder.join("repo").join("config").canonicalize()
    }
}
