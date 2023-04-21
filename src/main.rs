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
    let blacklist = vec![".obsidian", ".idea"];
    let args = Opts::parse();

    if !args.dir.is_empty() { 
        // remove custom user dir
        remove_dir_all(args.dir).unwrap_or_else(|_| println!("Error: No such directory"));
    }

    

    if let Ok(entries) = read_dir(".") {
        for entry in entries.flatten(){
            
            let entry_string = entry.file_name()
                                    .into_string()
                                    .unwrap();  


            for i in blacklist.iter(){
                if i == &entry_string.as_str(){
                    // remove somthing
                    remove_dir_all(&entry_string).unwrap_or_else(|_| println!("Error: No such file or directory"));
                }
            }

            

        }
    }
    
}
