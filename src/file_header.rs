use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::utils::sum_offset;

// pub struct ElfIdent {
//     ei_mag0: u8,
//     ei_mag1: u8,
//     ei_mag2: u8,
//     ei_mag3: u8,
//     ei_class: u8, // Specifies 32bit or 64bit format
//     ei_data: u8,  // Specifies little or big endian
//     ei_version: u8,
//     ei_osabi: ElfOsAbi,
//     ei_abiversion: u8,
//     ei_pad: [u8; 8],
// }

pub struct FileHeader {
    pub ei_mag0: u8,
    pub ei_mag1: u8,
    pub ei_mag2: u8,
    pub ei_mag3: u8,
    pub ei_class: u8, // Specifies 32bit or 64bit format
    pub ei_data: u8,  // Specifies little or big endian
    pub ei_version: u8,
    pub ei_osabi: ElfOsAbi,
    pub ei_abiversion: u8,
    pub ei_pad: [u8; 8],
    pub e_type: ElfType,
    pub e_machine: ElfMachine,
    pub e_version: u8,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

fn get_osabi(value: u8) -> ElfOsAbi {
    for abi in ElfOsAbi::iter() {
        if abi as u8 == value {
            return abi;
        }
    }
    panic!("Invalid File Header OsAbi!");
}

fn get_type(value: u16) -> ElfType {
    for ty in ElfType::iter() {
        if ty as u16 == value {
            return ty;
        }
    }
    panic!("Invalid File Header Type: {:x}", value);
}

fn get_machine(value: u16) -> ElfMachine {
    for machine in ElfMachine::iter() {
        if machine as u16 == value {
            return machine;
        }
    }
    panic!("Invalid File Header Machine: {:x}", value);
}

impl FileHeader {
    pub fn new(data: &[u8]) -> Self {
        let ei_mag0 = data[0];
        let ei_mag1 = data[1];
        let ei_mag2 = data[2];
        let ei_mag3 = data[3];
        let ei_class = data[4];
        let ei_data = data[5];
        let ei_version = data[6];
        let ei_osabi = get_osabi(data[7]);
        let ei_abiversion = data[8];
        let ei_pad = [0; 8];

        let mut offs = 10;

        let e_type = get_type(sum_offset(&data, offs, 2));
        offs += 2;
        let e_machine = get_machine(sum_offset(&data, offs, 2));
        offs += 2;
        let e_version = sum_offset(&data, offs, 4);
        offs += 4;

        let e_entry;
        if ei_class == 1 {
            e_entry = sum_offset(&data, offs, 4);
            offs += 4;
        } else {
            e_entry = sum_offset(&data, offs, 8);
            offs += 8;
        };

        offs += if ei_class == 1 { 0x1c } else { 0x20 };

        let e_phoff = if ei_class == 1 {
            sum_offset(&data, offs, 4)
        } else {
            sum_offset(&data, offs, 8)
        };
        offs += if ei_class == 1 { 4 } else { 8 };

        let e_shoff = if ei_class == 1 {
            sum_offset(&data, offs, 4)
        } else {
            sum_offset(&data, offs, 4)
        };
        offs += if ei_class == 1 { 4 } else { 8 };

        let e_flags = sum_offset(&data, offs, 4);
        offs += 4;

        let e_ehsize = sum_offset(&data, offs, 2);
        offs += 2;

        let e_phentsize = sum_offset(&data, offs, 2);
        offs += 2;

        let e_phnum = sum_offset(&data, offs, 2);
        offs += 2;

        let e_shentsize = sum_offset(&data, offs, 2);
        offs += 2;

        let e_shnum = sum_offset(&data, offs, 2);
        offs += 2;

        let e_shstrndx = sum_offset(&data, offs, 2);

        Self {
            ei_mag0,
            ei_mag1,
            ei_mag2,
            ei_mag3,
            ei_class,
            ei_data,
            ei_version,
            ei_osabi,
            ei_abiversion,
            ei_pad,
            e_type,
            e_machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        }
    }
}

#[derive(EnumIter, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum ElfOsAbi {
    SystemV = 0x00,
    HPUX = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNUHurd = 0x04,
    Solaris = 0x06,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0a,
    NovellModesto = 0x0b,
    OpenBSD = 0x0c,
    OpenVMS = 0x0d,
    NonStopKernel = 0x0e,
    AROS = 0x0f,
    FenixOS = 0x10,
    CloudABI = 0x11,
    StratusTechnologiesOpenVOS = 0x12,
}

#[derive(EnumIter, Clone, Copy)]
pub enum ElfType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    LoOS,
    HiOS,
    LoProc,
    HiProc,
}

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum ElfMachine {
    NoSpecificInstructionSet,
    ATnTWE32100,
    Sparc,
    X86,
    Motorola68000M68k,
    Motorola88000M88k,
    IntelMCU,
    Intel80860,
    Mips,
    IBMSystem370,
    MIPSRS3000LittleEndian,
    Intel80960,
    PowerPC,
    PowerPC64bit,
    S390,
    IBMSpuSpc,
    NECV800,
    FujitsuFR20,
    TRWRH32,
    MotorolaRCE,
    ARMuptoARMv7Aarch32,
    DigitalAlpha,
    SuperH,
    SPARCVersion9,
    SiemensTriCoreEmbeddedProcessor,
    ArgonautRISCCore,
    HitachiH8_300,
    HitachiH8_300H,
    HitachiH8S,
    HitachiH8_500,
    IA64,
    StanfordMipsX,
    MotorolaColdFire,
    MotorolaM68HC12,
    FujitsuMMAMultimediaAccelerator,
    SiemensPCP,
    SonynCPUembeddedRISCProcessor,
    DensoNDR1microProcessor,
    MotorolaStarCoreProcessor,
    ToyotaME16processor,
    STMicroelectronicsST100Processor,
    AdvancedLogicCorpTinyJEmbeddedProcessorFamily,
    AMDx86_64,
    TMS320C6000Family,
    MCSTElbruse2k,
    ARM64bitsARMv8Aarch64,
    RiscV,
    BerkeleyPacketFilter,
    WDC65C816,
}
