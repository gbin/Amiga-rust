use std::env;
use elf::File;
use num::FromPrimitive;
#[macro_use]
extern crate num_derive;


#[derive(FromPrimitive, Debug)]
#[repr(u8)]
enum sym_type {
    STB_LOCAL=0,
    STB_GLOBAL=1,
    STB_WEAK=2,
    STB_LOOS =10,
    STB_HIOS = 12,
    STB_LOPROC = 13,
    STB_HIPROC = 15,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = std::path::PathBuf::from(&args[1]);

    let data= std::fs::read(path).expect("Could not read file {path}.");
    let file_data = data.as_slice();

    let mut file = File::open_stream(file_data).expect("Could not parse ELF Header");

    let (symtab, strtab) = file
        .symbol_table()
        .expect("Failed to read symbol table")
        .expect("File contained no symbol table");
    for symbol in symtab {
        let symbol_name = strtab
            .get(symbol.st_name as usize)
            .expect("Failed to get name from strtab");
        let symbol_type = symbol.st_symtype();
        let t:sym_type = FromPrimitive::from_u8(symbol_type).expect(format!("Invalid Symbol type '{symbol_type}'").as_str());
        println!("{symbol_name}: {t:?}");


        }
}