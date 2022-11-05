use flarapak::auto::{Backend, Repo};
use flarapak::native::NativeRepository;
use flarapak::prelude::{App as A, Backend as B, Repository};

use iced::application::Appearance;
use iced::widget::{column, container, row, scrollable, text, text_input, button};
use iced::{executor, Application, Color, Command, Element, Renderer, Settings, Theme};

pub fn main() -> iced::Result {
    FlauraShop::run(Settings::default())
}

#[derive(Default)]
struct FlauraShop {
    view: View,
    searching_for: String,
    repositories: Vec<Repo>,
}

#[derive(Debug, Clone)]
enum View {
    App(App),
    Dev(String),
    Publisher(String),
    Home,
    Error(String, String, Option<String>),
    Search(String),
    SearchView(Vec<flarapak::auto::App>),
    MyApps,
    Category(String),
    Loading(String),
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
    Search(String),
    InstallStarted(App),
}
#[derive(Debug, Clone)]
enum App {
    App(String),
    Repository(NativeRepository),
}

impl Application for FlauraShop {
    type Message = Message;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let repositories = Backend::get_repositories();
        (
            Self {
                repositories,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Flara Shop")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::Open(view) => {
                self.view = view;

                Command::none()
            }
            Message::Install(app) => {
                let app = app.clone();

                todo!()
                /*
                Command::perform(App::install_id(), |_| Message::InstallStarted)
                    .map(|apps| Message::Open(View::SearchView(apps))) */
            }
            Message::Search(term) => {
                self.searching_for = term.clone();
                let mut term = term.as_str().to_owned();

                Command::perform(
                    async move {
                        let mut flatpak = std::process::Command::new("flatpak");
                        flatpak.args(["search", &term]);

                        match flatpak.output() {
                            Ok(output) => match String::from_utf8(output.stdout) {
                                Ok(list) => {
                                    let list = list.split("\n");
                                    let apps = Backend::get_apps(
                                        Backend::get_repositories().iter().collect(),
                                    );
                                    println!("{apps:#?}");
                                    let apps = list
                                        .map(|id| {
                                            apps[apps
                                                .iter()
                                                .position(|app| app.id() == id)
                                                .unwrap()]
                                            .clone()
                                        })
                                        .collect();

                                    View::SearchView(apps)
                                }
                                Err(error) => View::Error(
                                    "We found the apps, but they seem written in alien language!"
                                        .to_owned(),
                                    format!(
                                        "We can't read it trough UTF-8. The error is : {error:#?} ",
                                    ),
                                    None,
                                ),
                            },
                            Err(error) => View::Error(
                                "Failed to search the apps".to_string(),
                                format!("{error:#?}"),
                                None,
                            ),
                        }
                    },
                    |output| Message::Open(output),
                )
            }
            Message::InstallStarted(_) => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let view: Element<Message, Renderer> = match &self.view {
            View::App(app) => {
                let default;
                let name = match app {
                    App::App(app) => app,
                    App::Repository(repo) => {
                        if let Some(name) = repo.name() {
                            name
                        } else {
                            default = "No name".to_string();
                            &default
                        }
                    }
                };

                row(vec![text(name).into()]).into()
            }
            View::Dev(_) => todo!(),
            View::Publisher(_) => todo!(),
            View::Home => {
                let repos = Backend::get_repositories();
                let apps = Backend::get_apps(repos.iter().collect());

                column(vec![
                    text("All apps").into(),
                    row(apps
                        .iter()
                        .map(|app| row(vec![text(app.title()).into()]).into())
                        .collect())
                    .into(),
                ])
                .into()
            }
            View::Category(_) => todo!(),
            View::Search(search) => text("ok").into(),

            View::MyApps => column(vec![column(vec![
                text("Your Repositories").into(),
                row(self
                    .repositories
                    .iter()
                    .map(|app| {
                        let name;
                        let default;
                        if let Some(namee) = app.name() {
                            name = namee;
                        } else {
                            default = "No name".to_string();
                            name = &default;
                        }
                        let element: Element<Message, Renderer> =
                            button(column(vec![text(name).into()]))
                                .padding(25)
                                .on_press(Message::Open(View::App(App::Repository(app.clone()))))
                                .into();

                        element.explain(Color::from_rgb(233.0, 70.0, 134.0))
                    })
                    .collect())
                .spacing(15)
                .into(),
            ])
            .into()])
            .into(),
            View::Loading(string) => text(string).into(),
            View::Error(title, description, help) => row(vec![
                text(title).into(),
                text(description).into(),
                button("help").into(),
            ])
            .into(),
            View::SearchView(apps) => {
                row(apps.iter().map(|app| text(app.title()).into()).collect()).into()
            }
        };

        column(vec![
            row(vec![text_input(
                "Search something",
                &self.searching_for,
                |search| Message::Search(search),
            )
            .into()])
            .into(),
            scrollable(container(view)).into(),
            column(vec![
                button("Home").on_press(Message::Open(View::Home)).into(),
                button("My Stuffs!")
                    .on_press(Message::Open(View::MyApps))
                    .into(),
            ])
            .into(),
        ])
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::Custom(|theme| Appearance {
            background_color: Color::from_rgba(255.0, 229.0, 241.0, 50.0),
            text_color: Color::BLACK,
        })
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn should_exit(&self) -> bool {
        false
    }

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();
}

enum FlatpakState {
    Unfetched(String),
    Fetched,
}
