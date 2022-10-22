///
///  This is a template implementation for the Flarapak traits!!
/// 
/// 
pub struct NativeBackend {}
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

impl prelude::Backend<App, Repo> for NativeBackend {
    fn get_repositories() -> Vec<Repo> {
        todo!()
    }
}
