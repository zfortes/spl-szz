use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hunk {
    pub left_from: i64,
    pub left_to: i64,
    pub right_from: i64,
    pub right_to: i64,
    pub kind: i64
}

impl Hunk {
    pub fn new(left_from: i64, left_to :i64, right_from: i64, right_to: i64, kind: i64) -> Self {
        Hunk {
            left_from, left_to, right_from, right_to, kind
        }
    }

    pub fn initialized() -> Self {
        Hunk {
            left_from: -1, left_to: -1, right_from: -1, right_to: -1, kind: -1
        }
    }

    fn to_string(&self) -> String {
        format!("Hunk: leftFrom={}, leftTo={}, rightFrom={}, rightTo={}, kind={}", self.left_from,
                self.left_to, self.right_from, self.right_to, self.kind)
    }

    pub fn is_change(&self) -> bool {
        self.kind == 1
    }

    pub fn is_addition(&self) -> bool {
        self.kind == 2
    }

    pub fn is_deletion(&self) -> bool {
        self.kind == 3
    }
}