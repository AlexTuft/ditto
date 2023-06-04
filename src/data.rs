use super::save::SaveData;
use super::text_encoding::*;

pub fn read_u16_be(save_data: &SaveData, addr: usize) -> u16 {
    let data = &save_data.data;

    let b0 = data[addr] as u16;
    let b1 = data[addr + 1] as u16;

    (b0 << 8) | b1
}

pub fn read_u16_le(save_data: &SaveData, addr: usize) -> u16 {
    let data = &save_data.data;

    let b0 = data[addr] as u16;
    let b1 = data[addr + 1] as u16;

    (b1 << 8) | b0
}

pub fn write_u16_be(value: u16, save_data: &mut SaveData, addr: usize) {
    let data = &mut save_data.data;

    data[addr] = ((value >> 8) & 0xff) as u8;
    data[addr + 1] = (value & 0xff) as u8;
}

pub fn write_u16_le(value: u16, save_data: &mut SaveData, addr: usize) {
    let data = &mut save_data.data;

    data[addr] = (value & 0xff) as u8;
    data[addr + 1] = ((value >> 8) & 0xff) as u8;
}

pub fn read_u24_be(save_data: &SaveData, addr: usize) -> u32 {
    let data = &save_data.data;

    let b0 = data[addr] as u32;
    let b1 = data[addr + 1] as u32;
    let b2 = data[addr + 2] as u32;

    (b0 << 16) | (b1 << 8) | b2
}

pub fn write_u24_be(value: u32, save_data: &mut SaveData, addr: usize) {
    let data = &mut save_data.data;

    data[addr] = ((value >> 16) & 0xff) as u8;
    data[addr + 1] = ((value >> 8) & 0xff) as u8;
    data[addr + 2] = (value & 0xff) as u8;
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

pub fn write_string(value: &str, save_data: &mut SaveData, addr: usize, max_len: usize) {
    let data = &mut save_data.data;

    let mut chars = value.chars();
    for i in 0..value.len() {
       data[addr + i] = encode_char(chars.next().unwrap());
   }
   for i in value.len()..max_len {
        data[addr + i] = 0x50;
   }
}
