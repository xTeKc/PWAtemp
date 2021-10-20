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
impl Search {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(
                // produces the message when pressed
                Button::new(&mut self.search_button, Text::new("^"))
                        .on_press(Message::SearchPressed),
            )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchPressed => {
                self.value = "Search".to_string();
            }
        }
    }
}

fn main() {

}