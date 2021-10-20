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

use iced::{
    Element,
    Sandbox,
    Settings,
    Text
};

pub fn main() -> iced::Result {
    Hello::run(Settings::default())
}

struct Hello;

impl Sandbox for Hello {
    type Message = ();

    fn new() -> Hello {
        Hello
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}