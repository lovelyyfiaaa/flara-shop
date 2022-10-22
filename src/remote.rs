use crate::app::App;

pub trait Remote {
    fn get_application(app_id: &str) -> App;
}