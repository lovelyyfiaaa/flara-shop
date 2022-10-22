///
/// A trait that
///
pub trait Repository<A: App> {
    fn get_apps() -> Vec<A>;
}

pub trait Backend<A: App, R: Repository<A>> {
    fn get_repositories() -> Vec<R>;
}

/// A trait that represents all kinds of Flatpak Application!
pub trait App {}
