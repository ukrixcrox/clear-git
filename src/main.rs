use clap::Parser;
use colored::Colorize;
use dirs::config_dir;
use std::fs::{read_dir, read_to_string, remove_dir_all, remove_file, write};
use std::path::Path;
use std::process::exit;
use utils::check_config_file_existence;

mod utils;

/// Simple cli to remove unwanted files from a git folder
/// | Removes '.idea' and '.vscode' as standard
#[derive(Parser)]
#[command(author = "https://github.com/falkwitte", version)]
struct Opts {
    /// remove custom path, works with ',' seperated list e.g.: clear-git -r hello.txt,world.txt
    #[arg(default_value = "")]
    #[arg(long = "remove")]
    #[arg(short = 'r')]
    dir: String,
}

fn main() {
    let mut blacklist = vec![];
    let args = Opts::parse();
    let config_dir_str = config_dir().unwrap().to_str().unwrap().to_owned();

    if !check_config_file_existence(&config_dir_str) {
        create_config_file(&config_dir_str);
        println!(
            "{}\n{}",
            "Config file path:".bold().blue(),
            config_dir_str.to_owned() + "/clear_git_config.txt"
        );
        println!("\nFill the config file up with files you want to remove when running 'clear-git',\nthere are some examples in the file");
        exit(0x0100);
    }

    // remove user dir entry
    if !args.dir.is_empty() {
        let mut to_push_string: Vec<&str> = args.dir.split_terminator(',').collect();
        blacklist.append(&mut to_push_string);
    }

    // read config file
    let config_file_content_string =
        read_to_string(config_dir_str + "/clear_git_config.txt").unwrap();
    let mut config_file_content: Vec<&str> =
        config_file_content_string.split_terminator("\n").collect();
    blacklist.append(&mut config_file_content);

    println!("{}{:?}", "Remove: ".bold(), blacklist);

    if let Ok(entries) = read_dir(".") {
        for entry in entries.flatten() {
            let entry_string = entry.file_name().into_string().unwrap();

            for i in blacklist.iter() {
                // remove dir
                if i == &entry_string.as_str() && Path::new(i).is_dir() {
                    remove_dir_all(&entry_string).unwrap_or_else(|_| {
                        println!("{}", "Error: No such file or directory".bold().red())
                    });
                    println!("{}{}", "Removed dir: ".bold().red(), i.bold().blue());

                // remove file
                } else if i == &entry_string && !Path::new(i).is_dir() {
                    remove_file(&entry_string).unwrap_or_else(|_| {
                        println!("{}", "Error: No such file or directory".bold().red())
                    });
                    println!("{}{}", "Removed file: ".bold().red(), i.bold());
                }
            }
        }
    }
}

fn create_config_file(config_dir_str: &String) {
    let config_file_path = config_dir_str.to_owned() + "/clear_git_config.txt";

    write(config_file_path, ".idea\n.vscode").unwrap();
}
