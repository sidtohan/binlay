use std::env;
mod utils;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    // Arg format -> filename -- <user_args>
    if args.len() != 2 {
        println!("Usage -> {} <ELF_FILE_PATH>", args[0]);
        return;
    }

    // Arg pass to open file
    let buffer: Result<Vec<u8>, std::io::Error> = utils::file::read_file(&args[1]);
    
    // Error handling
    if buffer.is_err() {
        println!("Error: {}", buffer.unwrap_err());
        return;
    }
    
    // Print length of read 
    let data = buffer.unwrap();
    println!("Read {} bytes from the file {}",data.len(), &args[1]);

    // Parse ELF file
    let _ = utils::parse::parse_file(&data);
}
