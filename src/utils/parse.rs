use goblin::elf::Elf;
use goblin::elf64::sym::STT_FUNC;

// Named lifetime:: since elf file is using borrowed from buffer,
// and it is present inside the Elf struct, we need to use the named lifetime to ensure
// rust knows how long this data is valid
fn print_data<'a>(elf: &Elf<'a>) -> () {
    println!("Parsed ELF!");
    println!("Entry point: {}", elf.entry);
    for sym in &elf.syms {
        // Filter out only functions.. for now
        if sym.st_type() != STT_FUNC {
            continue;
        }
        // Symbol names are stored in symbol name table(denoted via strtab)
        // To access, use the offset we get from elf.syms, which is the symbol table 
        let name: Option<&'a str> = elf.strtab.get_at(sym.st_name);
        match name {
            Some(symbol_name) => println!("Name: {}, Size: {}", symbol_name, sym.st_value),
            None => println!("Invalid Symbol!"),
        }
    }
}

/* Parses the given binary by using goblin. Only supports ELF format for the time being */
// Box -> heap allocated wrapper
// Dyn std::error::Error -> dynamically return any kind of error, as long as it implements Error
// trait.
pub fn parse_file(data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Similar to match
    let parse_data = Elf::parse(data);
    match parse_data {
        Ok(elf) => print_data(&elf),
        Err(err) => println!("{}", err),
    }
    Ok(())
}
