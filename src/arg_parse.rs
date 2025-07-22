use clap::Parser;

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
    pub min_size: u32,
}

pub fn get_args() -> Args {
    let args = Args::parse();
    return args;
}

