use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, Settings, Text,
};

pub fn main() -> iced::Result {
    Randomizer::run(Settings::default())
}

struct Randomizer {
    generate: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    Generate,
}

impl Application for Randomizer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Randomizer, Command<Message>) {
        (
            Randomizer {
                generate: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("SSBU Randomizer")
    }

    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .max_width(600)
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new("Generate a randomized list of fighters!").size(20))
            .push(
                Button::new(&mut self.generate, Text::new("Generate")).on_press(Message::Generate),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
