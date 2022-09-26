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

#[derive(Debug, Serialize, Deserialize)]
struct HolydayManager {
    user: String,
    normal_holyday: u8,
    bank_holyday: u8,
}

impl HolydayManager {
    fn remaining_days_to_string(self: &HolydayManager) -> String {
        (self.normal_holyday + self.bank_holyday).to_string()
    }

    fn normal_days_to_string(self: &HolydayManager) -> String {
        self.normal_holyday.to_string()
    }

    fn bank_days_to_string(self: &HolydayManager) -> String {
        self.bank_holyday.to_string()
    }

    fn use_day() {

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

            ui.heading("Cool");

            ui.separator();

            egui::Grid::new("DayGrid")
                .striped(true)
                // .min_col_width
                .show(ui, |ui| {

                ui.label("User:");
                ui.label(&self.user);
                ui.end_row();

                ui.label("Days remaining:");
                ui.label(self.remaining_days_to_string());
                ui.end_row();

                ui.label("Normal days remaining:");
                ui.label(self.normal_days_to_string());
                ui.end_row();

                ui.label("Bank days remaining:");
                ui.label(self.bank_days_to_string());
                ui.end_row();

                ui.label("Remove days:");
                ui.horizontal(|ui| {
                    // let mut days: i32 = 0;
                    // ui.add(egui::DragValue::new(days).speed(1.0));
                    if ui.button("Remove").clicked() {
                        println!("lol");
                    }
                });
                ui.end_row();

                ui.collapsing("More options", |ui| {
                    // ui.horizontal()
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
