use super::data::*;
use super::save::*;
use phf::phf_map;

static GOLD_SILVER_TRAINER_INFO_LAYOUT: phf::Map<&str, usize> = phf_map! {
    "id" => 0x2009,
    "name" => 0x200B,
    "money" => 0x23DB,
};

pub struct TrainerInfo {
    pub id: u16,
    pub name: String,
    pub money: u32,
}

pub fn get_trainer_info(save_data: &SaveData) -> TrainerInfo {
    TrainerInfo {
        id: read_u16(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["id"]),
        name: read_string(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["name"], 7),
        money: read_u24(save_data, GOLD_SILVER_TRAINER_INFO_LAYOUT["money"]),
    }
}
