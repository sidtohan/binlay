use clap::Parser;
use once_cell::sync::Lazy;

// Command line argument parsing
#[derive(Parser, Debug)]
#[command(name = "binlay")]
#[command(about = "Parse ELF files and analyze symbol sizes", long_about = None)]
pub struct Args {
    /// PATH TO ELF FILE
    #[arg(short,long)] // enables -f or --file
    pub file: String,
    #[arg(short,long)]
    pub sort_ascending: bool,
    #[arg(short,long,default_value="1")]
    pub min_size: u64,
}

// Using lazy is needed if we want static, since function call is static is now allowed globally
// This also means this is initialized when first accessed
pub static CL_ARGS: Lazy<Args> = Lazy::new(|| Args::parse());
