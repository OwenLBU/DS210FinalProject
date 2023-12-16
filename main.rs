mod read_file;
use crate::read_file::read_files;
mod centrality;
use centrality::Graph;

use csv::{ReaderBuilder, Trim};
use std::fs::File;
use std::io::BufReader;

type Node = String;
type EdgeList = Vec<(Node, Node)>;
type AdjacencyLists = Vec<Vec<usize>>;


fn main() {
    let edges = read_files("CryptoExchange1.csv");

    let graph = Graph::create_undirected(&edges);

    // Compute the betweenness centrality scores of each vertex in the graph
    Graph::compute_and_print_centralities(&graph);
}