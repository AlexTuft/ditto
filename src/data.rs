use super::save::SaveData;
use super::text_encoding::*;

pub fn read_u16(save_data: &SaveData, addr: usize) -> u16 {
    let data = &save_data.data;

    let b0 = data[addr] as u16;
    let b1 = data[addr + 1] as u16;

    (b0 << 8) | b1
}

pub fn read_u24(save_data: &SaveData, addr: usize) -> u32 {
    let data = &save_data.data;

    let b0 = data[addr] as u32;
    let b1 = data[addr + 1] as u32;
    let b2 = data[addr + 2] as u32;

    (b0 << 16) | (b1 << 8) | b2
}

pub fn read_string(save_data: &SaveData, addr: usize, max_len: usize) -> String {
    let data = &save_data.data;

    let mut string = String::new();

    for i in 0..max_len {
        let ord = data[addr + i];

        if ord == 0x50 {
            break;
        }

        let char = decode_char(ord);
        string.push(char);
    }

    string
}
