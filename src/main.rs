use std::env;
mod elf;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    // Arg format -> filename -- user args
    if args.len() < 2 {
        println!("Usage -> {} <ELF_FILE_PATH>", args[0]);
        return;
    }

    // Arg pass to open file
    let buffer: Result<Vec<u8>, std::io::Error> = elf::file::read_file(&args[1]);
    
    if buffer.is_err() {
        println!("Error: {}", buffer.unwrap_err());
        return;
    }

    // Print length of read 
    println!("Read {} bytes",buffer.unwrap().len());
}
