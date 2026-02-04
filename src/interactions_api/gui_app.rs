/////////////////////////////
// don't look, it's a mess //
/////////////////////////////


pub static APP_NAME: &str = "Rustic Skyrim Mod Manager";

pub static IMAGE_BYTES: &[u8] = include_bytes!("../assets/icon128x128.png");

#[derive(Clone)]
pub struct ModEntry {
    pub mod_name: String,
    pub mod_path: String,
    pub enabled: bool,
    pub to_be_deleted: bool,
}

#[derive(Default)]
pub struct ModManagerApp {
    mods: Vec<ModEntry>,
    top_bar_buttons: Vec<(String,bool)>
}

impl eframe::App for ModManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_bar_buttons = vec![
            ("Add Dummy".to_string(), false),
            ("I am a button".to_string(), false),
            ("I am a button".to_string(), false),
        ];

        ctx.set_pixels_per_point(1.5);
    
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("Rustic Skyrim Mod Manager");
            ui.columns(self.top_bar_buttons.len(), |button| {
                for i in 0..self.top_bar_buttons.len() {
                    if button[i].button(self.top_bar_buttons[i].0.clone()).clicked() {
                        self.top_bar_buttons[i].1 = true;
                    }
                }
            });
            ui.separator();

            // page content
            ui.columns(2, |column| {
                column[0].label("EEE");
                column[1].label("AAA");
                column[0].set_width(200.0);
            });
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };

            if self.top_bar_buttons[0].1 {
                self.mods.push(ModEntry { mod_name: "DummyMod".to_string(), mod_path: "DummyMod".to_string(), enabled: true, to_be_deleted: false });
                self.top_bar_buttons[0].1 = false;
            };
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Iterate over the items in your data structure
                for item in &mut self.mods {
                    // Create a horizontal layout for items to place widgets on the same row
                    ui.horizontal(|ui| {
                        // Add widgets for each item (e.g., a checkbox and a label)
                        ui.checkbox(&mut item.enabled, &item.mod_name);

                        // Add a button with an action
                        if ui.button("Delete").clicked() {
                            // Handle deletion logic here
                            // Note: You cannot directly remove an item while iterating mutably. 
                            // A common pattern is to collect actions and apply them after the loop.
                        }
                    });
                    ui.separator(); // Add a separator after each item
                }
            });
        });

    }

    
}