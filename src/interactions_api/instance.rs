use std::{collections::HashMap, io::Write, path::PathBuf};

use serde::Serialize;

pub const APPLICATION_DATA_DIR: &str = "~/.rsmm";

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Instances {
    instances: HashMap<String, Vec<crate::gui::app::GameMod>>,
}

impl Instances {
    pub fn get_instance_data(self, name: String) -> Option<Vec<crate::gui::app::GameMod>> {
        self.instances.get(&name).cloned()
    }
}

fn absolve_home_paths(v: &str) -> String {
    if v.chars().collect::<Vec<char>>()[0] == '~' {
        if let Some(home) = std::env::home_dir() {
            (String::from(home.to_str().unwrap()) + v.split_at(1).1).to_owned()
        } else {
            v.to_string()
        }
    } else {
        v.to_string()
    }
}

fn mkdir_many(dirs: Vec<&str>) {
    dirs
        .iter()
        .map(|v|{absolve_home_paths(v)})
        .for_each(|v|{std::fs::create_dir_all(v);}); 
}

pub fn ensure_data_dir_init() {
    // Directories
    let dirs = vec![
        "~/.rsmm/instances",
        "~/.rsmm/mods/ar",
        "~/.rsmm/mods/unpacked",
    ];
    mkdir_many(dirs);

    // Files
    let dummy_config = crate::gui::app::Config {
        ..Default::default()
    };
    let path = absolve_home_paths("~/.rsmm/config.json");
    std::fs::File::create(&path);
    let mut file = std::fs::OpenOptions::new().write(true).append(false).open(path).unwrap();
    let dummy_string = serde_json::to_string_pretty(&dummy_config).unwrap();
    file.write_all(dummy_string.as_bytes());
}