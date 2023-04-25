use std::fs::read_dir;

pub fn check_config_file_existence(config_dir: &String) -> bool {
    let mut file_exist = true;

    if let Ok(entries) = read_dir(config_dir) {
        for entry in entries.flatten() {
            let entry_string = entry.file_name().into_string().unwrap();

            if entry_string == "clear_git_config.txt" {
                file_exist = true;
                break;
            } else {
                file_exist = false;
            }
        }
    }

    file_exist
}
