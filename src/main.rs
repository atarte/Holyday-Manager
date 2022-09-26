use eframe::egui;
use serde_derive::{Serialize, Deserialize};
use std::fs;

fn main() {
    let option = eframe::NativeOptions::default();

    eframe::run_native(
        "Joao Awesome Holyday Manager", 
        option, 
        Box::new(|_cc| Box::new(HolydayManager::default())),
    );
}

fn load_json() -> HolydayManager {
    let file_str: String = fs::read_to_string("./data.json")
        .expect("The file is not here so fuck it");

    serde_json::from_str(&file_str)
        .expect("Can't serialize to load_json")
}

fn save_json() {

}

#[derive(Debug, Serialize, Deserialize)]
struct HolydayManager {
    user: String,
    normal_holyday: u32,
    bank_holyday: u32,
    default_normal_holyday: u32,
    default_bank_holyday: u32,
}

impl HolydayManager {
    fn get_remaining_days(self: &HolydayManager) -> u32 {
        self.normal_holyday + self.bank_holyday
    }

    fn get_normal_days(self: &HolydayManager) -> u32 {
        self.normal_holyday
    }

    fn get_bank_days(self: &HolydayManager) -> u32 {
        self.bank_holyday
    }

    fn use_day(self: &mut HolydayManager) {
        if self.normal_holyday != 0 {
            self.normal_holyday -= 1;
        }
        else {
            self.bank_holyday -= 1;
        }
    }

    fn reser_used_days(self: &mut HolydayManager) {
        self.normal_holyday = self.default_normal_holyday;
        self.bank_holyday = self.default_bank_holyday;
    }
}

impl Default for HolydayManager {
   fn default() -> Self {
       load_json()
   } 
}

impl eframe::App for HolydayManager {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal(|ui| {
                ui.heading("Cool");
         
                if ui.button("Reset").clicked() {
                    // mettre une popup
                    self.reser_used_days();
                }
            });

            ui.separator();

            egui::Grid::new("DayGrid")
                .striped(true)
                // .min_col_width
                .show(ui, |ui| {

                ui.label("User:");
                ui.label(&self.user);
                ui.end_row();

                ui.label("Days remaining:");
                ui.label(self.get_remaining_days().to_string());
                ui.end_row();

                ui.label("Normal days remaining:");
                ui.label(self.get_normal_days().to_string());
                ui.end_row();

                ui.label("Bank days remaining:");
                ui.label(self.get_bank_days().to_string());
                ui.end_row();

                ui.label("Remove days:");
                ui.horizontal(|ui| {
                    let mut can_reduce_day: bool = true;
                    if self.get_remaining_days() == 0 {
                        can_reduce_day = false;
                    }

                    ui.add_enabled_ui(can_reduce_day, |ui| {
                        if ui.button("Remove").clicked() {
                            self.use_day();
                        }
                    });
                });
                ui.end_row();

                ui.collapsing("More options", |ui| {
                    ui.horizontal(|ui| {

                    });
                });
            });
        });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     let header: String = "User: ".to_owned() + &self.user;
        //     ui.heading(header);
        //
        //
        //
        // });
    }
}
