use std::fs::File;
use std::io::Read;

pub fn read_file(fname: &String) -> Result<Vec<u8>, std::io::Error>  {
    // Open file
    let mut file: File = File::open(fname)?;
    
    // Create buffer
    let mut buffer: Vec<u8> = Vec::new();

    // Read file entirely to buffer
    file.read_to_end(&mut buffer)?;
    
    return Ok(buffer);
}
