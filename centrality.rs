use std::collections::{HashMap, VecDeque};

type Node = String;
type EdgeList = Vec<(Node, Node)>;
type AdjacencyLists = Vec<Vec<usize>>;

#[derive(Debug, Clone)]
pub struct Graph {
    pub node_count: usize,
    pub outeredges: AdjacencyLists,
    pub node_labels: Vec<Node>,
    pub node_indices: HashMap<Node, usize>,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &EdgeList) {
        for (u, v) in edges {
            let u_index = self.node_indices[u.as_str()];
            let v_index = self.node_indices[v.as_str()];
            self.outeredges[u_index].push(v_index);
        }
    }

    pub fn create_directed(edges: &EdgeList) -> Graph {
        let mut node_labels: Vec<Node> = edges.iter().flat_map(|(u, v)| vec![u.clone(), v.clone()]).collect::<Vec<_>>();
        node_labels.sort();
        node_labels.dedup();

        let node_count = node_labels.len();

        let mut node_indices = HashMap::new();
        for (i, node) in node_labels.iter().enumerate() {
            node_indices.insert(node.clone(), i);
        }

        let mut outeredges: AdjacencyLists = vec![vec![]; node_count];
        let mut g = Graph {
            node_count,
            outeredges,
            node_labels,
            node_indices,
        };

        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outeredges.iter_mut() {
            l.sort();
        }
    }

    pub fn create_undirected(edges: &EdgeList) -> Graph {
        let mut directed_edges: EdgeList = Vec::with_capacity(edges.len() * 2);
        for (u, v) in edges {
            directed_edges.push((u.clone(), v.clone()));
            directed_edges.push((v.clone(), u.clone()));
        }
        let mut g = Graph::create_directed(&directed_edges);
        g.sort_graph_lists();
        g
    }

    pub fn compute_and_print_centralities(graph: &Graph) {
        let node_count = graph.node_count;

        let mut centrality: Vec<f64> = vec![0.0; node_count];

        for s in 0..node_count {
            let mut distance: Vec<Option<u32>> = vec![None; node_count];
            let mut num_paths: Vec<u32> = vec![0; node_count];
            let mut queue: VecDeque<Node> = VecDeque::new();

            distance[s] = Some(0);
            num_paths[s] = 1;
            queue.push_back(graph.node_labels[s].clone());

            while let Some(v) = queue.pop_front() {
                let v_index = graph.node_indices[&v];
                for &u_index in graph.outeredges[v_index].iter() {
                    let u = &graph.node_labels[u_index];
                    let u_index = graph.node_indices[u];
                    if distance[u_index] == None {
                        distance[u_index] = Some(distance[v_index].unwrap() + 1);
                        num_paths[u_index] += num_paths[v_index];
                        queue.push_back(u.clone());
                    } else if distance[u_index].unwrap() == distance[v_index].unwrap() + 1 {
                        num_paths[u_index] += num_paths[v_index];
                    }
                }
            }

            let mut c = 0.0;
            for t in 0..node_count {
                if s != t {
                    let t_index = graph.node_indices[&graph.node_labels[t]];
                    if distance[t_index] == Some(distance[s].unwrap() + 1) {
                        c += num_paths[t_index] as f64 / num_paths[s] as f64;
                    }
                }
            }

            centrality[s] = c;
            println!("{}: {}", graph.node_labels[s], centrality[s]);
        }

        let mut centrality_with_node: Vec<(f64, &str)> = centrality.iter().zip(graph.node_labels.iter()).map(|(c, v)| (*c, v.as_str())).collect();
        centrality_with_node.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        println!("Top 3 centralities:");
        for i in 0..3 {
            let (c, v) = &centrality_with_node[i];
            println!("{}: {}", v, c);
        }
    }
}
