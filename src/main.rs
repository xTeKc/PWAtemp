// use iced::{
//     button,
//     Button,
//     Column,
//     Sandbox,
//     Settings,
// }; 

// struct Search {
//     // search button value
//     value: String,

//    // local state of search button
//    search_button: button::State
// }

// // user interactions for search
// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     SearchPressed
// }

// // implement with view logic
// impl Search {
//     pub fn view(&mut self) -> Column<Message> {
//         Column::new()
//             .push(
//                 // produces the message when pressed
//                 Button::new(&mut self.search_button, Text::new("^"))
//                         .on_press(Message::SearchPressed),
//             )
//     }

//     pub fn update(&mut self, message: Message) {
//         match message {
//             Message::SearchPressed => {
//                 self.value = "Search".to_string();
//             }
//         }
//     }
// }

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
