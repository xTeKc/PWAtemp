use iced::*; 

struct Search {
    // search button value
    value: String,

   // local state of search button
   search_button: button::State
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SearchPressed
}

fn main() {

}