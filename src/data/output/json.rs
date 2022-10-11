use crate::data::bfc_result::Result;
use std::fs::File;
use std::io::prelude::*;

pub fn save_result_json(result: Vec<Result>) -> std::result::Result<(), ()> {
    let r = match serde_json::to_string(&result){
        Ok(e) => e,
        Err(_) => return Err(())
    };

    let mut file = File::create("result.json").unwrap();
    file.write_all(r.as_bytes()).unwrap();
    Ok(())
}