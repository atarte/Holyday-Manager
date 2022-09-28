use eframe::{egui, epaint::Vec2};

mod holyday_manager;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 200.0;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Option::from(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)); 

    eframe::run_native(
        "Joao Awesome Holyday Manager", 
        options, 
        Box::new(|_cc| Box::new(AppData::default())),
    );
}

struct AppData {
    days_to_remove: u32,
    normal_days_to_add: u32,
    bank_days_to_add: u32,
    data: holyday_manager::HolydayManager,
}

impl AppData {
    fn reset_normal_days_to_add(self: &mut AppData) {
        self.normal_days_to_add = 1;
    }

    fn reset_bank_days_to_add(self: &mut AppData) {
        self.bank_days_to_add = 1;
    }

    fn reset_days_to_remove(self: &mut AppData) {
        self.days_to_remove = 1;
    }
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            days_to_remove: 1,
            normal_days_to_add: 1,
            bank_days_to_add: 1,
            data: holyday_manager::HolydayManager::new(),
        }
    } 
}

impl eframe::App for AppData {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {

            // });

                ui.heading("Holyday Manager");

                ui.separator();

                egui::Grid::new("DayGrid")
                    .striped(true)
                    .min_col_width(WINDOW_WIDTH/2.0)
                    .max_col_width(WINDOW_WIDTH/2.0)
                    .show(ui, |ui| {

                    ui.label("User:");
                    ui.label(self.data.get_user());
                    ui.end_row();

                    ui.label("Days remaining:");
                    ui.label(self.data.get_remaining_days().to_string());
                    ui.end_row();

                    ui.label("Normal days remaining:");
                    ui.label(self.data.get_normal_days().to_string());
                    ui.end_row();

                    ui.label("Bank days remaining:");
                    ui.label(self.data.get_bank_days().to_string());
                    ui.end_row();

                    ui.label("Remove days:");
                    ui.horizontal(|ui| {
                        let mut can_reduce_day: bool = true;
                        if self.data.get_remaining_days() == 0 {
                            can_reduce_day = false;
                        }
                        
                        ui.add(egui::DragValue::new(&mut self.days_to_remove)
                            .clamp_range(0..=self.data.get_remaining_days())
                        );

                        ui.add_enabled_ui(can_reduce_day, |ui| {
                            if ui.button("Remove").clicked() {
                                self.data.use_days(self.days_to_remove);
                                
                                self.reset_days_to_remove();
                            }
                        });
                    });
                    ui.end_row();

                    ui.collapsing("Parameters", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Set username :");
                            ui.text_edit_singleline(&mut self.data.user);
                            if ui.button("Save").clicked() {
                                self.data.save_json();
                            }
                        });

                        ui.horizontal(|ui| {
                            let title_reset: String = format!("Reset (default {}, {}):", self.data.get_default_normal_days(), self.data.get_default_bank_days());
                            ui.label(title_reset);
                            if ui.button("Reset").clicked() {
                                self.data.reset_used_days();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Add normal Holyday:");
                            ui.add(egui::DragValue::new(&mut self.normal_days_to_add)
                                .clamp_range(0..=20) 
                            );

                            if ui.button("Add").clicked() {
                                self.data.add_normal_days(self.normal_days_to_add);
                                self.reset_normal_days_to_add();
                            }
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Add bank Holyday:");
                            ui.add(egui::DragValue::new(&mut self.bank_days_to_add)
                                .clamp_range(0..=20) 
                            );

                            if ui.button("Add").clicked() {
                                self.data.add_bank_days(self.bank_days_to_add);
                                self.reset_bank_days_to_add();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Set default normal days :");
                            ui.add(egui::DragValue::new(&mut self.data.default_normal_holyday)
                                .clamp_range(0..=365)
                            );
                            if ui.button("Save").clicked() {
                                self.data.save_json();
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Set default bank days :");
                            ui.add(egui::DragValue::new(&mut self.data.default_bank_holyday)
                                .clamp_range(0..=365)
                            );
                            if ui.button("Save").clicked() {
                                self.data.save_json();
                            }
                        });
                    });
                });
            });
        });
    }
}
