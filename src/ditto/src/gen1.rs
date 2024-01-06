use crate::common::Species;
use core_ux::u24;

/**
 * Struct for raw Pokemon data from Gen I games.
*  See https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_data_structure_in_Generation_I
*
*  Notes:
*  - species_index: In Gen I the index number does not match the national dex number
*  - level_box: Redundant. Keep in sync with level
*  - type1: Redundant, because types are always the same for a given species
*  - type2: Same as type1 if the Pokemon only has one type
*  - catch_rate_or_held_item: In Gen I this is the catch rate, not the held item
*    if caught in Gen II. Catch rate is not necessarily the same as the catch rate for  the species
*    because it doesn't update when a Pokemon evolves, and the evolution may have a different catch
*    rate.
* -
*/
struct PokemonData {
    // Data available to Pokemon in the player's party and PC
    species_index: u8,
    current_hp: u16,
    level_box: u8,
    status_condition: u8,
    type1: u8,
    type2: u8,
    catch_rate_or_held_item: u8,
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
    // Data only available to Pokemon in the player's party
    level: u8,
    max_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special: u16,
    // Additional data available to Pokemon in the player's party
    // and PC
    nickname: [u8; 10],
    ot_name: [u8; 10],
}
