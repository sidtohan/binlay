use goblin::elf::Elf;

/* Parses the given binary by using goblin. Only supports ELF format for the time being */
// Box -> heap allocated wrapper
// Dyn std::error::Error -> dynamically return any kind of error, as long as it implements Error
// trait.
pub fn parse_file(data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Similar to match
    if let Ok(elf) = Elf::parse(data) {
        println!("Parsed ELF!");
        println!("Entry point: {}", elf.entry);
        println!("Program Headers:");
        for sh in &elf.section_headers {
            println!("Offset: {}, : Size{}", sh.sh_offset, sh.sh_type);
        }
    } else {
        println!("Not a valid ELF file!");
    }

    Ok(())
}
