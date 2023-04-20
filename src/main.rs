use clap::Parser;
use std::fs::remove_dir_all;


/// Simple cli to remove unwanted files from a git folder
#[derive(Parser)]
#[command(author="https://github.com/falkwitte", version)]
struct Opts{
    /// remove dir 
    #[arg(default_value="")]
    #[arg(long="remove")]
    #[arg(short='r')]
    dir:String,

}


fn main() {
    let args = Opts::parse();

    if args.dir != "" { 
        // remove custom user dir
        remove_dir_all(args.dir).unwrap_or_else(|e| println!("Error: {}", e));
    }
    
    //TODO: go through all directory entries and check if either .obsidian or .idea exists and remove them via remove_dir_all
    // else don't call remove_dir_all, so there isn't an error message when either dir does not exist in the first place

    // remove obsidian folder
    remove_dir_all("./.obsidian").unwrap_or_else(|e| println!("Error: {}", e));

    // remove idea folder
    remove_dir_all("./.idea").unwrap_or_else(|e| println!("Error: {}", e));
    
}
