use std::env::args;

mod app;
mod instance;

/// just a collection of supported appids for if I impliment modding for anything that isn't skyrim
#[allow(unused)]
enum Appids {
    SkyrimSE,
}

// the implementation for conversion between the enum and it's appid counterpart
impl Into<i32> for Appids {
    fn into(self) -> i32 {
        match self {
            Self::SkyrimSE => 489830,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), i8> {
    let args: Vec<String> = args().skip(1).collect();
    // gui code:
    if args.contains(&"--gui".to_string()) {
        app::ModManager::run(iced::Size::new(800.0, 800.0));
    } else if args.contains(&"--install".to_string()) {
        let install_arg_pos = args.iter().position(|item| item == "--install");

        let nxm_link = match args.get(install_arg_pos.unwrap() + 1) {
            Some(v) => v,
            None => {
                println!("No nxm link provided!");
                return Err(-1);
            }
        };
        println!("nxm link found: {nxm_link}");
        return Ok(());
    } else {
        if let Some(v) = interactions::steam::find_game(Appids::SkyrimSE.into()) {
            println!("{v}");
        }

        instance::ensure_data_dir_init();

        return Ok(());
    }
    Ok(())
}
