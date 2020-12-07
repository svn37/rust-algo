#![warn(rust_2018_idioms)]
#![allow(clippy::needless_range_loop)]
#![feature(cell_leak)]

pub mod graph;
pub mod graph_arena;
pub mod graph_ref;
pub mod hashtable;
pub mod heap;
pub mod prime;
pub mod sort;

#[cfg(test)]
mod tests;
mod utils;
