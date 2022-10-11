use std::collections::LinkedList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub id: u64,
    pub repo_name: String,
    pub fix_commit_hash: String,
    pub bug_commit_hash: String,
    pub founded_bug_commit_hash: String,
    pub best_scenario_issue_date: String,
    pub language: LinkedList<String>,
    pub correct: bool,
}

impl Result {

    pub fn new(id: u64, repo_name :String, fix_commit_hash: String, bug_commit_hash: String, founded_bug_commit_hash: String, best_scenario_issue_date: String, language: LinkedList<String>) -> Self {
        Result {
            id,
            repo_name,
            fix_commit_hash: fix_commit_hash.to_string(),
            bug_commit_hash: bug_commit_hash.to_string(),
            founded_bug_commit_hash: founded_bug_commit_hash.to_string(),
            best_scenario_issue_date,
            language,
            correct: bug_commit_hash == founded_bug_commit_hash
        }
    }
}