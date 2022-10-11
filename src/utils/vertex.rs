use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct Vertex {
    pub hash :String,
    pub line :i64,
    pub label :String
}

impl Vertex {

    pub fn new() -> Vertex{
        Vertex {
            hash: " ".to_string(),
            line: -1,
            label: " ".to_string()
        }
    }

    pub fn from_hash_and_line(hash :String, line :i64) -> Vertex{
        Vertex {
            hash: hash.to_string(),
            line: line,
            label: "".to_string()
        }
    }

    pub fn eq(&self, other: Vertex) -> bool {
        if other.hash == self.hash && other.line == self.line && other.label == self.label { return true; }
        return false
    }

}