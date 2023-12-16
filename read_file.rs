use std::fs::File;
use std::io::{BufReader};
use csv::{ReaderBuilder, Trim};

type Node = String;
type EdgeList = Vec<(Node, Node)>;
type AdjacencyLists = Vec<Vec<usize>>;

pub fn read_files(filename: &str) -> EdgeList {
    let file = File::open(filename).expect("could not open file");
    let reader = BufReader::new(file);
    let mut edges = Vec::new();
    let mut csv_reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(reader);
    for result in csv_reader.records() {
        let record = result.expect("could not read record");
        let fields = record.iter().map(|field| field.to_owned()).collect::<Vec<_>>();
        if fields.len() == 2 {
            let name1 = fields[0].to_owned();
            let name2 = fields[1].to_owned();
            edges.push((name1, name2));
        }
    }
    edges
}  