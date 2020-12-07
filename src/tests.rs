use crate::hashtable::HashTable;
use rand::distributions::Standard;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;

fn test_suite(sort_fn: impl Fn(&mut [i32], &dyn Fn(&i32, &i32) -> Ordering) -> Vec<i32>) {
    // generate tests like hashtabletps://github.com/rust-lang/rust/blob/847ba835ce411d47364a93ddf0b4a5c0f27928a9/library/alloc/tests/slice.rs
    let rng = thread_rng();
    for len in (2..25).chain(500..510) {
        for &modulus in &[5, 10, 100, 1000] {
            for _ in 0..10 {
                let orig: Vec<_> = rng
                    .sample_iter::<i32, _>(&Standard)
                    .map(|x| x % modulus)
                    .take(len)
                    .collect();

                let mut v = orig.clone();
                let sorted = sort_fn(&mut v, &|a, b| a.cmp(b));
                assert!(sorted.windows(2).all(|w| w[0] <= w[1]));

                let mut v = orig.clone();
                let sorted = sort_fn(&mut v, &|a, b| b.cmp(a));
                assert!(sorted.windows(2).all(|w| w[0] >= w[1]));
            }
        }
    }
}

#[test]
fn heapsort_test() {
    use crate::sort::heapsort;

    test_suite(|arr, cmp| {
        heapsort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn std_heapsort_test() {
    use crate::sort::std_heapsort;

    test_suite(|arr, cmp| {
        std_heapsort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn mergesort_test() {
    use crate::sort::mergesort;

    test_suite(|arr, cmp| mergesort(arr, &cmp));
}

#[test]
fn quicksort_test() {
    use crate::sort::{quicksort, PartitionScheme};

    test_suite(|arr, cmp| {
        quicksort(arr, &cmp, PartitionScheme::Lomuto);
        arr.to_vec()
    });

    test_suite(|arr, cmp| {
        quicksort(arr, &cmp, PartitionScheme::Hoare);
        arr.to_vec()
    });
}

#[test]
fn bubblesort_test() {
    use crate::sort::bubblesort;

    test_suite(|arr, cmp| {
        bubblesort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn insertionsort_test() {
    use crate::sort::insertionsort;

    test_suite(|arr, cmp| {
        insertionsort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn selectionsort_test() {
    use crate::sort::selectionsort;

    test_suite(|arr, cmp| {
        selectionsort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn heap_test() {
    use crate::heap::BinaryHeap;
    use rand::seq::SliceRandom;

    let mut minheap = BinaryHeap::<u64, _>::new(&|a, b| a.cmp(b));
    let mut values = (0..1000).collect::<Vec<_>>();
    values.shuffle(&mut rand::thread_rng());
    for &elem in &values {
        minheap.push(elem);
    }
    for (i, elem) in minheap.into_iter().enumerate() {
        values[i] = elem;
    }
    assert!(values.windows(2).all(|w| w[0] <= w[1]));

    let mut maxheap = BinaryHeap::<u64, _>::new(&|a, b| b.cmp(a));
    values.shuffle(&mut rand::thread_rng());
    for &elem in &values {
        maxheap.push(elem);
    }
    for (i, elem) in maxheap.into_iter().enumerate() {
        values[i] = elem;
    }
    assert!(values.windows(2).all(|w| w[0] >= w[1]));
}

#[test]
fn hashtable_basic() {
    let mut m = HashTable::default();

    m.insert("key", "value");
    m.insert("false", "true");

    assert!(!m.is_empty());
    assert_eq!(m.len(), 2);

    assert_eq!(m.get(&"key"), Some(&"value"));
    assert_eq!(m.get(&"false"), Some(&"true"));

    m.remove(&"key");
    m.remove(&"false");

    assert!(m.is_empty());
    assert_eq!(m.len(), 0);

    assert_eq!(m.get(&"key"), None);
    assert_eq!(m.get(&"false"), None);
}

#[test]
fn hashtable_clear() {
    let mut m = HashTable::default();
    assert!(m.is_empty());

    for i in 1..101 {
        m.insert(format!("{}", i), i)
    }
    assert!(!m.is_empty());
    assert_eq!(m.len(), 100);

    assert_eq!(m.get(&String::from("1")), Some(&1));
    assert_eq!(m.get(&String::from("100")), Some(&100));

    m.clear();
    assert!(m.is_empty());

    assert_eq!(m.get(&String::from("1")), None);
    assert_eq!(m.get(&String::from("100")), None);
}

#[test]
fn hashtable_duplicate_key() {
    let mut m = HashTable::default();

    m.insert("key", 1);
    m.insert("key", 2);
    assert_eq!(m.get(&"key"), Some(&2));

    m.remove(&"key");
    assert_eq!(m.get(&"key"), None);
}

#[test]
fn hashtable_single_remove() {
    let mut m: HashTable<_, u64> = HashTable::default();

    m.remove(&"key");
    assert_eq!(m.get(&"key"), None);
}

#[test]
fn hashtable_overwrite() {
    let mut m = HashTable::default();

    m.insert("ok", 1);
    assert_eq!(m.get(&"ok"), Some(&1));

    m.insert("ok", 2);
    assert_eq!(m.get(&"ok"), Some(&2));
}

#[test]
fn hashtable_remove_nonexistent_item() {
    let mut m = HashTable::default();

    m.remove(&"ok");
    assert!(m.is_empty());
    assert_eq!(m.get(&"ok"), None);

    m.insert("ok1", 1);
    assert!(!m.is_empty());
    assert_eq!(m.get(&"ok1"), Some(&1));

    m.remove(&"ok2");
    assert!(!m.is_empty());
    assert_eq!(m.get(&"ok2"), None);

    m.remove(&"ok1");
    assert!(m.is_empty());
    assert_eq!(m.get(&"ok1"), None);
}

#[test]
fn hashtable_resize() {
    let mut m = HashTable::default();
    let range = 0..m.capacity() * 10;

    for i in range.clone() {
        m.insert(i, i);

        assert_eq!(m.len(), i + 1);
    }

    for i in range.rev() {
        m.remove(&i);

        assert_eq!(m.len(), i);
    }
}

// https://github.com/rust-lang/rust/blob/1facd4a77b181ad44b9c9a64f0fd21b6d5180458/library/std/src/collections/hash/map.rs#L3024
#[test]
fn hashtable_lots_of_insertions() {
    let mut m = HashTable::default();

    // Try this a few times to make sure we never screw up the hashmap's
    // internal state.
    for _ in 0..10 {
        assert!(m.is_empty());

        for i in 1..1001 {
            m.insert(i, i);

            for j in 1..=i {
                let r = m.get(&j);
                assert_eq!(r, Some(&j));
            }

            for j in i + 1..1001 {
                let r = m.get(&j);
                assert_eq!(r, None);
            }
        }

        for i in 1001..2001 {
            assert!(!m.contains_key(&i));
        }

        // remove forwards
        for i in 1..1001 {
            m.remove(&i);

            for j in 1..=i {
                assert!(!m.contains_key(&j));
            }

            for j in i + 1..1001 {
                assert!(m.contains_key(&j));
            }
        }

        for i in 1..1001 {
            assert!(!m.contains_key(&i));
        }

        for i in 1..1001 {
            m.insert(i, i);
        }

        // remove backwards
        for i in (1..1001).rev() {
            m.remove(&i);

            for j in i..1001 {
                assert!(!m.contains_key(&j));
            }

            for j in 1..i {
                assert!(m.contains_key(&j));
            }
        }
    }
}

// https://github.com/rust-lang/rust/blob/1facd4a77b181ad44b9c9a64f0fd21b6d5180458/library/std/src/collections/hash/map.rs#L3250test
#[allow(clippy::redundant_clone)]
#[test]
fn hashtable_clone() {
    let mut m = HashTable::default();
    assert_eq!(m.len(), 0);

    m.insert(1, 2);
    assert_eq!(m.len(), 1);

    m.insert(2, 4);
    assert_eq!(m.len(), 2);

    let m2 = m.clone();
    assert_eq!(*m2.get(&1).unwrap(), 2);
    assert_eq!(*m2.get(&2).unwrap(), 4);
    assert_eq!(m2.len(), 2);
}

// https://github.com/rust-lang/rust/blob/1facd4a77b181ad44b9c9a64f0fd21b6d5180458/library/std/src/collections/hash/map.rs#L3250
#[test]
fn hashtable_eq() {
    let mut m1 = HashTable::default();
    m1.insert(1, 2);
    m1.insert(2, 3);
    m1.insert(3, 4);

    let mut m2 = HashTable::default();
    m2.insert(1, 2);
    m2.insert(2, 3);

    assert!(m1 != m2);

    m2.insert(3, 4);

    assert_eq!(m1, m2);
}

// https://github.com/rust-lang/rust/blob/1facd4a77b181ad44b9c9a64f0fd21b6d5180458/library/std/src/collections/hash/map.rs#L3268
#[test]
fn hashtable_show() {
    let mut map = HashTable::default();
    let empty: HashTable<i32, i32> = HashTable::default();

    map.insert(1, 2);
    map.insert(3, 4);

    let map_str = format!("{:?}", map);

    assert!(map_str == "{1: 2, 3: 4}" || map_str == "{3: 4, 1: 2}");
    assert_eq!(format!("{:?}", empty), "{}");
}
