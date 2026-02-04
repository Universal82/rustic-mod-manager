use std::env::args;

mod interactions_api;

/// just a collection of supported appids for if I impliment modding for anything that isn't skyrim
#[allow(unused)]
enum Appids {
    SkyrimSE
}

// the implementation for conversion between the enum and it's appid counterpart
impl Into<i32> for Appids {
    fn into(self) -> i32 {
        match self {
            Self::SkyrimSE => 489830,
        }
    }
}

//

#[tokio::main]
async fn main() -> Result<(),i8> {

    let args: Vec<String> = args().skip(1).collect();

    // used for non-interactive cli usage, to specify not to query the user when you normally would 
    let no_user = args.contains(&"--no-user".to_string());

    // gui code:
    if args.contains(&"--gui".to_string()) {
        /////////////////////////////
        // don't look, it's a mess //
        /////////////////////////////
        
        //*
        use crate::interactions_api::gui_app::*;

        use egui::IconData;

        let icon = {
            //let image_bytes = include_bytes!("assets/icon32x32.png");
            let image = image::load_from_memory(IMAGE_BYTES)
                .expect("Failed to load icon from memory")
                .to_rgba8();
    
            let (width, height) = image.dimensions();
            IconData {
                rgba: image.into_vec(),
                width,
                height,
            }
        };
    
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size((1600.0, 900.0)).with_icon(icon),
            ..eframe::NativeOptions::default()
        };
    
        eframe::run_native(
            APP_NAME,
            native_options,
            Box::new(|_| Ok(Box::<interactions_api::gui_app::ModManagerApp>::default())),
            ).expect("Idk what to expect");
        // */
    } else if args.contains(&"--install".to_string()) {
        let install_arg_pos = args.iter().position(|item|{
            item == "--install"
        });

        let nxm_link = match args.get(install_arg_pos.unwrap()+1) {
            Some(v) => v,
            None => {
                println!("No nxm link provided!");
                return Err(-1);
            }
        };
        println!("nxm link found: {nxm_link}");
        return Ok(());
    } else {
        if let Some(v) = interactions_api::steam::find_game(Appids::SkyrimSE.into()) {
            println!("{v}");
        }
        return Ok(());
    }
    Ok(())
}

