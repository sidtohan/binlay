use goblin::elf::Elf;

// Named lifetime:: since elf file is using borrowed from buffer,
// and it is present inside the Elf struct, we need to use the named lifetime to ensure
// rust knows how long this data is valid
fn print_data<'a>(elf: &Elf<'a>) -> () {
    println!("Parsed ELF!");
    println!("Entry point: {}", elf.entry);
    println!("Program Headers:");
    for sh in &elf.section_headers {
        // Sh_name is the offset, need to refer to the section header string table to get the name
        let name: Option<&'a str> = elf.shdr_strtab.get_at(sh.sh_name);
        match name {
            Some(section_name) => println!("Offset: {}, Name: {}, Size: {}", sh.sh_offset, section_name, sh.sh_size),
            None => println!("Invalid name offset"),
        }
    }
}

/* Parses the given binary by using goblin. Only supports ELF format for the time being */
// Box -> heap allocated wrapper
// Dyn std::error::Error -> dynamically return any kind of error, as long as it implements Error
// trait.
pub fn parse_file(data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Similar to match
    if let Ok(elf) = Elf::parse(data) {
        print_data(&elf);
    } else {
        println!("Not a valid ELF file!");
    }

    Ok(())
}
