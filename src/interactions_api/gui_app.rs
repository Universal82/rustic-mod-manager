/////////////////////////////
// don't look, it's a mess //
/////////////////////////////

use std::collections::HashMap;

use iced::mouse::Interaction::Text;
use iced::widget::{column, Column, button, checkbox, row, text};
use iced::{Element, Fill, color};

#[derive(Debug, Clone, Default)]
pub struct ModDef {
    pub name: String
}

impl ModDef {
    fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AddMod(ModDef)
}

#[derive(Default)]
pub struct ModAdder {
    pub mod_data: ModDef,
    pub mod_name: String
}

impl ModAdder {
    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Column<'_, Message> {
        column![
            text("Add Mod").center(),
        ].width(Fill)
    }
}

#[derive(Default)]
pub struct ModManager {
    pub mods: HashMap<String, ModDef>
}

impl ModManager {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddMod(mod_data, ) => {
                iced::run(ModAdder::update, ModAdder::view).expect("msg");
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        column![
            button(text("Add").align_x(iced::Alignment::Center)).on_press(Message::AddMod(ModDef::new("name"))).width(Fill)
        ].width(Fill)
    }
}