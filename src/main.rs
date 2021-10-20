use iced::*; 

struct Search {
    // search button value
    value: String,

   // local state of search button
   search_button: button::State
}

pub enum Message {
    SearchPressed
}

fn main() {

}