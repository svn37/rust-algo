use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    elem: T,
    cmp: F,
}

impl<T, F> PartialEq for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}
impl<T, F> Eq for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
}

impl<T, F> Ord for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn cmp(&self, other: &Rev<T, F>) -> Ordering {
        (self.cmp)(&other.elem, &self.elem)
    }
}
impl<T, F> PartialOrd for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn partial_cmp(&self, other: &Rev<T, F>) -> Option<Ordering> {
        Some((self.cmp)(&other.elem, &self.elem))
    }
}

pub fn heapsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    let mut heap = BinaryHeap::with_capacity(arr.len());
    for i in 0..arr.len() {
        let elem = unsafe { std::ptr::read(&arr[i]) };
        heap.push(Rev { elem, cmp });
    }
    for i in 0..arr.len() {
        let Rev { elem, .. } = heap.pop().unwrap();
        arr[i] = elem
    }
}

#[test]
fn heapsort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [u64], cmp| {
        heapsort(arr, &cmp);
        arr.to_vec()
    });

    test_suite(|arr: &mut [u64], cmp| {
        heapsort(arr, &cmp);
        arr.to_vec()
    });
}
