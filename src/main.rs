use std::env::args;

mod interactions_api;

/// just a collection of supported appids for if I impliment modding for anything that isn't skyrim
#[allow(unused)]
enum Appids {
    SkyrimSEAppid
}

// the implementation for conversion between the enum and it's appid counterpart
impl Into<i32> for Appids {
    fn into(self) -> i32 {
        match self {
            Self::SkyrimSEAppid => 489830,
        }
    }
}

#[tokio::main]
async fn main() {

    // gui code:
    if args().collect::<Vec<String>>().contains(&"--gui".to_string()) {
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
    } else {
        if let Some(v) = interactions_api::steam::find_game(Appids::SkyrimSEAppid.into()) {
            println!("{v}");
        }
    }
}

