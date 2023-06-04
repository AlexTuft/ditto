use super::data::*;
use super::save::*;

static GOLD_SILVER_CHECKSUM_1_RANGE: (usize, usize) = (0x2009, 0x2D68);
static GOLD_SILVER_CHECKSUM_1: usize = 0x2D69;

static GOLD_SILVER_CHECKSUM_2_RANGES: [(usize, usize); 5] = [
    (0xC6B, 0x10E8 - 1),
    (0x10E8, 0x15C7 - 1),
    (0x15C7, 0x17ED - 1),
    (0x3D96, 0x3F40 - 1),
    (0x7E39, 0x7E6D - 1),
];
static GOLD_SILVER_CHECKSUM_2: usize = 0x7E6D;

pub fn calculate_checksums(save_data: &mut SaveData) {
    let checksum1 = (GOLD_SILVER_CHECKSUM_1_RANGE.0..=GOLD_SILVER_CHECKSUM_1_RANGE.1)
        .map(|x| save_data.data[x] as u16)
        .reduce(|acc, e| acc.wrapping_add(e))
        .unwrap();
    write_u16_le(checksum1, save_data, GOLD_SILVER_CHECKSUM_1);

    let checksum2 = GOLD_SILVER_CHECKSUM_2_RANGES.iter()
        .flat_map(|(start, end)| *start..*end)
        .map(|x| save_data.data[x] as u16)
        .reduce(|acc, e| acc.wrapping_add(e))
        .unwrap();
    write_u16_le(checksum2, save_data, GOLD_SILVER_CHECKSUM_2);
}
