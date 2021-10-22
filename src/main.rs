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

// change txt color (white)

fn main() {
    
}