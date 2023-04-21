use clap::Parser;
use std::fs::{remove_dir_all, read_dir};


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

    if !args.dir.is_empty() { 
        // remove custom user dir
        remove_dir_all(args.dir).unwrap_or_else(|_| println!("Error: No such directory"));
    }
    
    //TODO: go through all directory entries and check if either .obsidian or .idea exists and remove them via remove_dir_all
    // else don't call remove_dir_all, so there isn't an error message when either dir does not exist in the first place

    if let Ok(entries) = read_dir(".") {
        for entry in entries.flatten(){
            
            let entry_string = entry.file_name()
                                    .into_string()
                                    .unwrap();  

            if entry_string == ".obsidian"{
                // remove obsidian folder
                remove_dir_all("./.obsidian").unwrap_or_else(|_| println!("Error: No such file or directory"));
            }else if entry_string == ".idea"{
                // remove idea folder
                remove_dir_all("./.idea").unwrap_or_else(|_| println!("Error: No such file or directory"));
            }
        }
    }
    
}
