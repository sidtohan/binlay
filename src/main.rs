use clap::Parser;
mod utils;

// Command line argument parsing
#[derive(Parser, Debug)]
#[command(name = "binlay")]
#[command(about = "Parse ELF files and analyze symbol sizes", long_about = None)]
struct Args {
    /// PATH TO ELF FILE
    #[arg(short,long)] // enables -f or --file
    file: String,
}

fn main() -> () {
    let args = Args::parse();
    
    // Arg pass to open file
    let buffer: Result<Vec<u8>, std::io::Error> = utils::file::read_file(&args.file);
    
    // Error handling
    if buffer.is_err() {
        println!("Error: {}", buffer.unwrap_err());
        return;
    }
    
    // Print length of read 
    let data = buffer.unwrap();
    println!("Read {} bytes from the file {}",data.len(), &args.file);

    // Parse ELF file
    let _ = utils::parse::parse_file(&data);
}
