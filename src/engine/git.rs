use std::process::Command;
use crate::data::bfc::Bfc;
use crate::config::config::Config;
use crate::data::utils::create_directory;
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct ModifiedFile{
    pub path: String,
    pub content: Vec<String>,
}

impl ModifiedFile {
    pub fn from_lines_to_vec(paths: String, hash: &String, repository_path: &String) -> Result<Vec<ModifiedFile>, String>{
        let mut vec_m: Vec<ModifiedFile> = Vec::new();
        for s in paths.lines() {
            let content = match retrieve_file_content_from_commit_list(&hash.to_string(), &s.to_string(), &repository_path.to_string()) {
                Ok(e) => e,
                Err(e) => return Err(e.to_string())
            };
            let mod_file = ModifiedFile {
                path: s.to_string(),
                content
            };
            vec_m.push(mod_file);
        }
        Ok(vec_m)
    }

    pub fn from_retrive_file(hash: &String, file_path: &String, repository_path: &String) -> Result<ModifiedFile, String>{
        let content = match retrieve_file_content_from_commit_list(&hash.to_string(), &file_path.to_string(), &repository_path.to_string()){
            Ok(e) => e,
            Err(e) => return Err(e.to_string())
        };
        Ok(ModifiedFile {
            path: file_path.to_string(),
            content
        })
    }
}

pub fn get_date(hash: &String, repository_path: &String) -> Result<String, ()>{
    let command =
        format!("git show {} -s --date=iso --format=\"%cd\"", hash);
    let output = run_command(command, repository_path.to_string());
    Ok(output)
}


pub fn get_file_insertion_commit(hash: &String, file_path: &String, repository_path: &String) -> Result<String, ()>{
    let command = format!("git log --follow --diff-filter=A --find-renames=90% --pretty=%H {} -- {}", hash, file_path);
    let output = run_command(command, repository_path.to_string());

    Ok(output)
}

pub fn get_commits_touch_file(hash_one: &String, hash_two: &String, file_path: &String, repository_path: &String) -> Result<Vec<String>, ()>{
    let command = format!("git rev-list {}^..{} -- {}", hash_one, hash_two, file_path);
    let output = run_command(command, repository_path.to_string());

    let mut res: Vec<String> = output.split("\n")
        .filter(|i| !i.to_string().eq("")).map(|s| s.to_string()).collect();
    res.reverse();
    Ok(res)
}

pub fn get_modified_file(hash: &String, repository_path: &String) -> Result<Vec<ModifiedFile>, String>{
    let command_rev = format!("git rev-list HEAD | tail -n 1");
    let output_rev = run_command(command_rev, repository_path.to_string());
    let split = output_rev.split("\n");
    let first_oc: Vec<&str> = split.collect();
    let command;

    if hash.eq(first_oc[0]) {
        command = format!("git diff-tree --no-commit-id --name-only -r {} HEAD", hash);
    } else{
        command = format!("git log -m -1 --name-only --pretty=format: {}", hash);
    }


    let output = run_command(command, repository_path.to_string());

    let vec_m = match ModifiedFile::from_lines_to_vec(output, &hash, &repository_path){
        Ok(e) => e,
        Err(e) => return Err(e.to_string())
    };
    Ok(vec_m)

}

pub fn retrieve_file_content_from_commit(hash: &String, file_path: &String, repository_path: &String) -> Result<String, String>{
    let command = format!("git rev-parse HEAD");

    let last_commit = run_command(command.to_string(), repository_path.to_string());

    let command = format!("git checkout {}", hash);

    let _ = run_command(command.to_string(), repository_path.to_string());

    let data = match fs::read_to_string(format!("{}/{}", repository_path, file_path)){
        Ok(e ) => e,
        Err(e) => return Err(e.to_string())
    };

    let command = format!("git checkout {}", last_commit);

    let _ = run_command(command.to_string(), repository_path.to_string());

    Ok(data)
}

pub fn retrieve_file_content_from_commit_list(hash: &String, file_path: &String, repository_path: &String) -> Result<Vec<String>, String> {
    let command = format!("git rev-parse HEAD");
    let last_commit = run_command(command.to_string(), repository_path.to_string());

    let command = format!("git checkout {}", hash);

    let _ = run_command(command, repository_path.to_string());

    let data = match fs::read_to_string(format!("{}/{}", repository_path, file_path)){
        Ok(e) => e,
        Err(e) => return Err(e.to_string())
    };

    let mut file_in_commit :Vec<String> = Vec::new();
    for line in data.lines() {
        file_in_commit.push(line.to_string());
    }

    let command = format!("git checkout {}", last_commit);
    let _ = run_command(command, repository_path.to_string());

    Ok(file_in_commit)
}

// TODO NOT USED
pub fn clear_repository(repository_path: &String){
    let command = format!("git checkout origin/master -f");
    let output = run_command(command, repository_path.to_string());
    println!("{}", output);
}

pub fn clone(bfc: &Bfc, config: &Config) -> bool {
    let folder_is_created = create_directory(&config.path_projects);
    match folder_is_created {
        Ok(_) => {
            println!("Folder projects is OK")
        }
        Err(_) => {
            panic!("Error in folder projects creation")
        }
    }
    println!("Cloning project {}{}.git, wait...", config.git_cloud_base_url, &bfc.repo_name);
    let command = format!("git clone {}{}.git", config.git_cloud_base_url, &bfc.repo_name);
    run_command(command, config.path_projects.to_string());
    let name_project = bfc.clone().get_name_project().unwrap();
    Path::new(&format!("{}/{}", config.path_projects, name_project)).exists()
}



//TODO verificar se funciona no linux
pub fn run_command(command: String, repository_path: String) -> String{
    let output = if cfg!(target_os = "windows") {
        // Command::new("cmd")
        //         .args(&["/C", &command])
        //         .current_dir(repository_path)
        //         .output()
        //         .expect("failed to execute process")
        Command::new("C:\\Program Files\\Git\\bin\\sh.exe")
                .arg("-c")
                .arg(&command)
                .current_dir(repository_path)
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(repository_path)
                .output()
                .expect("failed to execute process")
    };
    let data = String::from_utf8_lossy(&output.stdout);
    String::from(data.to_string())
}

