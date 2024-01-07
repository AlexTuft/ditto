use crate::common::Save;
use crate::data_utils::{read_u16_be, read_u24_be, read_u8};
use core_ux::u24;

const GOLD_SILVER_PARTY_DATA_OFFSETS: phf::Map<&str, usize> = phf::phf_map! {
    "count" => 0x288A,
    "species_indices" => 0x288B,
    "mon_data" => 0x2892,
    "ot_names" => 0x29B2,
    "nicknames" => 0x29F4,
};

const GOLD_SILVER_POKEMON_DATA_SIZE: usize = 48;

const GOLD_SILVER_POKEMON_DATA_FIELD_OFFSETS: phf::Map<&str, usize> = phf::phf_map! {
    "species_index" => 0x0,
    "item" => 0x1,
    "moves" => 0x2,
    "original_trainer_id" => 0x6,
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
    "status" => 0x20,
    "current_hp" => 0x22,
    "max_hp" => 0x24,
    "attack" => 0x26,
    "defense" => 0x28,
    "speed" => 0x2A,
    "special_attack" => 0x2C,
    "special_defense" => 0x2E
};

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
    ivs: u16,
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

fn read_party_pokemon_count(save: &Save) -> u8 {
    save.data[GOLD_SILVER_PARTY_DATA_OFFSETS["count"]]
}

fn read_party_pokemon_data(save: &Save, slot: u8) -> Option<PokemonData> {
    if slot >= read_party_pokemon_count(save) {
        return None;
    }

    let party_pokemon_data_offset =
        GOLD_SILVER_PARTY_DATA_OFFSETS["mon_data"] + slot as usize * GOLD_SILVER_POKEMON_DATA_SIZE;

    let mut pokemon_data = read_pokemon_data(
        save,
        party_pokemon_data_offset,
        GOLD_SILVER_POKEMON_DATA_FIELD_OFFSETS,
    );
    Some(pokemon_data)
}

fn read_pokemon_data(
    save: &Save,
    offset: usize,
    field_offsets: phf::Map<&str, usize>,
) -> PokemonData {
    PokemonData {
        species_index: read_u8(save, offset + field_offsets["species_index"]),
        held_item: read_u8(save, offset + field_offsets["item"]),
        moves: [
            read_u8(save, offset + field_offsets["moves"]),
            read_u8(save, offset + field_offsets["moves"] + 1),
            read_u8(save, offset + field_offsets["moves"] + 2),
            read_u8(save, offset + field_offsets["moves"] + 3),
        ],
        original_trainer_id: read_u16_be(save, offset + field_offsets["original_trainer_id"]),
        experience: read_u24_be(save, offset + field_offsets["exp"]),
        hp_ev: read_u16_be(save, offset + field_offsets["ev_hp"]),
        attack_ev: read_u16_be(save, offset + field_offsets["attack_ev"]),
        defense_ev: read_u16_be(save, offset + field_offsets["defense_ev"]),
        speed_ev: read_u16_be(save, offset + field_offsets["speed_ev"]),
        special_ev: read_u16_be(save, offset + field_offsets["special_ev"]),
        ivs: read_u16_be(save, offset + field_offsets["ivs"]),
        move_pps: [
            read_u8(save, offset + field_offsets["pp"]),
            read_u8(save, offset + field_offsets["pp"] + 1),
            read_u8(save, offset + field_offsets["pp"] + 2),
            read_u8(save, offset + field_offsets["pp"] + 3),
        ],
        friendship_or_remaining_egg_cycles: read_u8(save, offset + field_offsets["friendship"]),
        pokerus: read_u8(save, offset + field_offsets["pokerus"]),
        caught_data: read_u16_be(save, offset + field_offsets["caught"]),
        level: read_u8(save, offset + field_offsets["level"]),
        status_condition: read_u8(save, offset + field_offsets["status"]),
        current_hp: read_u16_be(save, offset + field_offsets["current_hp"]),
        max_hp: read_u16_be(save, offset + field_offsets["max_hp"]),
        attack: read_u16_be(save, offset + field_offsets["attack"]),
        defense: read_u16_be(save, offset + field_offsets["defense"]),
        speed: read_u16_be(save, offset + field_offsets["speed"]),
        special_attack: read_u16_be(save, offset + field_offsets["special_attack"]),
        special_defense: read_u16_be(save, offset + field_offsets["special_defense"]),
        ot_name: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        nickname: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        is_egg: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_party_count() {
        let save = crate::read_save("test-saves/pokesilver.sav").unwrap();
        let count = read_party_pokemon_count(&save);
        assert_eq!(6, count);
    }

    #[test]
    fn read_party_pokemon_species() {
        let save = crate::read_save("test-saves/pokesilver.sav").unwrap();
        let pokemon_1 = read_party_pokemon_data(&save, 0).unwrap();
        let pokemon_2 = read_party_pokemon_data(&save, 1).unwrap();
        let pokemon_3 = read_party_pokemon_data(&save, 2).unwrap();
        let pokemon_4 = read_party_pokemon_data(&save, 3).unwrap();
        let pokemon_5 = read_party_pokemon_data(&save, 4).unwrap();
        let pokemon_6 = read_party_pokemon_data(&save, 5).unwrap();

        assert_eq!(64, pokemon_1.species_index); // Kadabra
        assert_eq!(160, pokemon_2.species_index); // Feraligatr
        assert_eq!(17, pokemon_3.species_index); // Pidgeotto
        assert_eq!(33, pokemon_4.species_index); // Nidorino
        assert_eq!(185, pokemon_5.species_index); // Sudowoodo
        assert_eq!(130, pokemon_6.species_index); // Gyarados
    }

    #[test]
    fn read_party_pokemon_items() {
        let save = crate::read_save("test-saves/pokesilver.sav").unwrap();
        let pokemon_1 = read_party_pokemon_data(&save, 0).unwrap();
        let pokemon_2 = read_party_pokemon_data(&save, 1).unwrap();
        let pokemon_3 = read_party_pokemon_data(&save, 2).unwrap();
        let pokemon_4 = read_party_pokemon_data(&save, 3).unwrap();
        let pokemon_5 = read_party_pokemon_data(&save, 4).unwrap();
        let pokemon_6 = read_party_pokemon_data(&save, 5).unwrap();

        assert_eq!(0, pokemon_1.held_item); // None
        assert_eq!(91, pokemon_2.held_item); // Amulet Coin
        assert_eq!(83, pokemon_3.held_item); // Bitter Berry
        assert_eq!(17, pokemon_4.held_item); // Super Potion
        assert_eq!(63, pokemon_5.held_item); // Ether
        assert_eq!(80, pokemon_6.held_item); // Ice Berry
    }

    #[test]
    fn read_party_pokemon_moves() {
        let save = crate::read_save("test-saves/pokesilver.sav").unwrap();
        let pokemon_1 = read_party_pokemon_data(&save, 0).unwrap();
        let pokemon_2 = read_party_pokemon_data(&save, 1).unwrap();
        let pokemon_3 = read_party_pokemon_data(&save, 2).unwrap();
        let pokemon_4 = read_party_pokemon_data(&save, 3).unwrap();
        let pokemon_5 = read_party_pokemon_data(&save, 4).unwrap();
        let pokemon_6 = read_party_pokemon_data(&save, 5).unwrap();

        assert_eq!(60, pokemon_1.moves[0]); // Psybeam
        assert_eq!(148, pokemon_1.moves[1]); // Flash
        assert_eq!(248, pokemon_1.moves[2]); // Future Sight
        assert_eq!(50, pokemon_1.moves[3]); // Disable

        assert_eq!(15, pokemon_2.moves[0]); // Cut
        assert_eq!(163, pokemon_2.moves[1]); // Slash
        assert_eq!(44, pokemon_2.moves[2]); // Bite
        assert_eq!(57, pokemon_2.moves[3]); // Surf

        assert_eq!(33, pokemon_3.moves[0]); // Tackle
        assert_eq!(19, pokemon_3.moves[1]); // Fly
        assert_eq!(16, pokemon_3.moves[2]); // Gust
        assert_eq!(98, pokemon_3.moves[3]); // Quick Attack

        assert_eq!(40, pokemon_4.moves[0]); // Poison Sting
        assert_eq!(31, pokemon_4.moves[1]); // Fury Attack
        assert_eq!(30, pokemon_4.moves[2]); // Horn Attack
        assert_eq!(24, pokemon_4.moves[3]); // Double Kick

        assert_eq!(88, pokemon_5.moves[0]); // Rock Throw
        assert_eq!(70, pokemon_5.moves[1]); // Strength
        assert_eq!(175, pokemon_5.moves[2]); // Flail
        assert_eq!(67, pokemon_5.moves[3]); // Low Kick

        assert_eq!(37, pokemon_6.moves[0]); // Thrash
        assert_eq!(44, pokemon_6.moves[1]); // Bite
        assert_eq!(82, pokemon_6.moves[2]); // Dragon Rage
        assert_eq!(43, pokemon_6.moves[3]); // Leer
    }
}
