use crate::common::Species;
use core_ux::u24;

/**
 * Struct for raw Pokemon data from Gen II games.
 * See https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_data_structure_in_Generation_II
*/
struct PokemonData {
    // Data available to Pokemon in the player's party and PC
    species_index: u8,
    held_item: u8,
    moves: [u8; 4],
    original_trainer_id: u16,
    experience: u24,
    hp_ev: u16,
    attack_ev: u16,
    defense_ev: u16,
    speed_ev: u16,
    special_ev: u16,
    ivs: u32,
    move_pps: [u8; 4],
    friendship_or_remaining_egg_cycles: u8,
    pokerus: u8,
    caught_data: u16, // Only used in Crystal
    level: u8,
    // Data only available to Pokemon in the player's party
    status_condition: u8,
    current_hp: u16,
    max_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special_attack: u16,
    special_defense: u16,
    // Additional data available to Pokemon in the player's party
    // and PC
    nickname: [u8; 10],
    ot_name: [u8; 10],
    is_egg: bool,
}
