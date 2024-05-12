use iced::executor;
use iced::widget::{
    column, text, button, Column, container
};
use iced::{Application, Element, Theme, Command, Settings, Length};

pub fn main() -> iced::Result {
    SimpleMath::run(Settings::default())
}

#[derive(Default, Debug)]
struct SimpleMath {
    value: i64
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Application for SimpleMath {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (SimpleMath, Command<Self::Message>) {
        (
            SimpleMath {
                value: 0
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("GUI Simple Math")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1
        }
    }

    fn view(&self) -> Element<Self::Message> {
        // The buttons
        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);

        // The number
        let counter = text(15);

        let content = Column::new()  // Use Column::new() instead of column! macro
            .push(increment)
            .push(counter)
            .push(decrement);

        // Convert the Column to an Element explicitly
        container(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}