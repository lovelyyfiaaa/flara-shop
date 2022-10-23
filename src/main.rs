use app::App;
use flarapak::auto::{Backend, Repo};
use flarapak::prelude::{Backend as B, Repository};
use iced::widget::qr_code::{self, QRCode};
use iced::widget::{column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};
mod app;
mod flathub;
mod remote;
pub fn main() -> iced::Result {
    FlauraShop::run(Settings::default())
}

#[derive(Default)]
struct FlauraShop {
    view: View,
    repositories: Vec<Repo>,
}

#[derive(Debug, Clone)]
enum View {
    App(App),
    Dev(String),
    Publisher(String),
    Home,
    Category(String),
}

impl Default for View {
    fn default() -> Self {
        Self::Home
    }
}
enum Category {}

#[derive(Debug, Clone)]
enum Message {
    Open(View),
    Install(App),
}

impl Sandbox for FlauraShop {
    type Message = Message;

    fn new() -> Self {
        let repositories = Backend::get_repositories();
        Self {
            repositories,
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Flara Shop")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Open(view) => {
                self.view = view;
            }
            Message::Install(app) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        column(vec![
            row(vec![text("Welcome to Flara Shop!").into()]).into(),
            scrollable(column(vec![column(vec![
                text("New apps").into(),
                row(self
                    .repositories
                    .iter()
                    .map(|app| {
                        column(vec![text(
                            &app.name().unwrap_or_else(|| "No name".to_string()),
                        )
                        .into()])
                        .into()
                    })
                    .collect())
                .into(),
            ])
            .into()]))
            .into(),
        ])
        .into()
    }
}
