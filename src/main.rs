use iced::*; 

struct Search {
    // search button value
    value: String,

   // local state of search button
   search_button: button::State
}

// user interactions for search
#[derive(Debug, Clone, Copy)]
pub enum Message {
    SearchPressed
}

// implement with view logic


fn main() {

}