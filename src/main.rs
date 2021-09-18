mod elf;
mod file_header;
mod program_header;
mod section_header;
mod utils;

use std::fs;

use elf::Elf;

fn main() {
    let data = fs::read("test").unwrap();
    // println!("{:x?}", data);
    let e = Elf::from_data(&data);
    let arch = e.file_header.ei_class;
    let machine = e.file_header.e_machine;
    println!("Arch: {}", arch);
    println!("Machine: {:?}", machine);
}
