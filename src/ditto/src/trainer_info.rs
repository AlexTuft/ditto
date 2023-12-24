use super::checksum::*;
use super::data::*;
use super::save::*;
use phf::phf_map;

static GOLD_SILVER_TRAINER_INFO_LAYOUT: phf::Map<&str, usize> = phf_map! {
    "id" => 0x2009,
    "name" => 0x200B,
    "money" => 0x23DB,
};

static GOLD_SILVER_TRAINER_INFO_BACKUP_LAYOUT: phf::Map<&str, usize> = phf_map! {
    "id" => 0x15C7,
    "name" => 0x15C9,
    "money" => 0xC6D,
};

pub struct TrainerInfo {
    pub id: u16,
    pub name: String,
    pub money: u32,
}

pub fn get_trainer_info(save_data: &SaveData) -> TrainerInfo {
    TrainerInfo {
        id: read_u16_be(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["id"]),
        name: read_string(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["name"], 7),
        money: read_u24_be(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["money"]),
    }
}

pub fn set_trainer_info(trainer_info: &TrainerInfo, save_data: &mut SaveData) {
    _set_trainer_info(trainer_info, save_data, &GOLD_SILVER_TRAINER_INFO_LAYOUT); 
    _set_trainer_info(trainer_info, save_data, &GOLD_SILVER_TRAINER_INFO_BACKUP_LAYOUT); 
    calculate_checksums(save_data);
}

fn _set_trainer_info(trainer_info: &TrainerInfo, save_data: &mut SaveData, layout: &phf::Map<&'static str, usize>) {
    println!("Writing data to: {}", layout["id"]);
    write_u16_be(trainer_info.id, save_data, layout["id"]);
    println!("Done");
    write_string(trainer_info.name.as_str(), save_data, layout["name"], 7);
    write_u24_be(trainer_info.money, save_data, layout["money"]);
}
