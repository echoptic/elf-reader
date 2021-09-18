use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::utils::sum_offset;

pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: SectionHeaderType,
    pub sh_flags: SectionHeaderFlags,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

fn get_type(value: u32) -> SectionHeaderType {
    for ty in SectionHeaderType::iter() {
        if ty as u32 == value {
            return ty;
        }
    }
    panic!("Invalid Section Header Type: {:x}", value)
}

// TODO: Panics here
fn get_flag(value: u32) -> SectionHeaderFlags {
    for flag in SectionHeaderFlags::iter() {
        if flag as u32 == value {
            return flag;
        }
    }
    panic!("Invalid Section Header Flags: {:x}", value)
}

impl SectionHeader {
    pub fn new(data: &[u8], arch: u8) -> Self {
        let size;
        let mut offs = 0;

        let sh_name = sum_offset(&data, offs, 4);
        offs += 4;

        let sh_type = get_type(sum_offset(&data, offs, 4));
        offs += 4;

        if arch == 1 {
            size = 4;
        } else {
            size = 8;
        };

        let sh_flags = get_flag(sum_offset(&data, offs, size));
        offs += size;

        let sh_addr = sum_offset(&data, offs, size);
        offs += size;

        let sh_offset = sum_offset(&data, offs, size);
        offs += size;

        let sh_size = sum_offset(&data, offs, size);
        offs += size;

        let sh_link = sum_offset(&data, offs, size);
        offs += size;

        let sh_info = sum_offset(&data, offs, size);
        offs += size;

        let sh_addralign = sum_offset(&data, offs, size);
        offs += size;

        let sh_entsize = sum_offset(&data, offs, size);

        Self {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        }
    }
}

#[derive(EnumIter, Clone, Copy)]
pub enum SectionHeaderType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    ShLib,
    DynSym,
    InitArray = 0x0e,
    FiniArray = 0x0f,
    PreinitArray = 0x10,
    Group = 0x11,
    SymTabShndx = 0x12,
    Num = 0x13,
    LoOs = 0x60000000,
}

#[derive(EnumIter, Clone, Copy)]
pub enum SectionHeaderFlags {
    Write,
    Alloc,
    Execinstr,
    Merge,
    Strings,
    InfoLink,
    LinkOrder,
    OsNonConforming,
    Group,
    Tls,
    Maskos,
    Maskproc,
    Ordered,
    Exclude,
}
