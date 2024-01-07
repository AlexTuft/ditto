use crate::common::Save;
use core_ux::u24;

pub fn read_u8(save: &Save, offset: usize) -> u8 {
    let data = &save.data;
    data[offset]
}

pub fn read_u16_be(save: &Save, offset: usize) -> u16 {
    let data = &save.data;
    ((data[offset] as u16) << 8) | data[offset + 1] as u16
}

pub fn read_u16_le(save: &Save, offset: usize) -> u16 {
    let data = &save.data;
    ((data[offset + 1] as u16) << 8) | data[offset] as u16
}

pub fn read_u24_be(save: &Save, offset: usize) -> u24 {
    let data = &save.data;
    (u24::from(data[offset]) << 16)
        | (u24::from(data[offset + 1]) << 8)
        | u24::from(data[offset + 2])
}

pub fn read_u24_le(save: &Save, offset: usize) -> u24 {
    let data = &save.data;
    (u24::from(data[offset + 2]) << 16) | (u24::from(data[offset + 1]) << 8) | u24::from(data[offset])
}

pub fn read_u32_be(save: &Save, offset: usize) -> u32 {
    let data = &save.data;
    ((data[offset] as u32) << 24)
        | ((data[offset + 1] as u32) << 16)
        | ((data[offset + 2] as u32) << 8)
        | data[offset + 3] as u32
}

pub fn read_u32_le(save: &Save, offset: usize) -> u32 {
    let data = &save.data;
    ((data[offset + 3] as u32) << 24)
        | ((data[offset + 2] as u32) << 16)
        | ((data[offset + 1] as u32) << 8)
        | data[offset] as u32
}
