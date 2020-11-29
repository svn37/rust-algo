// Implementation from this post:
// https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::collections::{HashSet, VecDeque};
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

            for n in self.successors(node) {
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
            for &n in successors.iter().rev() {
                queue.push_back(n);
            }
        }
    }
}

pub struct Successors<'a, T: Eq + Hash> {
    graph: &'a Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'a, T: Eq + Hash> Iterator for Successors<'a, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_edge_index.map(|edge_num| {
            let edge = &self.graph.edges[edge_num];
            self.current_edge_index = edge.next_outgoing_edge;
            edge.target
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_indices_traverse() {
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
}
