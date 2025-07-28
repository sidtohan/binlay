use goblin::elf::Elf;
use goblin::elf64::sym::STT_FUNC;
use crate::print;
use crate::arg_parse::CL_ARGS;

/* This function prints the symbol information given the ELF file data */
fn collect_data<'a>(elf: &Elf<'a>) -> Vec<(String,u64)> {
    // Named lifetime:: since elf file is using borrowed data from buffer,
    // and it is present inside the Elf struct, we need to use the named lifetime to ensure
    // rust knows how long this data is valid
    // Sort symbols on basis of their size
    let mut symbols: Vec<_> = elf.syms
        .iter()
        .filter(|sym| sym.st_size > 0)
        .collect::<Vec<_>>();
    
    if !CL_ARGS.sort_ascending {
        symbols.sort_by(|a,b|  b.st_size.cmp(&a.st_size));
    } else {
        symbols.sort_by(|a,b|  a.st_size.cmp(&b.st_size));
    }

    
    // First collect data in form of <name,size> pairs, makes easier to perform some computation
    let mut final_symbols: Vec<(String,u64)> = Vec::new();

    for sym in symbols {
        // Filter out only functions.. for now
        if sym.st_type() != STT_FUNC { 
            continue;
        }

        // Use Command Line args to filter as per requirement
        if sym.st_size < CL_ARGS.min_size {
            continue;
        }
        
        // Symbol names are stored in symbol name table(denoted via strtab)
        // To access, use the offset we get from elf.syms, which is the symbol table 
        let symbol_name: Option<&'a str> = elf.strtab.get_at(sym.st_name);

        match symbol_name {
            Some(name) => final_symbols.push((name.to_string(),sym.st_size)),
            None => println!("Invalid Symbol Offset {}. Skipping...", sym.st_name),
        }
    }

    return final_symbols;
}

/* Parses the given binary by using goblin. Only supports ELF format for the time being */
// Box -> heap allocated wrapper
// Dyn std::error::Error -> dynamically return any kind of error, as long as it implements Error
// trait.
pub fn parse_file(data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Similar to match
    let parse_data = Elf::parse(data);

    match parse_data {
        Ok(elf) => print::print_table(collect_data(&elf)),
        Err(err) => println!("{}", err),
    }
    Ok(())
}
