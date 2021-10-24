use egui::{self, Area, Color32, Frame, Id, Label, Order, Resize, ScrollArea};


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

pub struct LayerId {
    pub order: Order,
    pub id: Id,
}

fn main() {
    
}