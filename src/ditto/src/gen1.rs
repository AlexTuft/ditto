use core_ux::u24;

/**
 * Struct for raw Pokemon data from Gen I games.
*  See https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_data_structure_in_Generation_I
 */
struct Gen1PokemonData {
    index_number: u8, // The index number is not the same as the Pokedex number
    current_hp: u16,
    level_pc: u8, // This is the level of the Pokemon when stored in the PC
    status: u8,
    type1: u8, // These seem to be reduntant, as the type is determined by the index number
    type2: u8,
    catch_rate: u8,
    moves: [u8; 4],
    original_trainer_id: u16,
    experience: u24,
    hp_ev: u8,
    attack_ev: u8,
    defense_ev: u8,
    speed_ev: u8,
    special_ev: u8,
    ivs: u16,
    move_pps: [u8; 4],
    // Everything below this is only present in Pokemon that are in the player's party
    level: u8, // This is the calculated level of the Pokemon when placed in the party
    max_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special: u16,
    // Additional data for all Pokemon
    nickname: [u8; 10],
    ot_name: [u8; 10],
}
