use crate::remote::Remote;

const FLATHUB: &'static str = "https://beta.flathub.org/_next/data/j0obREkOgE5WQXkgSo-m5/";

pub struct FlathubBetaStore {
    pub url: String,
}

impl Remote for FlathubBetaStore {
    fn get_application(app_id: &str) -> crate::app::App {
        todo!()
    }
}
