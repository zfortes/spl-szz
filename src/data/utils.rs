use std::fs;
use std::fs::File;

pub fn create_directory(path: &String) -> std::io::Result<()> {
    let is_created = fs::create_dir_all(path.to_string());
    match is_created {
        Err(err) => println!("{}", err),
        _ => {}
    };
    Ok(())
}

pub fn create_file(name: &String, format: &String, path: &String) -> std::io::Result<File>{
    let _ = create_directory(&path);
    let file = File::create(format!("{}{}{}{}{}", path, "/", name, "_", format))?;
    Ok(file)
}