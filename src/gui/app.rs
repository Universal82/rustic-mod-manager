/////////////////////////////
// don't look, it's a mess //
/////////////////////////////


use std::ops::Not;
use std::path::PathBuf;

use iced::Alignment::Center;
use iced::widget::{Column, button, checkbox, column, container, row, scrollable, space, text, text_editor};
use iced::window;
use iced::{Fill, Padding, Renderer, Subscription, Theme};

use crate::gui::style;
use crate::interactions_api::nexus;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
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
    DeployMods,
    AddMod(GameMod),
    AddModsFromLines(String),
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
    AddMod,
    Instances,
    InstallMod,
    Settings,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub enum AppTheme {
    #[default]
    Light,
    Dark
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub instances: crate::interactions_api::instance::Instances,
    pub theme: AppTheme
}

#[derive(Default)]
pub struct ModManager {
    pub window_size: iced::Size,
    pub view_state: ViewState,
    pub mod_uris: text_editor::Content,
    pub mods: Vec<GameMod>,
    // pub mods_scroll: Viewport,
    pub config: Config,
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
                // println!("Changing view_state to {s:#?}");
                self.view_state = s;
            },
            Message::AddMod(m) => {
                // println!("Adding mod with path/uri: {s}");
                for _ in 1..=10 {
                    self.mods.push(m.clone());
                }
            },
            Message::AddModEntryText(a) => {
                // println!("Action: {a:#?}");
                self.mod_uris.perform(a);
            },
            Message::WindowResize(s) => {
                // println!("Resized to: {s:#?}");
                self.window_size = s;
            },
            Message::ToggleMod(i) => {
                self.mods[i].toggle();
            },
            Message::AddModsFromLines(s) => {
                for line in s.lines() {
                    if line.starts_with("nxm://") {
                        // handle nxm links
                    } else {
                        // handle file paths
                    }
                }
            },
            Message::DeployMods => {
                
            },
            Message::None => {}
        }
    }

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
                    .style(style::light::button)
                    .width(Fill),
                button("Instances")
                    .on_press(Message::ChangeViewState(ViewState::Instances))
                    .style(style::light::button)
                    .width(Fill),
                button("Mod View")
                    .on_press(Message::ChangeViewState(ViewState::ModView))
                    .style(style::light::button)
                    .width(Fill),
                button("Add Mod(s)")
                    .on_press(Message::ChangeViewState(ViewState::AddMod))
                    .style(style::light::button)
                    .width(Fill),
            ],
            match self.view_state {
                ViewState::Settings => {
                    column![
                        "Settings Screen",
                        row![
                            column![

                            ]
                        ]
                    ].height(Fill)
                    .width(Fill)
                },
                
                
                ViewState::AddMod => {
                    column![
                        text("Enter NXM links and file paths, split by line").center(),
                        container(
                            text_editor(&self.mod_uris)
                                .placeholder("nxm://...\n/path/to/mod.archive")
                                .on_action(Message::AddModEntryText)
                                .width(self.window_size.width * 95.0)
                                .height(200.0)
                        ).padding(10.0).style(style::light::container),
                        button("Add Mods")
                            .on_press(Message::AddModsFromLines(self.mod_uris.text()))
                            .style(style::light::button),
                        button("Add Dummy Mod")
                            .on_press(Message::AddMod(GameMod::new("Dummy Mod", ModSource::None)))
                            .style(style::light::button),
                    ].height(Fill).width(Fill)
                },


                ViewState::ModView => {
                    let mut list_bg = true;
                    let mod_list_content = Column::<Message>::with_children(self.mods.iter().enumerate().map(|(i, game_mod)|{
                        container(row![
                            checkbox::<Message, Theme, Renderer>(game_mod.enabled).on_toggle(move |_|{Message::ToggleMod(i)}),
                            space().width(5.0),
                            text(game_mod.name.clone()),
                            text("  |  "),
                            text(game_mod.source.to_string())
                        ].align_y(Center)).width(Fill).style(match list_bg {
                            true => {list_bg = list_bg.not(); style::light::list_container_1},
                            false => {list_bg = list_bg.not(); style::light::list_container_2}
                        }).padding(Padding { top: 2.5, right: 5.0, bottom: 2.5, left: 5.0 }).into()
                    })).spacing(1.0);

                    let control_panel_content = column![
                        button("Deploy Mods").on_press(Message::DeployMods).style(style::light::button),
                    ];
                    
                    column![
                        row![
                            container(
                                scrollable(mod_list_content)
                                    .height(Fill)
                                    .width(Fill)
                                    .style(style::light::scrollable)
                            )
                                .height(Fill)
                                .width(self.window_size.width * (2.0 / 3.0))
                                .style(style::light::container),
                            control_panel_content
                        ].height(Fill).width(Fill),
                    
                    ].height(Fill).width(Fill)
                },


                ViewState::InstallMod => {
                    column![
                        "Install mod ViewState"
                    ]
                },


                ViewState::Instances => {
                    column![
                        "Instance manager"
                    ]
                }
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
