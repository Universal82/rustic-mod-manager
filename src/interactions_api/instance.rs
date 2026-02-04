use std::{any::Any, error::Error};

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

#[derive(serde::Serialize,serde::Deserialize)]
pub struct InstanceMetadata {
    display_name: String,
    path: String,

}

pub enum JobStatus {
    Complete,
    FailedGame(GameNotInstalled),
    FailedPermission(std::io::Error),
}

use crate::interactions_api::{instance, steam::{self, find_game}};
/// ensures that the game's data directory has been initialized and is ready for instantiation
pub fn ensure_game_init(appid: i32) -> JobStatus {
    // simply get the game path
    let game_path = match find_game(appid) {
        Some(v) => v,
        None => return JobStatus::FailedGame(GameNotInstalled { appid })
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

    // after everything return complete
    JobStatus::Complete
}

/// install mod to it's relevant game in the active instance
pub fn install_mod(appid: i32) -> JobStatus {
    // get game path
    let game_path = match find_game(appid) {
        Some(v) => v,
        None => return JobStatus::FailedGame(GameNotInstalled { appid })
    };

    
    // after everything return complete
    JobStatus::Complete
}

pub fn create_instance(appid: i32, instance_metadata: InstanceMetadata) -> JobStatus {

    // return complete status
    JobStatus::Complete
}