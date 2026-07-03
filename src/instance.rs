use std::{collections::HashMap, default, io::Write, ops::Index, path::PathBuf, slice::Iter};

use serde::Serialize;

pub const APPLICATION_DATA_DIR: &str = "~/.rsmm";

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Instance {
    pub name: String,
    pub desc: String,
    pub mods: Vec<crate::app::GameMod>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Instances {
    instances: Vec<Instance>,
}

impl<'life> Instances {
    pub fn from_disk() -> Self {
        let mut out = Self::default();
        for item in std::fs::read_dir(absolve_home_paths("~/.rsmm/instances")).unwrap() {
            if let Ok(item) = item {
                let file_type = item.file_type().unwrap();
                if file_type.is_file() {
                    let data = std::fs::read_to_string(item.path()).unwrap();
                    let struct_data: Instance = serde_json::from_str(data.as_str()).unwrap();
                    out.add(struct_data);
                }
            }
        }

        out
    }

    pub fn get(self, name: String) -> Option<Instance> {
        self.instances.iter().find(|n|{
            if n.name == name {
                return true;
            } else {
                return false;
            }
        }).cloned()
    }

    pub fn get_mut(&mut self, name: String) -> Option<&mut Instance> {
        self.instances.iter_mut().find(|n|{
            if n.name == name {
                return true;
            } else {
                return false;
            }
        })
    }

    pub fn add(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    pub fn iter(&'life self ) -> Iter<'life, Instance> {
        self.instances.iter()
    }
}

pub fn absolve_home_paths(v: &str) -> String {
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
    let dummy_config = crate::app::Config {
        ..Default::default()
    };
    let path = absolve_home_paths("~/.rsmm/config.json");
    std::fs::File::create(&path);
    let mut file = std::fs::OpenOptions::new().write(true).append(false).open(path).unwrap();
    let dummy_string = serde_json::to_string_pretty(&dummy_config).unwrap();
    file.write_all(dummy_string.as_bytes());
}