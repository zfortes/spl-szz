use std::collections::HashMap;
use crate::engine::git;
use crate::{ Vertex };
use crate::utils::hunk::Hunk;
use crate::utils;
use petgraph::{ Graph, Incoming};
use petgraph::graph::{Node, NodeIndex};
use petgraph::visit::{Bfs, EdgeRef};
use crate::git::ModifiedFile;

pub fn build_annotation_graph(hash :String, file_modified :&ModifiedFile, repository_path :&String, work_directory :&String) -> Result<Graph<Vertex, ()>, String>{
    let insertion_file_commit = git::get_file_insertion_commit(&hash, &file_modified.path.to_string(), &repository_path.to_string()).unwrap();

    let commits_touch_file :Vec<String> = git::get_commits_touch_file(&insertion_file_commit.replace("\n", "").to_string(), &hash, &file_modified.path.to_string(), &repository_path.to_string()).unwrap();

    let mut graph = Graph::new();

    let mut file_size_at_commit = HashMap::new();

    for i in 0..commits_touch_file.len(){
        let file_lines :Vec<String>;

        if !hash.eq(commits_touch_file.get(i).unwrap()){
           file_lines = match git::retrieve_file_content_from_commit_list(&commits_touch_file.get(i).unwrap(), &file_modified.path, &repository_path){
               Ok(e) => e,
               Err(e) => {
                   println!("File note found: {}", e.to_string());
                   continue;
               }
           };
        }else {
            file_lines = file_modified.content.clone();
        }
        file_size_at_commit.insert(commits_touch_file.get(i).unwrap(), file_lines.len());

        for j in 1..=file_lines.len() {
            let vertex :Vertex = Vertex::from_hash_and_line(commits_touch_file.get(i).unwrap().to_string(), j as i64);
            let _ = graph.add_node(vertex);
        }
    }


    for i in 1..commits_touch_file.len() {
        let commit_left :String = commits_touch_file.get(i-1).unwrap().clone();
        let commit_right :String = commits_touch_file.get(i).unwrap().clone();

        let hunks :Vec<Hunk> = match utils::utils::get_hunks(
            &commit_left, &commit_right, &file_modified.path, &repository_path, &work_directory){
            Ok(e) => e,
            Err(e) => {
                println!("File not read: {}", e.to_string());
                continue
            }
        };

        let mut pos_l = 1;
        let mut pos_r = 1;

        for j in 0..hunks.len() {
            let hunk = hunks.get(j).unwrap();

            while pos_l <= hunk.left_from && pos_r <= hunk.right_from {
                let final_pos_l = pos_l;
                let final_pos_r = pos_r;

                let vertex_left = graph.filter_map(
                    |i, v|
                        if v.hash.eq(&commit_left)
                            && v.line == final_pos_l { Some(i) }
                        else { None },
                    |_, weight| Some(*weight),
                ).raw_nodes().get(0).unwrap().weight;

                let vertex_right  = graph.filter_map(
                    |i, v| if v.hash.eq(&commit_right) && v.line == final_pos_r { Some(i) } else { None },
                    |_, weight| Some(*weight),
                ).raw_nodes().get(0).unwrap().weight;

                graph.add_edge(vertex_left, vertex_right, ());

                pos_r = pos_r + 1;
                pos_l = pos_l + 1;
            }

            let is_larget_modification :bool =
                utils::utils::large_modification(
                    &hunk, file_size_at_commit.get(&commit_left).unwrap().clone() as i64,
                    file_size_at_commit.get(&commit_right).unwrap().clone() as i64).unwrap();

            if hunk.is_change(){
                if !is_larget_modification {
                    for l in hunk.left_from..=hunk.left_to {
                        for r in hunk.right_from..=hunk.right_to {
                            let final_l = l; // TODO precisa disso?
                            let final_r = r;
                            let vertex_left = graph.filter_map(
                                |i, v| if v.hash.eq(&commit_left) && v.line == final_l { Some(i) } else { None },
                                |i, weight| Some(*weight),
                            ).raw_nodes().get(0).unwrap().weight;

                            let vertex_right  = graph.filter_map(
                                |i, v| if v.hash.eq(&commit_right) && v.line == final_r { Some(i) } else { None },
                                |i, weight| Some(*weight),
                            ).raw_nodes().get(0).unwrap().weight;
                            graph.add_edge(vertex_left, vertex_right, ());
                        }
                    }
                }
            }

            //Check for larger modifications
            if !is_larget_modification {
                if hunk.is_change() || hunk.is_deletion(){

                   for r in hunk.right_from..=hunk.right_to{
                       let final_r = r;


                       let vertex_right  = match graph.filter_map(
                           |i, v| if v.hash.eq(&commit_right) && v.line == final_r { Some(i) } else { None },
                           |i, weight| Some(*weight),
                       ).raw_nodes().get(0){
                           None => { return Err("Error in hunk: large modification".parse().unwrap())}
                           Some(e) => e.weight
                       };

                       let vertex_right_replace  :Vertex = Vertex{
                           hash: commit_right.to_string(),
                           line: r,
                           label: "CHANGE".to_string()
                       };

                       let mut vector_vertex :Vec<NodeIndex> = Vec::<NodeIndex>::new();

                       // Add vertex
                       let vertex_replace = graph.add_node(vertex_right_replace);

                       // Get the vertex source from all edges in vertex_right
                       for edge in graph.edges_directed(vertex_right, Incoming){
                           vector_vertex.push(edge.source());
                       }

                       // Add new edges with vertex source and vertex_replace
                       for v in vector_vertex{
                           graph.add_edge(vertex_replace, v, ());
                       }

                       // Remove vertex_right
                       graph.remove_node(vertex_right);
                   }
                }
            }

            if hunk.is_change() || hunk.is_deletion() {
                pos_l = hunk.left_to + 1;
            }
            if hunk.is_change() || hunk.is_addition() {
                pos_r = hunk.right_to + 1;
            }
        }

        while pos_l <= file_size_at_commit.get(&commit_left).unwrap().clone() as i64 && pos_r <= file_size_at_commit.get(&commit_right).unwrap().clone() as i64 {
            let final_pos_l = pos_l;
            let final_pos_r = pos_r;

            let vertex_left = graph.filter_map(
                |i, v| if v.hash.eq(&commit_left) && v.line == final_pos_l { Some(i) } else { None },
                |i, weight| Some(*weight),
            ).raw_nodes().get(0).unwrap().weight;

            let vertex_right  = graph.filter_map(
                |i, v| if v.hash.eq(&commit_right) && v.line == final_pos_r { Some(i) } else { None },
                |i, weight| Some(*weight),
            ).raw_nodes().get(0).unwrap().weight;

            graph.add_edge(vertex_left, vertex_right, ());

            pos_r = pos_r + 1;
            pos_l = pos_l + 1;
        }
    }


    Ok(graph.clone())
}


pub fn bfs(g: &Graph<Vertex, ()>,  v : &NodeIndex) -> Result<Vertex, String>{
    let graph = g.clone();
    let vertex = graph.node_weight(v.clone()).unwrap();
    let mut bfs = Bfs::new(&graph, v.clone());
    while let Some(nx) = bfs.next(&graph) {
        let aux = graph.node_weight(nx).unwrap();
        if aux.label.eq("CHANGE") && !aux.hash.eq(&vertex.hash){
            return Ok(aux.clone());
        }
    }

    return Err("Not found".to_string());
}