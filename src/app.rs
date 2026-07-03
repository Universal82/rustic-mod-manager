/////////////////////////////
// don't look, it's a mess //
/////////////////////////////


use std::error::Error;
use std::io::Read;
use std::ops::Not;
use std::path::PathBuf;
use std::rc::Rc;

use iced::Alignment::Center;
use iced::widget::{Column, button, checkbox, column, container, row, scrollable, space, text, text_editor};
use iced::window;
use iced::{Fill, Padding, Renderer, Subscription, Theme};
use serde::Deserializer;

use crate::instance::{self, Instances};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum ModSource {
    #[default]
    None,
    Nexus(interactions::nexus::UrlData),
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
    ToggleMod(usize),
    AddMod(GameMod),
    AddModsFromLines(String),
    AddInstance(instance::Instance),
    SelectInstance(String),
    
    // General Application management
    ChangeViewState(ViewState),
    WindowResize(iced::Size),
    AddModEntryText(iced::widget::text_editor::Action),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum InstancesState {
    #[default]
    Add,
    Edit,
}

#[derive(Debug, Clone, Default)]
pub enum ViewState {
    #[default]
    ModView,
    AddMod,
    Instances(InstancesState),
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
    pub active_instance: String,
    #[serde(default)]
    pub mods_directory: PathBuf,
    pub theme: AppTheme
}

impl Config {
    pub fn from_disk() -> Self {
        let mut binding = std::fs::OpenOptions::new();
        let file = binding.read(true).write(false);
        let mut file = match file.open(instance::absolve_home_paths("~/.rsmm/config.json")) {
            Ok(v) => v,
            Err(_) => {
                instance::ensure_data_dir_init();
                file.open(instance::absolve_home_paths("~/.rsmm/config.json")).unwrap()
            },
        };
        let mut text_conf = String::new();
        let _ = file.read_to_string(&mut text_conf);
         
        return match serde_json::from_str(&text_conf.as_str()) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e.to_string());
                panic!("Failed for config reasons.")
            }
        };
    }
}

#[derive(Default)]
pub struct ModManager {
    pub window_size: iced::Size,
    pub view_state: ViewState,
    pub text_entries: Vec<String>,
    pub text_editors: Vec<text_editor::Content>,
    pub mods: Vec<GameMod>,
    pub config: Config, // Everything in here will get saved to a config file and restored on
    pub instances: instance::Instances,
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
        let mut out = Self {
            ..Default::default()
        };

        out.config = Config::from_disk();
        out.instances = Instances::from_disk();

        out.text_editors.push(text_editor::Content::new());

        out
    }

    pub fn load_config(&mut self) -> &Self {
        
        self.config = Config::from_disk();

        self
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeViewState(s) => {
                // println!("Changing view_state to {s:#?}");
                match s {
                    // (Re)load instances on opening of instances page
                    ViewState::Instances(..) => self.instances = Instances::from_disk(),
                    _ => {}
                }
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
                self.text_editors[0].perform(a);
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
            Message::AddInstance(i) => {
                self.instances.add(i);
            }
            Message::SelectInstance(i) => {
                self.config.active_instance = i;
                self.update(Message::ChangeViewState(ViewState::Instances(InstancesState::Edit)));
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
                    .style(match self.config.theme {
                        AppTheme::Light => style::light::button,
                        AppTheme::Dark => style::dark::button
                    })
                    .width(Fill),
                button("Instances")
                    .on_press(Message::ChangeViewState(ViewState::Instances(InstancesState::Edit)))
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
                        row![
                            //button().
                        ]
                    ].height(Fill)
                    .width(Fill)
                },
                
                
                ViewState::AddMod => {
                    column![
                        text("Enter NXM links and file paths, split by line").center(),
                        container(
                            text_editor(&self.text_editors[0])
                                .placeholder("nxm://...\n/path/to/mod.archive")
                                .on_action(Message::AddModEntryText)
                                .width(self.window_size.width * 95.0)
                                .height(200.0)
                        ).padding(10.0).style(style::light::container),
                        button("Add Mods")
                            .on_press(Message::AddModsFromLines(self.text_editors[0].text()))
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


                ViewState::Instances(state) => {
                    let mut list_bg = true;
                    let instances = Column::<Message>::with_children(self.instances.iter().map(|i|{
                        container(row![
                            button("Select").on_press(Message::SelectInstance(i.name.clone())).style(style::light::button),
                            space().width(5.0),
                            text(i.name.clone()).align_y(Center)
                        ].align_y(Center)).width(Fill).style(match list_bg {
                            true => {list_bg = list_bg.not(); style::light::list_container_1},
                            false => {list_bg = list_bg.not(); style::light::list_container_2}
                        }).padding(Padding { top: 2.5, right: 5.0, bottom: 2.5, left: 5.0 }).into()
                    }));
                    column![
                        row![
                            container(
                                scrollable(instances).width(Fill).height(Fill).style(style::light::scrollable)
                            ).width(self.window_size.width * 0.25).height(Fill).style(style::light::container),
                            column![
                                row![
                                    button("Add Instance").on_press(Message::ChangeViewState(ViewState::Instances(InstancesState::Add))).width(Fill).style(style::light::button),
                                    button("STB").width(Fill).style(style::light::button),
                                    button("STB").width(Fill).style(style::light::button),
                                ],
                                match state {
                                    InstancesState::Add => column![
                                        "Adding"
                                    ],
                                    InstancesState::Edit => column![
                                        text(String::from("Editing Instance: \"") + self.config.active_instance.clone().as_str() + "\"")
                                    ]
                                }
                            ],
                        ],
                    ]
                }
            }
        ]
        .width(Fill)
    }
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
