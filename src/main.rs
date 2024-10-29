//! https://writings.stephenwolfram.com/2020/04/finally-we-may-have-a-path-to-the-fundamental-theory-of-physics-and-its-beautiful/
//![Wolfram on Dimension](https://www.wolframphysics.org/technical-introduction/limiting-behavior-and-emergent-geometry/the-notion-of-dimension/)
//!
//! [Wolfram Physics](https://www.wolframphysics.org)
//!
//! https://docs.rs/hypergraph/latest/hypergraph/

use hypergraph::{HyperedgeIndex, Hypergraph, VertexIndex};
use std::collections::HashSet;
use std::fmt;

use eframe::{self, egui, Frame};

struct State {
    graph: Graph,
    // graph_vis: egui_graphs::Graph,
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, _: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add(&mut egui_graphs::GraphView::new(&mut self.graph));
        });
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    /// A unique ID.
    id: u32, // todo: u64?
    /// todo: Experimenting; maybe we need something like this as an easy way
    /// todo to find associated edges. [connections]
    connections: Vec<u32>, // index?
}

#[derive(Clone)]
struct Edge {
    /// Node IDs.
    vertices: Vec<u32>,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.vertices
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

struct Rule {
    /// The inner V ec corresponds to Wolfram's terms Unary, Binary, Ternary. (..., edge, hyperedge).
    /// It can go beyond 3, but the simpler examples are 1, 2, or 3. This is the subscript
    /// value in Wolfram's notation.
    ///
    /// The outer Vec corresponds to the number ofelements the rule applies to. Associated with the non-subscript
    /// value in Wolfram's notation. The ratio of this len in output:input roughly corresponds to how
    /// quickly the graph expands.
    pub input: Vec<Vec<usize>>,
    pub output: Vec<Vec<usize>>,
}

impl Rule {
    /// Match an edge against the rule's input
    /// // todo: Wrong?
    pub fn matches(&self, edges: &[Vec<usize>]) -> bool {
        if edges.len() != self.input.len() {
            return false; // todo: Error instead?
        }

        // self.input.iter().zip(edges).all(|(input_edge, edge)| input_edge == edge)
        //
        let mut result = false;
        for (i, edge) in edges.iter().enumerate() {
            let r = &self.input[i];

            if edge != &self.input[i] {
                result = false;
                break;
            }
        }

        result
    }
}

#[derive(Debug)]
struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(edges: Vec<Vec<usize>>) -> Self {
        let edges = edges
            .into_iter()
            .map(|e| Edge { vertices: e.into() })
            .collect();
        let nodes = vec![]; // Nodes can be dynamically generated if needed
        Self { nodes, edges }
    }

    pub fn apply_rule(&mut self, rule: &Rule) {
        let mut new_edges = vec![];

        let mut mapping = Vec::new();

        // for (i, edge) in self.edges.iter().enumerate() {
        //     let r = rule.input[i];
        //
        //     if i == 0 {
        //         // Assign the rule's first input to the first edge.
        //         mapping.push(r.clone());
        //         continue;
        //     }
        // }

        self.edges = new_edges;
    }

    /// Helper function to recursively traverse and count nodes within distance `r`.
    fn traverse_within_distance(
        &self,
        node: &Node,
        dist_remaining: usize,
        visited: &mut HashSet<u32>,
    ) {
        // If we've already visited this node, skip it
        if !visited.insert(node.id) {
            return;
        }
        // Stop recursion if no distance remains
        if dist_remaining == 0 {
            return;
        }
        // Recur for each connected node
        for conn_id in &node.connections {
            if let Some(neighbor) = self.nodes.iter().find(|n| n.id == *conn_id) {
                self.traverse_within_distance(neighbor, dist_remaining - 1, visited);
            }
        }
    }

    /// https://www.wolframphysics.org/technical-introduction/limiting-behavior-and-emergent-geometry/the-notion-of-dimension/index.html
    /// "For any point X in the graph define Vr(X) to be the number of points in the graph that can
    /// be reached by going at most graph distance r. This can be thought of as the volume of a ball
    /// of radius r in the graph centered at X."
    pub fn v_r(&self, x: &Node, r: usize) -> usize {
        // todo: Is x the node ID?
        // todo: How do we calculate this efficiently?
        // let mut nodes_in_r = Vec::new();
        //         //
        //         // for conn in &x.connections {
        //         //     let mut dist = 0;
        //         //     let node = self.nodes.iter().find(|n| n.id == *conn).unwrap();
        //         //
        //         //     // todo: Recur...
        //         //     if dist >= r {
        //         //         continue;
        //         //     }
        //         //
        //         //     nodes_in_r.push(node)
        //         // }
        //         //
        //         // let unique_items: HashSet<_> = nodes_in_r.into_iter().collect();
        //         // unique_items.len()

        let mut visited = HashSet::new();
        self.traverse_within_distance(x, r, &mut visited);
        visited.len() - 1 // Exclude the starting node
    }

    /// https://www.wolframphysics.org/technical-introduction/limiting-behavior-and-emergent-geometry/the-notion-of-dimension/index.html
    pub fn dimension(&self) -> f32 {
        // Placeholder, this could be based on average node degrees or other criteria
        self.edges.len() as f32 / (self.nodes.len() as f32 + 1.0)
    }

    pub fn visualize(&self) {}
}

fn main() {
    // Wolfram's example of a 3D plane
    let rule_plane = Rule {
        input: vec![vec![0, 1, 1], vec![2, 0, 3]],
        output: vec![vec![1, 4, 1], vec![1, 2, 4], vec![3, 4, 4]],
    };

    let mut graph = Graph::new(vec![vec![0, 1, 1], vec![2, 0, 3]]);

    for _ in 0..10 {
        // graph.apply_rule(&rule_plane);
    }

    println!("Graph: {:?}", graph);

    graph.visualize();
}
