use std::cell::UnsafeCell;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use typed_arena::Arena;

pub struct Node<'a, T: Eq + Hash> {
    value: T,
    edges: UnsafeCell<Vec<&'a Node<'a, T>>>,
}

impl<'a, T: Eq + Hash> Node<'a, T> {
    pub fn new<'b>(value: T, arena: &'b Arena<Node<'b, T>>) -> &'b Node<'b, T> {
        arena.alloc(Node {
            value,
            edges: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn dfs<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        let mut stack = Vec::new();
        let mut seen = HashSet::new();
        stack.push(self);

        while let Some(node) = stack.pop() {
            if seen.contains(&node.value) {
                continue;
            }
            f(&node.value);
            seen.insert(&node.value);

            let edges = unsafe { &(*node.edges.get()) };
            edges.iter().rev().for_each(|node| stack.push(node));
        }
    }

    pub fn bfs<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if seen.contains(&node.value) {
                continue;
            }
            f(&node.value);
            seen.insert(&node.value);

            let edges = unsafe { &(*node.edges.get()) };
            edges.iter().for_each(|node| queue.push_back(node));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_arena_traverse() {
        let arena = Arena::new();
        let a = Node::new("A", &arena);

        let b = Node::new("B", &arena);
        let c = Node::new("C", &arena);
        let d = Node::new("D", &arena);
        let e = Node::new("E", &arena);
        let f = Node::new("F", &arena);

        unsafe {
            (*a.edges.get()).push(b);
            (*a.edges.get()).push(c);
            (*a.edges.get()).push(d);

            (*c.edges.get()).push(e);
            (*c.edges.get()).push(f);
            (*c.edges.get()).push(a);
        }
        let mut visited_nodes = Vec::new();
        a.dfs(&mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "E", "F", "D"], visited_nodes);

        let mut visited_nodes = Vec::new();
        a.bfs(&mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "D", "E", "F"], visited_nodes);
    }
}
