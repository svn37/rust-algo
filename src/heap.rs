pub struct Heap<T: PartialOrd + Clone> {
    values: Vec<T>,
}

impl<T: PartialOrd + Clone> Heap<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn from_slice(arr: &[T]) -> Self {
        let mut heap = Self {
            values: arr.to_vec(),
        };
        if let Some(i) = Heap::<T>::parent(heap.values.len()) {
            for idx in (0..=i).rev() {
                heap.heapify(idx);
            }
        }
        heap
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
        let mut i = self.values.len() - 1;
        if let Some(mut parent) = Heap::<T>::parent(i) {
            while self.values[i] < self.values[parent] {
                self.values.swap(i, parent);
                i = parent;
                parent = match Heap::<T>::parent(parent) {
                    Some(parent) => parent,
                    None => break,
                }
            }
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut value = self.values.pop()?;
        if !self.values.is_empty() {
            std::mem::swap(&mut value, &mut self.values[0]);
            self.heapify(0);
        }
        Some(value)
    }

    pub fn empty(&self) -> bool {
        self.values.len() == 0
    }

    fn heapify(&mut self, idx: usize) {
        let left = self.left_child(idx);
        let right = self.right_child(idx);

        let mut smallest = idx;
        if let Some(left) = left {
            if self.values[left] < self.values[smallest] {
                smallest = left;
            }
        }
        if let Some(right) = right {
            if self.values[right] < self.values[smallest] {
                smallest = right;
            }
        }
        if smallest != idx {
            self.values.swap(smallest, idx);
            self.heapify(smallest)
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

    fn parent(idx: usize) -> Option<usize> {
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

pub fn heapsort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    let mut heap = Heap::from_slice(arr);
    for i in 0..arr.len() {
        arr[i] = heap.pop().unwrap();
    }
}

#[test]
fn heapsort_test() {
    use rand::distributions::Standard;
    use rand::thread_rng;
    use rand::Rng;

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
                heapsort(&mut v);
                assert!(v.windows(2).all(|w| w[0] <= w[1]));
            }
        }
    }
}
