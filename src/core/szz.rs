use std::collections::HashSet;
use petgraph::{Graph};
use petgraph::graph::{NodeIndex};
use crate::graph::{bfs, build_annotation_graph};
use crate::{Config, data, engine, git, Vertex};
use crate::data::bfc_result::Result;
use crate::utils::utils::recent_vertex;


pub struct Szz {}

impl Szz{
    pub fn init(){

        Szz::run();

    }

    fn run(){
        let input_file_path :String = String::from("bugfix_commits.json");

        let mut _bfcs = data::input::json::read_bfcs_from_json(&input_file_path).unwrap();
        let config = Config::new().unwrap();

        let mut result:Vec<Result> = Vec::new();

        let mut n_correct:usize = 0;

        let mut n_not_found:usize = 0;

        for bfc in _bfcs.clone() {
            println!("| -------- Cloning project = {} ---------- |", bfc.repo_name);
            match engine::git::clone(&bfc,&config){
                true => {
                    println!("Project clone succeeded")
                }
                false => {
                    println!("Error in clone project");
                    println!("Skiping...");
                    continue
                }
            };

            let repository_path = format!("{}/{}", config.path_projects,
                                          bfc.clone().get_name_project().unwrap().to_string());

            println!("Repository path = {}", repository_path);

            let modfiles = match git::get_modified_file(&bfc.fix_commit_hash.to_string(),
                                                  &repository_path.to_string()){
                Ok(e) => e,
                Err(e) => {
                    println!("Ignoring commit {} -> Error {}", bfc.fix_commit_hash,e.to_string());
                    continue
                }
            };

            let mut vertex_insertion_commit: HashSet<Vertex> = HashSet::new();

            for file_modified in modfiles {
                println!("File {}", file_modified.path);

                let graph: Graph<Vertex, ()>
                    = match build_annotation_graph(bfc.fix_commit_hash.to_string(),
                                                   &file_modified, &repository_path,
                                                   &config.work_directory.to_string()) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("Skiping file: {} -> Err {}", file_modified.path.to_string(), e.to_string());
                        continue
                    }
                };

                let mut fix_vertex: Vec<NodeIndex> = Vec::new();

                for vertex in graph.node_indices() {
                    if graph.node_weight(vertex).unwrap().hash.eq(&bfc.fix_commit_hash.to_string()) && graph.node_weight(vertex).unwrap().label.eq(&"CHANGE") {
                        fix_vertex.push(vertex);
                    }
                }


                for vertex in fix_vertex.clone() {
                    let res = match bfs(&graph, &vertex) {
                        Ok(e) => e,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            continue;
                        }
                    };
                    vertex_insertion_commit.insert(res);
                }
            }
            let r = recent_vertex(vertex_insertion_commit.clone(), &repository_path.to_string()).unwrap();
            result.push(Result::new(bfc.id, bfc.repo_name, bfc.fix_commit_hash.to_string(),  bfc.bug_commit_hash.get(0).unwrap().to_string(),r.to_string(), bfc.best_scenario_issue_date, bfc.language));

            if r.to_string() != "" && r.to_string() != " " {
                n_correct = if bfc.bug_commit_hash.get(0).unwrap().to_string() == r.to_string() { n_correct + 1 } else { n_correct };
            } else {
                n_not_found = n_not_found + 1;
            }
            println!("Res ->>>> {:?} - {:?}", bfc.bug_commit_hash.get(0).unwrap().clone(), r.clone());
        }

        for i in result.clone() {
            println!("{:?} - {:?} ", i.bug_commit_hash.to_string(), i.founded_bug_commit_hash.to_string());
        }

        println!("|--------------------------------------------|");
        println!("Saving result.json");
        data::output::json::save_result_json(result);
        println!("Done!");
        println!("|--------------------------------------------|");

        println!("n_corret = {}", n_correct.to_string());
        println!("Not found = {}", n_not_found);
        println!("whrongs = {}", _bfcs.len() - n_correct - n_not_found)

    }
}