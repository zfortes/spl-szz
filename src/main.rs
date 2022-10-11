use crate::config::config::Config;
use crate::utils::vertex::Vertex;
use crate::core::{graph, szz};
use crate::engine::git;
use crate::szz::Szz;

#[allow(dead_code)]
mod data;
mod engine;
mod config;
mod utils;
mod core;

fn main() {
    println!("Hello, this is the SPL-SZZ");

    Szz::init();

}
