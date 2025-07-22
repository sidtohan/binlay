/* This prints the table in a readable format */
pub fn print_table(symbols: Vec<(String,u64)>) -> () {
    // Compute total size
    // & -> destructuring to access fields
    // first param _ -> ignore it. no need to check for anything like borrow and stuff.
    let total_size: u64 = symbols.iter().map(|&(_, size) | size).sum();

    // Compute table width by using the max width of name
    let max_name_length: usize = symbols.iter().map(|(name,_)| name.len() as usize).max().unwrap_or(18);
    
    // Max bar length
    let max_bar_len = 25;
    
    // Table width: first column should have max(8, max_name_length) width
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

