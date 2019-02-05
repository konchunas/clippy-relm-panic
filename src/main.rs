extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

use relm::Widget;
use relm_attributes::widget;

pub struct Model {
}

#[derive(Msg)]
pub enum Msg {
    #[cfg(test)] Test,
    Decrement,
    Increment,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
    }

    view! {
        gtk::Button {}
    }
}

fn main() {
}