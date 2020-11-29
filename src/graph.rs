// Implementation from this post:
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::hash::Hash;

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData<T: Eq + Hash> {
    first_outgoing_edge: Option<EdgeIndex>,
    value: T,
}

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Graph<T: Eq + Hash> {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}

impl<T: Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, value: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            first_outgoing_edge: None,
            value,
        });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge,
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn add_undirected_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.add_edge(source, target);
        self.add_edge(target, source);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge,
        }
    }

    pub fn get_value(&self, node_index: NodeIndex) -> &T {
        &self.nodes[node_index].value
    }

    pub fn dfs<F>(&self, start_node: NodeIndex, f: &mut F)
    where
        F: FnMut(&T),
    {
        let mut stack = Vec::new();
        let mut seen = HashSet::new();
        stack.push(start_node);

        while let Some(node) = stack.pop() {
            if seen.contains(&node) {
                continue;
            }
            f(&self.get_value(node));
            seen.insert(node);

            for (n, _) in self.successors(node) {
                stack.push(n)
            }
        }
    }

    pub fn bfs<F>(&self, start_node: NodeIndex, f: &mut F)
    where
        F: FnMut(&T),
    {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(start_node);

        while let Some(node) = queue.pop_front() {
            if seen.contains(&node) {
                continue;
            }
            f(&self.get_value(node));
            seen.insert(node);

            let successors: Vec<_> = self.successors(node).collect();
            for &(n, _) in successors.iter().rev() {
                queue.push_back(n);
            }
        }
    }

    // The algorithm requires positive edge lengths and will break with negative edge lengths
    pub fn dijkstra(&self, source_node: NodeIndex, edge_weights: &[u64]) -> Vec<u64> {
        // distances from node to source
        let mut dist_vec = vec![u64::max_value(); self.nodes.len()];
        dist_vec[source_node] = 0;

        let mut heap = BinaryHeap::new();
        heap.push((Reverse(dist_vec[source_node]), source_node));

        while let Some((Reverse(dist), cur_node)) = heap.pop() {
            if dist_vec[cur_node] == dist {
                for (node, edge) in self.successors(cur_node) {
                    // Calculate Dijkstra's greedy score
                    let tent_dist = dist + edge_weights[edge];
                    if tent_dist < dist_vec[node] {
                        dist_vec[node] = tent_dist;
                        heap.push((Reverse(tent_dist), node));
                    }
                }
            }
        }
        dist_vec
    }
}

pub struct Successors<'a, T: Eq + Hash> {
    graph: &'a Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'a, T: Eq + Hash> Iterator for Successors<'a, T> {
    type Item = (NodeIndex, EdgeIndex);

    fn next(&mut self) -> Option<Self::Item> {
        self.current_edge_index.map(|edge_num| {
            let edge = &self.graph.edges[edge_num];
            self.current_edge_index = edge.next_outgoing_edge;
            (edge.target, edge_num)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_traverse() {
        let mut graph = Graph::new();

        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");
        let e = graph.add_node("E");
        let f = graph.add_node("F");

        graph.add_edge(a, b);
        graph.add_edge(a, c);
        graph.add_edge(a, d);

        graph.add_edge(c, e);
        graph.add_edge(c, f);
        graph.add_edge(c, a);

        let mut visited_nodes = Vec::new();
        graph.dfs(a, &mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "E", "F", "D"], visited_nodes);

        let mut visited_nodes = Vec::new();
        graph.bfs(a, &mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "D", "E", "F"], visited_nodes);
    }

    #[test]
    fn graph_dijkstra() {
        let mut graph = Graph::new();

        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");
        let e = graph.add_node("E");
        let f = graph.add_node("F");
        let g = graph.add_node("G");
        let h = graph.add_node("H");

        let mut weights = Vec::new();
        for (nodes, weight) in vec![
            ((a, b), 8),
            ((a, c), 3),
            ((b, e), 3),
            ((c, e), 4),
            ((c, d), 1),
            ((e, f), 2),
            ((f, h), 1),
            ((f, g), 7),
            ((b, g), 4),
            ((a, h), 5),
        ] {
            graph.add_undirected_edge(nodes.0, nodes.1);
            weights.append(&mut vec![weight, weight]);
        }

        assert_eq!(vec![0, 8, 3, 4, 7, 6, 12, 5], graph.dijkstra(a, &weights))
    }
}
