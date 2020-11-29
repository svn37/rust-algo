use std::cell::{Ref, RefCell};
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

pub struct Node<T: Eq + Hash> {
    value: T,
    edges: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: Eq + Hash> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value,
            edges: Vec::new(),
        }))
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

            for node in node.edges.iter().rev() {
                stack.push(Ref::leak(node.borrow()))
            }
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

            for node in &node.edges {
                queue.push_back(Ref::leak(node.borrow()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_ref_traverse() {
        let a = Node::new("A");

        let b = Node::new("B");
        let c = Node::new("C");
        let d = Node::new("D");
        let e = Node::new("E");
        let f = Node::new("F");

        {
            let mut a = a.borrow_mut();
            a.edges.push(b.clone());
            a.edges.push(c.clone());
            a.edges.push(d.clone());
        }
        {
            let mut c = c.borrow_mut();
            c.edges.push(e.clone());
            c.edges.push(f.clone());
            c.edges.push(a.clone());
        }

        let mut visited_nodes = Vec::new();
        a.borrow().dfs(&mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "E", "F", "D"], visited_nodes);

        let mut visited_nodes = Vec::new();
        a.borrow().bfs(&mut |&val| visited_nodes.push(val));
        assert_eq!(vec!["A", "B", "C", "D", "E", "F"], visited_nodes);
    }
}
