///
/// Native implementation of the
///
/// This requires a Linux-like environment in the program that has Flatpak installed in it. So WebAssembly is not supported by this one. You can try the flathub-beta-web backend however!!
pub struct NativeBackend {}
use std::path::PathBuf;

use crate::prelude;
use crate::schemas::remote::{deserialize, Remote};
pub use NativeBackend as Backend;
pub struct NativeRepository {
    remote: Remote,
}
pub use NativeRepository as Repository;
pub use Repository as Repo;
impl prelude::Repository<App> for Repository {
    fn get_apps() -> Vec<App> {
        todo!()
    }

    fn name(&self) -> Option<String> {
        if let Some(xa) = &self.remote.xa {
            xa.title.clone()
        } else {
            None
        }
    }
}
pub struct NativeApp {}
impl prelude::App for NativeApp {}
use configparser::ini::Ini;
pub use App as Application;
pub use App as NativeApplication;
pub use NativeApp as App;

impl Backend {
    pub const SCAN_FOLDERS: [&'static str; 2] = ["/var/lib/flatpak", "~/local/share/flatpak"];
}

#[cfg(test)]
mod test {
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
            let repo = Repo { remote };
            assert_eq!(repo.name(), Some("Fedora Flathub Selection".to_string()));
        }
    }
}

impl prelude::Backend<App, Repo> for Backend {
    fn get_repositories() -> Vec<Repo> {
        for folder in Self::SCAN_FOLDERS {
            let path = PathBuf::from(folder).join("repo").join("config");

            
        }

        todo!()
    }
}
