use crate::prime::is_prime;

use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};

// inspired by https://github.com/jamesroutley/write-a-hash-table
// currently, it's a very simple hashmap with linear probing

const INITIAL_BASE_SIZE: usize = 53;

pub struct HashTable<K: Eq + Hash, V> {
    buckets: Vec<Option<Entry<K, V>>>,
    items: usize,
}

#[derive(Clone, Debug)]
enum Entry<K: Eq + Hash, V> {
    Item(K, V),
    Tombstone,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    pub fn new() -> Self {
        let mut m = Self {
            buckets: Vec::new(),
            items: 0,
        };
        m.resize(INITIAL_BASE_SIZE);
        m
    }

    pub fn insert(&mut self, key: K, value: V) {
        // if load is above 0.7, resize
        if self.load() > 70 {
            self.resize_up();
        }
        let mut idx = self.find_index(&key);
        loop {
            let cur_item = &mut self.buckets[idx];
            match cur_item {
                Some(Entry::Tombstone) | None => {
                    cur_item.replace(Entry::Item(key, value));
                    self.items += 1;
                    break;
                }
                Some(Entry::Item(cur_key, ..)) if *cur_key == key => {
                    cur_item.replace(Entry::Item(key, value));
                    break;
                }
                _ => idx = self.next_index(idx),
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        // after map is cleared, find_index might try to calculate remainder with zero
        if self.buckets.len() == 0 {
            return None;
        }

        let mut idx = self.find_index(key);
        let start_idx = idx;
        loop {
            match &self.buckets[idx] {
                Some(Entry::Item(cur_key, cur_value)) if cur_key == key => return Some(cur_value),
                None => return None,
                _ => {
                    idx = self.next_index(idx);
                }
            }
            if idx == start_idx {
                return None;
            }
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: &K) {
        // if load is below 0.1, resize
        if self.load() < 10 {
            self.resize_down();
        }

        if self.buckets.len() == 0 {
            return;
        }
        let mut idx = self.find_index(&key);
        let start_idx = idx;
        loop {
            let cur_item = &mut self.buckets[idx];
            match cur_item {
                Some(Entry::Item(cur_key, ..)) if cur_key == key => {
                    cur_item.replace(Entry::Tombstone);
                    self.items -= 1;
                    return;
                }
                None => return,                  // nothing found
                _ => idx = self.next_index(idx), // continue, because key does not match or tombstone
            }
            if idx == start_idx {
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn clear(&mut self) -> Vec<(K, V)> {
        let items: Vec<_> = self
            .buckets
            .drain(..)
            .filter_map(|entry| {
                if let Some(Entry::Item(k, v)) = entry {
                    Some((k, v))
                } else {
                    None
                }
            })
            .collect();
        self.items = 0;
        items
    }

    pub(crate) fn capacity(&self) -> usize {
        self.buckets.len()
    }

    fn find_index(&self, key: &K) -> usize {
        let mut hasher = HasherDJB2::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    fn next_index(&self, idx: usize) -> usize {
        (idx + 1) % self.buckets.len()
    }

    fn resize(&mut self, mut size: usize) {
        if size < INITIAL_BASE_SIZE {
            return;
        }
        let items = self.clear();

        // buckets size had better be a prime
        while !is_prime(size) {
            size += 1;
        }
        self.buckets.resize_with(size, || None);

        for (k, v) in items {
            self.insert(k, v);
        }
    }

    fn load(&self) -> usize {
        self.items * 100 / self.capacity()
    }

    fn resize_up(&mut self) {
        self.resize(self.capacity() * 2)
    }

    fn resize_down(&mut self) {
        self.resize(self.capacity() / 2)
    }
}

impl<K, V> Debug for HashTable<K, V>
where
    K: Eq + Hash + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dm = f.debug_map();
        self.buckets
            .iter()
            .filter_map(|entry| match entry {
                Some(Entry::Item(k, v)) => Some((k, v)),
                _ => None,
            })
            .for_each(|(k, v)| {
                dm.entry(k, v);
            });
        dm.finish()
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Clone for HashTable<K, V> {
    fn clone(&self) -> Self {
        Self {
            buckets: self.buckets.clone(),
            items: self.items,
        }
    }
}

impl<K, V> PartialEq for HashTable<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &HashTable<K, V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.buckets.iter().all(|entry| match entry {
            Some(Entry::Item(key, value)) => other.get(key).map_or(false, |v| *value == *v),
            _ => true,
        })
    }
}

pub struct HasherDJB2 {
    hash: u64,
}

// D. J. Bernstein hash function
// http://cr.yp.to/cdb/cdb.txt
impl HasherDJB2 {
    #[inline]
    pub fn new() -> HasherDJB2 {
        HasherDJB2 { hash: 5381u64 }
    }
}

impl Hasher for HasherDJB2 {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 5)
                .wrapping_add(self.hash)
                .wrapping_add(*byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}
