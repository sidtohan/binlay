mod file;
mod parse;
mod print;
mod arg_parse;

fn main() -> () {
    let buffer: Result<Vec<u8>, std::io::Error> = file::read_file(&arg_parse::CL_ARGS.file);
    
    // Error handling
    if buffer.is_err() {
        println!("Error: {}", buffer.unwrap_err());
        return;
    }
    
    // Print length of read 
    let data = buffer.unwrap();

    // Parse ELF file
    let _ = parse::parse_file(&data);
}
