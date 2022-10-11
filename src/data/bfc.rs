use std::collections::LinkedList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bfc {
    pub id: u64,
    pub repo_name: String,
    pub fix_commit_hash: String,
    pub bug_commit_hash: Vec<String>,
    pub best_scenario_issue_date: String,
    pub language: LinkedList<String>,
}

impl Bfc {
    pub fn get_name_project(self) -> Result<String, ()>{
        Ok(String::from(self.repo_name.split("/").last().unwrap().to_string()))
    }
}