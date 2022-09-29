use serde_derive::{Serialize, Deserialize};

use std::{fs::{self, File}, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct HolydayManager {
    pub user: String,
    normal_holyday: u32,
    bank_holyday: u32,
    pub default_normal_holyday: u32,
    pub default_bank_holyday: u32,
}

impl HolydayManager {
    pub fn new() -> Self {
        if !Path::new("./data.json").exists() {
            File::create("./data.json")
                .expect("Can't init file");

            let new_data: HolydayManager = HolydayManager{
                user: "new_user".to_owned(),
                normal_holyday: 22,
                bank_holyday: 9,
                default_normal_holyday: 22,
                default_bank_holyday: 9,
            };

            new_data.save_json();
        }

        let file_str: String = fs::read_to_string("./data.json")
            .expect("The file is not here so fuck it");

        serde_json::from_str(&file_str)
            .expect("Can't serialize to load_json")
    }

    pub fn get_user(self: &HolydayManager) -> &String {
        &self.user
    }

    pub fn get_remaining_days(self: &HolydayManager) -> u32 {
        self.normal_holyday + self.bank_holyday
    }

    pub fn get_normal_days(self: &HolydayManager) -> &u32 {
        &self.normal_holyday
    }

    pub fn get_bank_days(self: &HolydayManager) -> &u32 {
        &self.bank_holyday
    }

    pub fn get_default_normal_days(self: &HolydayManager) -> &u32 {
        &self.default_normal_holyday
    }

    pub fn get_default_bank_days(self: &HolydayManager) -> &u32 {
        &self.default_bank_holyday
    }

    pub fn use_days(self: &mut HolydayManager, nb_days: u32) {
        if nb_days > self.get_remaining_days() {
            self.normal_holyday = 0;
            self.bank_holyday = 0;
        }
        else if nb_days >= self.normal_holyday {
            self.bank_holyday -= nb_days - self.normal_holyday;
            self.normal_holyday = 0;
        }
        else {
            self.normal_holyday -= nb_days
        }

        self.save_json();
    }

    pub fn add_normal_days(self: &mut HolydayManager, nb_days: u32) {
        self.normal_holyday += nb_days;

        self.save_json();
    }

    pub fn add_bank_days(self: &mut HolydayManager, nb_days: u32) {
        self.normal_holyday += nb_days;

        self.save_json();
    }

    pub fn reset_used_days(self: &mut HolydayManager) {
        self.normal_holyday = self.default_normal_holyday;
        self.bank_holyday = self.default_bank_holyday;

        self.save_json();
    }

    pub fn save_json(self: &HolydayManager) {
        let file: File = File::create("./data.json")
            .expect("Can't open the data file");

        serde_json::to_writer_pretty(&file, self)
            .expect("Can't write in the data file")
    }
}
