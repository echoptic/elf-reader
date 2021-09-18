use std::{fs, io};

use crate::{
    file_header::FileHeader, program_header::ProgramHeader, section_header::SectionHeader,
};

pub struct Elf {
    pub file_header: FileHeader,
    pub program_header: ProgramHeader,
    pub section_header: SectionHeader,
}

impl Elf {
    pub fn from_file(path: &str) -> io::Result<Self> {
        let data = fs::read(path)?;

        Ok(Elf::from_data(&data))
    }

    pub fn from_data(data: &[u8]) -> Self {
        let file_header = FileHeader::new(&data);

        let data = if file_header.ei_class == 1 {
            &data[0x34..]
        } else {
            &data[0x40..]
        };

        let program_header = ProgramHeader::new(&data, file_header.ei_class);

        let data = if file_header.ei_class == 1 {
            &data[0x20..]
        } else {
            &data[0x38..]
        };

        let section_header = SectionHeader::new(&data, file_header.ei_class);

        Self {
            file_header,
            program_header,
            section_header,
        }
    }
}
