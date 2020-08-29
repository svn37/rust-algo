use std::cmp::Ordering;

pub struct Heap<'a, T: PartialOrd + Clone, F: Fn(&T, &T) -> Ordering> {
    values: Vec<T>,
    cmp: &'a F,
}

impl<'a, T: PartialOrd + Clone, F: Fn(&T, &T) -> Ordering> Heap<'a, T, F> {
    pub fn new(cmp: &'a F) -> Self {
        Self {
            values: Vec::new(),
            cmp,
        }
    }

    pub fn from_slice(arr: &[T], cmp: &'a F) -> Self {
        let mut heap = Self {
            values: arr.to_vec(),
            cmp,
        };
        if let Some(i) = heap.parent(heap.values.len()) {
            for idx in (0..=i).rev() {
                heap.heapify(idx);
            }
        }
        heap
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);

        let mut idx = self.values.len() - 1;
        while let Some(parent) = self.parent(idx) {
            if (*self.cmp)(&self.values[idx], &self.values[parent]) == Ordering::Greater {
                break;
            }
            self.values.swap(idx, parent);
            idx = parent;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop().map(|mut value| {
            if !self.values.is_empty() {
                std::mem::swap(&mut value, &mut self.values[0]);
                self.heapify(0);
            }
            value
        })
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn empty(&self) -> bool {
        self.values.len() == 0
    }

    fn heapify(&mut self, mut idx: usize) {
        let mut min_or_max;
        loop {
            min_or_max = idx;
            if let Some(left) = self.left_child(idx) {
                if (*self.cmp)(&self.values[left], &self.values[min_or_max]) == Ordering::Less {
                    min_or_max = left;
                }
            }
            if let Some(right) = self.right_child(idx) {
                if (*self.cmp)(&self.values[right], &self.values[min_or_max]) == Ordering::Less {
                    min_or_max = right;
                }
            }
            if min_or_max == idx {
                break;
            }
            self.values.swap(min_or_max, idx);
            idx = min_or_max;
        }
    }

    fn left_child(&self, idx: usize) -> Option<usize> {
        let left_child = 2 * idx + 1;
        if left_child >= self.values.len() {
            return None;
        }
        Some(left_child)
    }

    fn right_child(&self, idx: usize) -> Option<usize> {
        let right_child = 2 * idx + 2;
        if right_child >= self.values.len() {
            return None;
        }
        Some(right_child)
    }

    fn parent(&self, idx: usize) -> Option<usize> {
        if idx == 0 {
            return None;
        }
        if idx % 2 == 0 {
            Some((idx - 2) / 2)
        } else {
            Some((idx - 1) / 2)
        }
    }
}

pub fn heapsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: PartialOrd + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mut heap = Heap::from_slice(arr, cmp);
    for elem in arr {
        *elem = heap.pop().unwrap();
    }
}

#[test]
fn heapsort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [i32], cmp| {
        heapsort(arr, &cmp);
        arr.to_vec()
    });
}
