use iced::{Color, Element, Sandbox, Settings, Text};

struct Browser;

impl Sandbox for Browser {
    type Message = ();

    fn new() -> Browser {
        Browser
    }

    fn title(&self) -> String {
        String::from("PWA Browser")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Browser").into()
    }

    fn background_color(&self) -> iced::Color {
        Color::BLACK
    }
}

pub fn main() -> iced::Result {
    Browser::run(Settings::default())
}
