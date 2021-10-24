use egui::{self, Area, Frame, Label, Resize, ScrollArea};


pub struct Window<'open> {
    title_label: Label,
    open: Option<&'open mut bool>,
    area: Area,
    frame: Option<Frame>,
    resize: Resize,
    scroll: Option<ScrollArea>,
    collapsible: bool,
    with_title_bar: bool,
}

// change bg color (black)
pub fn background_color(self, background_color: impl Into<Color32>) -> Self {

}

// change txt color (white)
pub fn text_color(self, text_color: impl Into<Color32>) -> Self {
    
}

fn main() {
    
}