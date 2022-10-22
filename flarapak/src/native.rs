///
/// Native implementation of the
///
/// This requires a Linux-like environment in the program that has Flatpak installed in it. So WebAssembly is not supported by this one. You can try the flathub-beta-web backend however!!
pub struct NativeBackend {}
use std::path::PathBuf;

use crate::prelude;
pub use NativeBackend as Backend;
pub struct NativeRepository {}
pub use NativeRepository as Repository;
pub use Repository as Repo;
impl prelude::Repository<App> for Repository {
    fn get_apps() -> Vec<App> {
        todo!()
    }
}
pub struct NativeApp {}
impl prelude::App for NativeApp {}
pub use App as Application;
pub use App as NativeApplication;
pub use NativeApp as App;

impl Backend {
    pub const SCAN_FOLDERS: [&'static str; 2] = ["/var/lib/flatpak", "~/local/share/flatpak"];
}
impl prelude::Backend<App, Repo> for Backend {
    fn get_repositories() -> Vec<Repo> {
        for folder in Self::SCAN_FOLDERS {
            let path = PathBuf::from(folder).join("repo").join("config");

            
        }

        todo!()
    }
}
