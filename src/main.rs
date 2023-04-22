use clap::Parser;
use std::fs::{remove_dir_all, read_dir, remove_file};
use std::path::Path;
use colored::Colorize;


/// Simple cli to remove unwanted files from a git folder
/// | Removes '.obsidian' and '.idea' as standard
#[derive(Parser)]
#[command(author="https://github.com/falkwitte", version)]
struct Opts{
    /// remove custom path, works with ',' seperated list | e.g.: clear-git -r hello.txt,world.txt | 
    #[arg(default_value="")]
    #[arg(long="remove")]
    #[arg(short='r')]
    dir:String,

    /// Don't remove '.idea'
    #[arg(long="dont_remove_idea")]
    #[arg(short='i')]
    remove_idea:bool,

    /// Don't remove '.obsidian'
    #[arg(long="dont_remove_obsidian")]
    #[arg(short='o')]
    remove_obsidian:bool,
}


fn main() {
    let mut blacklist = vec![];
    let args = Opts::parse();


    // remove user dir entry
    if !args.dir.is_empty() { 
        let mut to_push_string: Vec<&str> = args.dir.split_terminator(',').collect();
        blacklist.append(&mut to_push_string);
    }

    if !args.remove_idea{
        blacklist.push(".idea");
    }

    if !args.remove_obsidian{
        blacklist.push(".obsidian");
    }

    println!("{}{:?}", "Remove: ".bold(), blacklist);

    if let Ok(entries) = read_dir(".") {
        for entry in entries.flatten(){
            
            let entry_string = entry.file_name()
                                    .into_string()
                                    .unwrap();  

            for i in blacklist.iter(){
                // remove dir
                if i == &entry_string.as_str() && Path::new(i).is_dir(){
                    remove_dir_all(&entry_string).unwrap_or_else(|_| println!("{}","Error: No such file or directory".bold().red()));
                    println!("{}{}", "Removed dir: ".bold().red(), i.bold().blue());

                // remove file
                }else if i == &entry_string && !Path::new(i).is_dir(){
                    remove_file(&entry_string).unwrap_or_else(|_| println!("{}","Error: No such file or directory".bold().red()));
                    println!("{}{}", "Removed file: ".bold().red(), i.bold());
                }
            }
        }
    }
    
}
