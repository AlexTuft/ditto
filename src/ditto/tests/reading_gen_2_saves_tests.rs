#[test]
fn reads_party_count_correctly() {
    let save = crate::read_save("test-saves/pokesilver.sav");
    let count = crate::gen2::read_party_pokemon_count(&save);
    assert_eq!(6, count);
}
