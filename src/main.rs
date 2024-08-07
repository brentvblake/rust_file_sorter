use std::fs;

fn read_all_files_in_downloads(directory: String) -> Vec<String> {
    let mut files = Vec::new();
    let paths = match fs::read_dir(&directory) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error reading directory {}: {}", directory, e);
            return files;
        }
    };
    for path in paths {
        match path {
            Ok(entry) => {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            }
            Err(e) => println!("Error reading path: {}", e),
        }
    }
    files
}

fn get_file_extension(file: &str) -> String {
    let parts: Vec<&str> = file.split('.').collect();
    if parts.len() > 1 {
        parts[parts.len() - 1].to_string()
    } else {
        "".to_string()
    }
}

fn check_if_folder_exists(folder: &str) -> bool {
    let path = std::path::Path::new(folder);
    path.exists()
}

fn check_if_file(file: &str) -> bool {
    let path = std::path::Path::new(file);
    path.is_file()
}

fn move_file_to_folder(file: &str, folder: &str) {
    let file_name = file.split('/').collect::<Vec<&str>>();
    let file_name = file_name[file_name.len() - 1];
    let new_file_path = format!("{}/{}", folder, file_name);
    if let Err(e) = fs::rename(file, &new_file_path) {
        println!("Error moving file {} to {}: {}", file, new_file_path, e);
    }
}

fn main() {
    let current_directory = match std::env::current_dir() {
        Ok(dir) => dir.to_str().unwrap().to_string(),
        Err(e) => {
            println!("Error getting current directory: {}", e);
            return;
        }
    };
    
    println!(
        "Are you very sure you want to sort files in the current directory \"{}\"?  \nThis can not be undone. (y/n)",
        current_directory
    );
    
    let mut response = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut response) {
        println!("Error reading input: {}", e);
        return;
    }

    if response.trim() != "y" {
        println!("Exiting program");
        return;
    }

    let files = read_all_files_in_downloads(current_directory.clone());
    for file in files {
        if !check_if_file(&file) {
            continue;
        }
        let file_extension = get_file_extension(&file);

        if file_extension.is_empty() {
            println!("File {} does not have an extension, skipping.", file);
            continue;
        }

        let folder = format!("{}/{}", current_directory, file_extension);

        if !check_if_folder_exists(&folder) {
            if let Err(e) = fs::create_dir(&folder) {
                println!("Error creating directory {}: {}", folder, e);
                continue;
            }
        }
        move_file_to_folder(&file, &folder);
    }
    println!("Files sorted successfully");
}
