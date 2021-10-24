use egui::{self, Area, Color32, Frame, Label, Resize, ScrollArea};


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
fn background_color(background_color: impl Into<Color32>) {

}

// change txt color (white)
fn text_color(text_color: impl Into<Color32>) {

}

fn main() {
    
}