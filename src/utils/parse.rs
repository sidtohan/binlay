use goblin::elf::Elf;
use goblin::elf64::sym::STT_FUNC;

/* This prints the table in a readable format */
fn print_table(symbols: Vec<(String,u64)>) -> () {
    // Compute total size
    // & -> destructuring to access fields
    let total_size: u64 = symbols.iter().map(|&(_, size) | size).sum();

    // Compute table width by using the max width of name
    let max_name_length: usize = symbols.iter().map(|(name,_)| name.len() as usize).max().unwrap_or(18);
    
    // Max bar length
    let max_bar_len = 25;
    
    // Table width: first column should have max(8, max_name_length) width
    println!("Max name length: {}", max_name_length);
    println!("+{:-<width$}+{:-<24}+{:-<12}+{:-<27}+", "", "", "", "", width = max_name_length + 2);
    println!("| {:<width$} | {:>6} | {:>10} | {:<25} |", "Symbol Name", "Symbol Size (in bytes)", "% of Total", "Visual Size Bar", width = max_name_length);
    println!("+{:-<width$}+{:-<24}+{:-<12}+{:-<27}+", "", "", "", "", width = max_name_length + 2);
    for (name,size) in symbols {
        let percentage = (size as f64 / total_size as f64) * 100.0;
        let bar_len = ((percentage / 100.0) * max_bar_len as f64).round() as usize;
        let bar = "█".repeat(bar_len);
        println!("| {:<width$} | {:>22} | {:>9.1}% | {:<25} |", name, size, percentage, bar, width = max_name_length);
    }
    println!("+{:-<width$}+{:-<24}+{:-<12}+{:-<27}+", "", "", "", "", width = max_name_length + 2);
}

/* This function prints the symbol information given the ELF file data */
fn print_data<'a>(elf: &Elf<'a>) -> () {
    // Named lifetime:: since elf file is using borrowed data from buffer,
    // and it is present inside the Elf struct, we need to use the named lifetime to ensure
    // rust knows how long this data is valid
    // Sort symbols on basis of their size
    let mut symbols: Vec<_> = elf.syms
        .iter()
        .filter(|sym| sym.st_size > 0)
        .collect::<Vec<_>>();
        
    symbols.sort_by(|a,b| b.st_size.cmp(&a.st_size));
    
    // First collect data in form of <name,size> pairs, makes easier to perform some computation
    let mut final_symbols: Vec<(String,u64)> = Vec::new();

    for sym in symbols {
        // Filter out only functions.. for now
        if sym.st_type() != STT_FUNC { 
            continue;
        }
        
        // Symbol names are stored in symbol name table(denoted via strtab)
        // To access, use the offset we get from elf.syms, which is the symbol table 
        let symbol_name: Option<&'a str> = elf.strtab.get_at(sym.st_name);

        match symbol_name {
            Some(name) => final_symbols.push((name.to_string(),sym.st_size)),
            None => {
                println!("Invalid Symbol!")
                // TODO: throw some error here.
            },
        }
    }
    print_table(final_symbols);
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
