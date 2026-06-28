/////////////////////////////
// don't look, it's a mess //
/////////////////////////////

use std::collections::HashMap;
use std::ops::Not;
use std::path::PathBuf;

use iced::Alignment::Center;
use iced::Length::Shrink;
use iced::application::BootFn;
use iced::theme::Style;
use iced::wgpu::wgc::command::CopySide::Source;
use iced::widget::{Checkbox, Column, Row, Text, button, checkbox, column, container, row, scrollable, text, text_editor};
use iced::window::Event::{self, CloseRequested};
use iced::window::{Id, size};
use iced::{Element, Fill, Program, Renderer, Size, Subscription, Theme, color, event, window};
use serde_json::de;

use crate::gui::app::Message::ToggleMod;
use crate::gui::style;
use crate::interactions_api::nexus;

#[derive(Debug, Clone, Default)]
pub enum ModSource {
    #[default]
    None,
    Nexus(nexus::UrlData),
    Local(PathBuf),
}

impl ToString for ModSource {
    fn to_string(&self) -> String {
        match self {
            ModSource::None => "No mod source listed.".to_string(),
            ModSource::Local(_) => format!("File"),
            ModSource::Nexus(_) => format!("Nexus")
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GameMod {
    pub name: String,
    pub source: ModSource,
    pub enabled: bool
}

impl GameMod {
    fn new(name: impl ToString, source: ModSource) -> Self {
        Self {
            name: name.to_string(),
            source: source,
            enabled: true
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = self.enabled.not();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // None case
    None,

    // Mod Management
    AddMod(String),
    ToggleMod(usize),
    AddModEntryText(iced::widget::text_editor::Action),

    // General Application management
    ChangeViewState(ViewState),
    WindowResize(iced::Size),
}

#[derive(Debug, Clone, Default)]
pub enum ViewState {
    #[default]
    ModView,
    Settings,
    AddMod
}

#[derive(Default)]
pub struct ModManager {
    pub window_size: iced::Size,
    pub view_state: ViewState,
    pub mod_uris: text_editor::Content,
    pub mods: Vec<GameMod>,
}

impl ModManager {
    pub fn run(size: iced::Size) {
        iced::application(Self::boot, Self::update, Self::view)
            .window_size(size)
            .subscription(Self::subscription)
            .run()
            .expect("msg");
    }

    pub fn boot() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeViewState(s) => {
                println!("Changing view_state to {s:#?}");
                self.view_state = s;
            },
            Message::AddMod(s) => {
                println!("Adding mod with path/uri: {s}");
                self.mods.push(GameMod::new(s, ModSource::None));
            },
            Message::AddModEntryText(a) => {
                println!("Action: {a:#?}");
                self.mod_uris.perform(a);
            },
            Message::WindowResize(s) => {
                println!("Resized to: {s:#?}");
                self.window_size = s;
            },
            Message::ToggleMod(i) => {
                self.mods[i].toggle();
            },
            Message::None => {}
        }
    }

    // This doesn't really work yet, so I'm not including it until I get a working version
    fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(|(_, size)|{
            Message::WindowResize(size)
        })
    }

    pub fn view(&self) -> Column<'_, Message> {
        column![
            row![
                button("Settings")
                    .on_press(Message::ChangeViewState(ViewState::Settings))
                    .style(style::toolbar::button)
                    .width(Fill),
                button("Mod View")
                    .on_press(Message::ChangeViewState(ViewState::ModView))
                    .style(style::toolbar::button)
                    .width(Fill),
                button("Add Mod(s)")
                    .on_press(Message::ChangeViewState(ViewState::AddMod))
                    .style(style::toolbar::button)
                    .width(Fill),
            ],
            match self.view_state {
                ViewState::Settings => {
                    column![
                        "Settings Screen"
                    ].height(Fill)
                    .width(Fill)
                },
                ViewState::AddMod => {
                    column![
                        text("Enter NXM links and file paths, split by line"),
                        text_editor(&self.mod_uris)
                            .placeholder("nxm://...")
                            .on_action(Message::AddModEntryText)
                            .width(self.window_size.width)
                            .height(200.0),
                        button("Add Dummy Mod")
                            .on_press(Message::AddMod("Dummy Mod".to_string()))
                            .style(style::toolbar::button),
                    ].height(Fill).width(Fill)
                },
                ViewState::ModView => {
                    let mod_list_content = Column::<Message>::with_children(self.mods.iter().enumerate().map(|(i, game_mod)|{
                        row![
                            checkbox::<Message, Theme, Renderer>(game_mod.enabled).on_toggle(move |_|{Message::ToggleMod(i)}),
                            text(game_mod.name.clone()),
                            text(game_mod.source.to_string())
                        ].into()
                    })).spacing(1.0);
                    
                    column![
                        row![
                            container(mod_list_content).width(self.window_size.width * 0.75).height(Fill).style(style::mod_view::container),
                            column![
    
                            ].height(Fill).width(Fill)
                        ].height(Fill).width(Fill),
                    
                    ].height(Fill).width(Fill)
                },
                _ => column![
                    "No ViewState, this shouldn't happen."
                ]
            }
        ]
        .width(Fill)
    }

    // fn toggle_mod(&mut self, name) {

    // }
}

// impl BootFn<ModManager, Message> for ModManager {
//     fn boot(&self) -> (ModManager, iced::Task<Message>) {
//         (
//             Self {
//                 window_size: iced::Size::new(400.0, 400.0),
//                 ..Default::default()
//             },
//             iced::Task<Message>
//         )
//     }
// }
