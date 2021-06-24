use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use iced::{
    button, executor, scrollable, Align, Application, Button, Clipboard, Column, Command,
    Container, Element, Length, Row, Scrollable, Settings, Space, Text,
};

fn main() -> iced::Result {
    Randomizer::run(Settings::default())
}

// Modeling the state of the application
struct Randomizer {
    generate: button::State,
    list_area: scrollable::State,
    fighter_list: Vec<Fighter>,
}

// Defining the possible user interactions for the randomizer
#[derive(Debug, Clone)]
pub enum Message {
    Generate,
    Toggled(bool),
}

impl Application for Randomizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Randomizer, Command<Message>) {
        (
            Randomizer {
                generate: button::State::new(),
                list_area: scrollable::State::new(),
                fighter_list: Fighter::load_fighters(),
            },
            Command::none(),
        )
    }

    // Title of the application
    fn title(&self) -> String {
        String::from("SSBU Randomizer")
    }

    // Where we update the state of the application
    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match _message {
            Message::Generate => {
                *self = Randomizer {
                    generate: button::State::new(),
                    list_area: scrollable::State::new(),
                    fighter_list: Fighter::randomize_fighters(&self.fighter_list),
                }
            }
            Message::Toggled(_) => {}
        }
        Command::none()
    }

    // View of the application
    fn view(&mut self) -> Element<Message> {
        let content = match self {
            Randomizer {
                generate,
                list_area,
                fighter_list,
            } => Column::new()
                .spacing(50)
                .align_items(Align::Center)
                .push(
                    Column::new()
                        .max_width(600)
                        .spacing(20)
                        .align_items(Align::Center)
                        .push(Text::new("Generate a randomized list of fighters!").size(20))
                        .push(
                            Button::new(generate, Text::new("Generate"))
                                .on_press(Message::Generate)
                                .padding(8)
                                .style(style::Button::Primary),
                        ),
                )
                .push(
                    Column::new()
                        .max_width(600)
                        .max_height(300)
                        .spacing(20)
                        .align_items(Align::Center)
                        .push(
                            Scrollable::new(list_area)
                                .padding(10)
                                .width(Length::Units(500))
                                .scrollbar_width(10)
                                .push({
                                    let mut container = Column::new();
                                    for fighter in fighter_list {
                                        container = container.push(
                                            Row::new()
                                                .push(
                                                    Text::new(format!("{} -", &fighter.name))
                                                        .size(20),
                                                )
                                                .push(
                                                    Text::new(format!(" {} ", &fighter.series))
                                                        .size(20),
                                                )
                                                // .push(Text::new(format!(
                                                //     "- Is echo: {}",
                                                //     fighter.is_echo.to_string()
                                                // )))
                                                .push(Space::with_height(Length::Units(30))),
                                        );
                                    }

                                    container
                                }),
                        ),
                ),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Clone, Debug)]
struct Fighter {
    name: String,
    series: String,
    is_echo: bool,
}

impl Fighter {
    // Returns a list of all fighters from data file
    fn load_fighters() -> Vec<Fighter> {
        // The output is wrapped in a Result to allow matching on errors
        // Returns an Iterator to the Reader of the lines of the file.
        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        let mut fighter_list: Vec<Fighter> = Vec::new();

        if let Ok(lines) = read_lines("./src/fighters.csv") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(fighter) = line {
                    let split: Vec<&str> = fighter.split(",").collect();
                    let fighter = Fighter {
                        name: split[0].to_string(),
                        series: split[1].to_string(),
                        is_echo: split[2] == "true",
                    };
                    fighter_list.push(fighter);
                }
            }
        }
        fighter_list
    }

    fn randomize_fighters(list: &Vec<Fighter>) -> Vec<Fighter> {
        let mut new_list = list.to_vec();
        new_list.shuffle(&mut thread_rng());
        new_list
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 4.0,
                shadow_offset: Vector::new(0.0, 0.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
