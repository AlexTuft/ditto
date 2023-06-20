use super::data::*;
use super::game_data::*;
use super::save::SaveData;
use phf::phf_map;
use std::collections::HashMap;

const OT_NAME_MAX_LEN: usize = 11;
const NICKNAME_MAX_LEN: usize = 11;

static GOLD_SILVER_PARTY_INFO_LAYOUT: phf::Map<&str, usize> = phf_map! {
    "count" => 0x288A,
    "dex_nos" => 0x288B,
    "mon_data" => 0x2892,
    "ot_names" => 0x29B2,
    "nicknames" => 0x29F4,
};

static GOLD_SILVER_POKEMON_INFO_LAYOUT: phf::Map<&str, usize> = phf_map! {
    "dex_no" => 0x0,
    "item" => 0x1,
    "moves" => 0x2,
    "ot_id" => 0x6,
    "exp" => 0x8,
    "ev_hp" => 0x0B,
    "attack_ev" => 0x0D,
    "defense_ev" => 0x0F,
    "speed_ev" => 0x11,
    "special_ev" => 0x13,
    "ivs" => 0x15,
    "pp" => 0x17,
    "friendship" => 0x1B,
    "pokerus" => 0x1C,
    "caught" => 0x1D,
    "level" => 0x1F,
    // Items below this are only stored for party pokemon.
    "status" => 0x20,
    "current_hp" => 0x22,
    "max_hp" => 0x24,
    "attack" => 0x26,
    "defense" => 0x28,
    "speed" => 0x2A,
    "special_attack" => 0x2C,
    "special_defense" => 0x2E,
};

pub struct Pokemon {
    pub dex_no: u8,
    pub item: u8,
    pub moves: Vec<u8>,
    pub ot_id: u16,
    pub exp: u32,
    pub ev_hp: u16,
    pub attack_ev: u16,
    pub defense_ev: u16,
    pub speed_ev: u16,
    pub special_ev: u16,
    pub ivs: u16,
    pub pp: Vec<u8>,
    pub friendship: u8,
    pub pokerus: u8,
    pub caught: u16,
    pub level: u8,
    pub status: u8,
    pub current_hp: u16,
    pub max_hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub ot_name: String,
    pub nickname: String,
}

impl Pokemon {
    pub fn species_name(&self) -> &str {
        SPECIES[self.dex_no as usize - 1].name
    }

    pub fn item(&self) -> &str {
        ITEMS[self.item as usize]
    }

    pub fn moves(&self) -> Vec<&str> {
        vec![
            MOVES[self.moves[0] as usize],
            MOVES[self.moves[1] as usize],
            MOVES[self.moves[2] as usize],
            MOVES[self.moves[3] as usize],
        ]
    }
}

pub fn get_party_pokemon(save_data: &SaveData) -> Vec<Pokemon> {
    let count = read_u8(save_data, GOLD_SILVER_PARTY_INFO_LAYOUT["count"]) as usize;
    (0..count)
        .map(|x| {
            let pokemon_data_addr = GOLD_SILVER_PARTY_INFO_LAYOUT["mon_data"] + x * 48;
            let mut pokemon = get_pokemon_data(save_data, pokemon_data_addr);

            let ot_name_addr = GOLD_SILVER_PARTY_INFO_LAYOUT["ot_names"] + x * OT_NAME_MAX_LEN;
            pokemon.ot_name = read_string(save_data, ot_name_addr, OT_NAME_MAX_LEN);

            let nickname_addr = GOLD_SILVER_PARTY_INFO_LAYOUT["nicknames"] + x * OT_NAME_MAX_LEN;
            pokemon.nickname = read_string(save_data, nickname_addr, NICKNAME_MAX_LEN);

            pokemon
        })
        .collect()
}

fn get_pokemon_data(save_data: &SaveData, addr: usize) -> Pokemon {
    let layout: HashMap<&'static str, usize> = GOLD_SILVER_POKEMON_INFO_LAYOUT
        .entries()
        .into_iter()
        .map(|(k, v)| (*k, v + addr))
        .collect();
    Pokemon {
        dex_no: read_u8(save_data, layout["dex_no"]),
        item: read_u8(save_data, layout["item"]),
        moves: vec![
            read_u8(save_data, layout["moves"]),
            read_u8(save_data, layout["moves"] + 1),
            read_u8(save_data, layout["moves"] + 2),
            read_u8(save_data, layout["moves"] + 3),
        ],
        ot_id: read_u16_be(save_data, layout["ot_id"]),
        exp: read_u24_be(save_data, layout["exp"]),
        ev_hp: read_u16_be(save_data, layout["ev_hp"]),
        attack_ev: read_u16_be(save_data, layout["attack_ev"]),
        defense_ev: read_u16_be(save_data, layout["defense_ev"]),
        speed_ev: read_u16_be(save_data, layout["speed_ev"]),
        special_ev: read_u16_be(save_data, layout["special_ev"]),
        ivs: read_u16_be(save_data, layout["ivs"]),
        pp: vec![
            read_u8(save_data, layout["pp"]),
            read_u8(save_data, layout["pp"] + 1),
            read_u8(save_data, layout["pp"] + 2),
            read_u8(save_data, layout["pp"] + 3),
        ],
        friendship: read_u8(save_data, layout["friendship"]),
        pokerus: read_u8(save_data, layout["pokerus"]),
        caught: read_u16_be(save_data, layout["caught"]),
        level: read_u8(save_data, layout["level"]),
        status: read_u8(save_data, layout["status"]),
        current_hp: read_u16_be(save_data, layout["current_hp"]),
        max_hp: read_u16_be(save_data, layout["max_hp"]),
        attack: read_u16_be(save_data, layout["attack"]),
        defense: read_u16_be(save_data, layout["defense"]),
        speed: read_u16_be(save_data, layout["speed"]),
        special_attack: read_u16_be(save_data, layout["special_attack"]),
        special_defense: read_u16_be(save_data, layout["special_defense"]),
        ot_name: String::from(""),
        nickname: String::from(""),
    }
}
