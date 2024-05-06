use iced::executor;
use iced::alignment;
use iced::widget::{
    column, container, scrollable, text,
};
use iced::{Color, Command, Length, Settings};
use iced::{Application, Element, Theme};

pub fn main() -> iced::Result {
    SimpleMath::run(Settings::default())
}

struct SimpleMath;

impl Application for SimpleMath {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (SimpleMath, Command<Self::Message>) {
        (SimpleMath, Command::none())
    }

    fn title(&self) -> String {
        String::from("GUI Simple Math")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("Simple Math")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        let content = column![title]
            .spacing(20)
            .max_width(800);

        scrollable(container(content).padding(40).center_x()).into()
    }
}