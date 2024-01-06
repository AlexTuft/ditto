use crate::common::Species;
use core_ux::u24;

/**
 * Struct for raw Pokemon data from Gen III games.
*  See https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_data_structure_in_Generation_III
 */
struct PokemonData {
    personality_value: u32,
    ot_id: u16,
    nickname: [u8; 10],
    language: u8,
    misc_flags: u8,
    ot_name: [u8; 7],
    markings: u8,
    checksum: u16,
    data: [u8; 48],
    status_condition: u32,
    level: u8,
    mail_id: u8,
    current_hp: u16,
    total_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special_attack: u16,
    special_defense: u16,
}
