
pub mod types {
    use std::{any::Any, error::Error};

    use serde_json::{Value, json};
    
    #[derive(Debug)]
    pub struct GameNotInstalled {
        appid: i32
    }
    
    impl std::fmt::Display for GameNotInstalled {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(println!("Error, could not find app with id {} in any known Steam libraries", self.appid))
        }
    }
    
    impl Error for GameNotInstalled {}
    
    impl GameNotInstalled {
        pub fn new(appid: i32) -> Self {
            Self { appid }
        }
    }

    #[derive(serde::Serialize,serde::Deserialize)]
    pub struct InstanceMetadata {
        pub display_name: String,
        pub path: String,
    
    }

    impl Into<serde_json::Value> for InstanceMetadata {
        fn into(self) -> serde_json::Value {
            json!({"display_name": self.display_name, "path": self.path})
        }
    }
    
    pub enum JobStatus {
        Complete,
        FailedGame(GameNotInstalled),
        FailedPermission(std::io::Error),
        FailedParse(serde_json::Error),
        FailedCast(String),
        FailedOther(String)
    }
}

use std::{io::{Read, Write}, str::FromStr};

use serde::{Serialize, de::value};
use serde_json::Value;

use crate::interactions_api::{instance, steam::{self, find_game}};
use crate::interactions_api::instance::types::*;
/// ensures that the game's data directory has been initialized and is ready for instantiation
pub fn ensure_game_init(appid: i32) -> JobStatus {
    // simply get the game path
    let game_path = match find_game(appid) {
        Some(v) => v,
        None => return JobStatus::FailedGame(GameNotInstalled::new(appid))
    };

    // if the mod manager's path doesn't exist, create it
    if !match std::fs::exists(format!("{game_path}/RusticModManager")) {
        Ok(v) => v,
        Err(e) => return JobStatus::FailedPermission(e)
    } {
        match std::fs::create_dir(format!("{game_path}/RusticModManager")) {
            Ok(_) => {},
            Err(e) => return JobStatus::FailedPermission(e)
        };
    }

    // if instances.json doesn't exist then create it
    if !match std::fs::exists(format!("{game_path}/RusticModManager/instances.json")) {
        Ok(v) => v,
        Err(e) => return JobStatus::FailedPermission(e)
    } {
        let mut instances_json = match std::fs::File::create(format!("{game_path}/RusticModManager/instances.json")) {
            Ok(v) => v,
            Err(e) => return JobStatus::FailedPermission(e)
        };

        instances_json.write_all("[]".as_bytes());
    }

    // after everything return complete
    JobStatus::Complete
}

/// install mod to it's relevant game in the active instance
pub fn install_mod(appid: i32, no_user: bool) -> JobStatus {
    // get game path
    let game_path = match find_game(appid) {
        Some(v) => v,
        None => return JobStatus::FailedGame(GameNotInstalled::new(appid))
    };

    
    // after everything return complete
    JobStatus::Complete
}

pub fn create_instance(appid: i32, instance_metadata: InstanceMetadata) -> JobStatus {
    // ensure the game is initialized
    match ensure_game_init(appid) {
        JobStatus::Complete => {},
        JobStatus::FailedGame(e) => return JobStatus::FailedGame(e),
        JobStatus::FailedPermission(e) => return JobStatus::FailedPermission(e),
        JobStatus::FailedParse(e) => return JobStatus::FailedParse(e),
        JobStatus::FailedCast(e) => return JobStatus::FailedCast(e),
        JobStatus::FailedOther(e) => return JobStatus::FailedOther(e)
    }

    // get game path
    let game_path = match find_game(appid) {
        Some(v) => v,
        None => return JobStatus::FailedGame(GameNotInstalled::new(appid))
    };

    // create instance folder
    match std::fs::create_dir_all(format!("{game_path}/RusticModManager/instances/{}", instance_metadata.path)) {
        Ok(_) => {},
        Err(e) => return JobStatus::FailedPermission(e)
    }

    // add instance to instances.json
    {
        // open the file
        let raw_json_data = match std::fs::read_to_string(format!("{game_path}/RusticModManager/instances.json")) {
            Ok(v) => v,
            Err(e) => return JobStatus::FailedPermission(e)
        };

        // parse into json
        let old_json = match Value::from_str(&raw_json_data.as_str()) {
            Ok(v) => v,
            Err(e) => return JobStatus::FailedParse(e)
        };

        // convert to array
        let mut array = match old_json.as_array() {
            Some(v) => v.clone(),
            None => return JobStatus::FailedCast(String::from("Failed to cast instances.json root element to array"))
        };

        // push data to array
        array.push(instance_metadata.into());

        // cast back to json
        let new_json = Value::from(array);

        // convert to pretty string
        let new_raw_json_data = match serde_json::to_string_pretty(&new_json) {
            Ok(v) => v,
            Err(e) => return JobStatus::FailedParse(e)
        };

        // write data to file
        match std::fs::write(format!("{game_path}/RusticModManager/instances.json"), new_raw_json_data) {
            Ok(_) => return JobStatus::Complete,
            Err(e) => return JobStatus::FailedPermission(e)
        };
    }

    // return complete status
    JobStatus::Complete
}