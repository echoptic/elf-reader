use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::utils::sum_offset;

pub struct ProgramHeader {
    pub p_type: ProgramType,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_allign: u64,
}

fn get_type(value: u32) -> ProgramType {
    for ty in ProgramType::iter() {
        if ty as u32 == value {
            return ty;
        }
    }
    panic!("Invalid Program Type!")
}

impl ProgramHeader {
    pub fn new(data: &[u8], architecture: u8) -> Self {
        let size;
        let mut offs;

        if architecture == 1 {
            size = 4;
            offs = 0;
        } else {
            size = 8;
            offs = 4;
        };

        let p_flags = if architecture == 1 {
            sum_offset(&data, 0x04, 4)
        } else {
            sum_offset(&data, 0x18, 4)
        };

        let p_type = get_type(sum_offset(&data, offs, 4));
        offs += 4;

        if architecture == 2 {
            offs += 4
        }

        let p_offset = sum_offset(&data, offs, size);
        offs += size;

        let p_vaddr = sum_offset(&data, offs, size);
        offs += size;

        let p_paddr = sum_offset(&data, offs, size);
        offs += size;

        let p_filesz = sum_offset(&data, offs, size);
        offs += size;

        let p_memsz = sum_offset(&data, offs, size);
        offs += size;

        if architecture == 1 {
            offs += 4;
        }

        let p_allign = sum_offset(&data, offs, size);

        Self {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_allign,
        }
    }
}

#[derive(EnumIter, Clone, Copy)]
pub enum ProgramType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    ShLib,
    PHdr,
    Tls,
    LoOS,
    HiOS,
    LoProc,
    HiProc,
}
