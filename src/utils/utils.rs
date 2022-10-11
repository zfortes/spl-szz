use std::collections::{HashSet};
use crate::engine::git;
use crate::data;
use std::io::{ Write};
use crate::utils::hunk::Hunk;
use std::string::String;
use regex::Regex;
use chrono::{NaiveDateTime};
use crate::utils::vertex::Vertex;

//TODO refatorar essa função
pub fn get_files_diff(hash_a: &String, hash_b: &String, file_path: &String, repository_path: &String, work_directory: &String) -> Result<(String, String), String>{
    let file_left = match git::retrieve_file_content_from_commit(&hash_a, &file_path, &repository_path) {
        Ok(e)  => e,
        Err(e) => return Err(e.to_string())
    };
    let file_right= match git::retrieve_file_content_from_commit(&hash_b, &file_path, &repository_path) {
        Ok(e)  => e,
        Err(e) => return Err(e.to_string())
    };

    let name_file = file_path.split("/").last().unwrap().to_string();

    let mut file = data::utils::create_file(&name_file.to_string(), &format!("{}.txt", hash_a), &work_directory.to_string()).unwrap(); // File::create("tempFile.txt").unwrap();
    file.write_all(file_left.as_bytes());

    file = data::utils::create_file(&name_file.to_string(), &format!("{}.txt", hash_b), &work_directory.to_string()).unwrap(); // File::create("tempFile.txt").unwrap();
    file.write_all(file_right.as_bytes());

    let file_a = format!("{}{}_{}{}", work_directory, name_file, hash_a, ".txt");

    let file_b = format!("{}{}_{}{}", work_directory, name_file, hash_b, ".txt");

    Ok((file_a.to_string() ,file_b.to_string()))

}

//TODO refatorar essa funcao
pub fn get_hunks(hash_a: &String, hash_b: &String, file_path: &String, repository_path: &String, work_directory: &String) -> Result<Vec<Hunk>, String> {
    let files = match get_files_diff(&hash_a, &hash_b, &file_path, &repository_path, &work_directory) {
        Ok(e)  => e,
        Err(e) => return Err(e.to_string())
    };
    let command = format!("diff -a -d {} {}", files.0, files.1);

    let output = git::run_command(command.to_string(), ".".to_string());
    let split = output.split("\n");
    let mut hunks :Vec<Hunk> = Vec::new();
    for line in split {
        let hunk :Hunk = get_hunk_parsed(&line.to_string()).unwrap();
        if hunk.left_from != -1 && hunk.left_to != -1 && hunk.right_from != -1 && hunk.right_to != -1 {
            hunks.push(hunk);
        }
    }

    Ok(hunks)
}

// for change
// for additions
// for deleletions
fn get_hunk_parsed(line :&String) -> Result<Hunk, String>{
    let change = Regex::new(r"(^)(\d+)(,\d+)?(c)(\d+)(,\d+)?").unwrap();
    let additions = Regex::new(r"(^)(\d+)(,\d+)?(a)(\d+)(,\d+)?").unwrap();
    let deletions = Regex::new(r"(^)(\d+)(,\d+)?(d)(\d+)(,\d+)?").unwrap();

    let line_change = change.is_match(line);
    let line_additions = additions.is_match(line);
    let line_deletions = deletions.is_match(line);
    let hunk :Hunk;

    if line_change {
        hunk = generate_hunk(line.to_string(), "c".to_string(), 1).unwrap();
    }else if line_additions {
        hunk = generate_hunk(line.to_string(), "a".to_string(), 2).unwrap();
    }else if line_deletions {
        hunk = generate_hunk(line.to_string(), "d".to_string(), 3).unwrap();
    } else {
        hunk = Hunk::initialized();
    }

    Ok(hunk)

}

fn generate_hunk(line :String, token_letter :String, kind :i64) -> Result<Hunk, ()> {

    let letter_token = format!("{}", token_letter);

    let tokens :Vec<String> = line.replace(" ", "").split(&letter_token.to_string()).map(|s| s.to_string()).collect();

    let token_left :Vec<String> = tokens[0].split(",").map(|s| s.to_string()).collect();
    let token_right :Vec<String> = tokens[1].split(",").map(|s| s.to_string()).collect();


    let mut left_line_from :i64 = -1;
    let mut left_line_to :i64 = -1;
    let mut right_line_from :i64 = -1;
    let mut right_line_to :i64 = -1;


    if token_letter.eq("c"){
        if token_left.len() == 2 {
            left_line_from = token_left.get(0).unwrap().to_string().parse().unwrap();
            left_line_to = token_left.get(1).unwrap().to_string().parse().unwrap();
        }else{
            left_line_from = tokens.get(0).unwrap().to_string().parse().unwrap();
            left_line_to = left_line_from;
        }

        if token_right.len() == 2 {
            right_line_from = token_right.get(0).unwrap().to_string().parse().unwrap();
            right_line_to = token_right.get(1).unwrap().to_string().parse().unwrap();
        }else{
            right_line_from = tokens.get(1).unwrap().to_string().parse().unwrap();
            right_line_to = right_line_from;
        }
    }else { if token_letter.eq("a"){
        if token_left.len() == 2 {
            left_line_from = token_left.get(0).unwrap().to_string().parse().unwrap();
            left_line_to = token_left.get(1).unwrap().to_string().parse().unwrap();
        }else{
            left_line_from = tokens.get(0).unwrap().to_string().parse().unwrap();
            left_line_to = tokens.get(0).unwrap().to_string().parse().unwrap(); // TODO ver isso
        }

        if token_right.len() == 2 {
            right_line_from = token_right.get(0).unwrap().to_string().parse().unwrap();
            right_line_to = token_right.get(1).unwrap().to_string().parse().unwrap();
        }else{
            right_line_from = tokens.get(1).unwrap().to_string().parse().unwrap();
            right_line_to = tokens.get(1).unwrap().to_string().parse().unwrap();
        }
    } else { if token_letter.eq("d"){
        if token_left.len() == 2 {
            left_line_from = token_left.get(0).unwrap().to_string().parse().unwrap();
            left_line_to = token_left.get(1).unwrap().to_string().parse().unwrap();
        }else{
            left_line_from = tokens.get(0).unwrap().to_string().parse().unwrap();
            left_line_to =  tokens.get(0).unwrap().to_string().parse().unwrap();
        }

        if token_right.len() == 2 {
            right_line_from = token_right.get(0).unwrap().to_string().parse().unwrap();
            right_line_to = token_right.get(1).unwrap().to_string().parse().unwrap();
        }else{
            right_line_from = tokens.get(1).unwrap().to_string().parse().unwrap();
            right_line_to = tokens.get(1).unwrap().to_string().parse().unwrap();
        }
    }}}


    let hunk = Hunk{
        left_from: left_line_from,
        left_to: left_line_to,
        right_from: right_line_from,
        right_to: right_line_to,
        kind: kind
    };

    Ok(hunk)
}

fn max_float(val_a :f64, val_b :f64) -> f64 {
    if val_a >= val_b { val_a } else { val_b }
}

pub fn large_modification(hunk :&Hunk, size_file_a :i64, size_file_b :i64) -> Result<bool, ()> {
    let alpha :f64 = 0.1;
    let beta :f64 = 4.0;

    let mut size_hunk_left :i64 = 1;
    if hunk.left_to - hunk.left_from != 0 { size_hunk_left = hunk.left_to - hunk.left_from };
    let size_hunk_right :i64 = 1;
    if hunk.right_to - hunk.right_from != 0 { size_hunk_left = hunk.right_to - hunk.right_from };

    let sizehunkleft = size_hunk_left as f64;
    let sizehunkright = size_hunk_right as f64;
    let sizefilea = size_file_a as f64;
    let sizefileb = size_file_b as f64;
    let muta = alpha * sizefilea;
    if sizehunkleft > max_float(muta, beta) || sizehunkright > max_float(alpha*sizefileb, beta) {
        return Ok(true);
    }

    if (sizehunkleft / sizehunkright) < (1 as f64/beta) || beta < (sizehunkleft / sizehunkright) as f64 {
        return Ok(true);
    }

    Ok(false)
}

// TODO rever e refatorar isso

pub fn recent_vertex(vertexs :HashSet<Vertex>, repository_path: &String) -> Result<String, ()> {

    let initial_date = NaiveDateTime::parse_from_str(
        &"1981-09-05 00:00:00 +0100".to_string(), "%Y-%m-%d %H:%M:%S %z").unwrap();
    let mut aux_date: NaiveDateTime = initial_date.clone();
    let mut aux_vertex= Vertex::new();
    for v in vertexs {
        let data = git::get_date(&v.hash, &repository_path).unwrap().to_string();
        let date_time = NaiveDateTime::parse_from_str(
            &data.replace(" ", "").replace("\n", "").to_string(),
            "%Y-%m-%d %H:%M:%S %z").unwrap();
        if aux_date.eq(&initial_date){
            aux_date = date_time;
            aux_vertex = v;
        } else if date_time > aux_date {
            aux_date = date_time;
            aux_vertex = v;
        }
    }

    Ok(aux_vertex.hash.to_string())

}
