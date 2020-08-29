use std::cmp::Ordering;

// Simplest possible merge sort implementation
pub fn mergesort<T, F>(arr: &[T], cmp: &F) -> Vec<T>
where
    T: PartialOrd + Copy + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() < 2 {
        return arr.to_vec();
    }
    let middle = arr.len() / 2;
    let left = mergesort(&arr[..middle], cmp);
    let right = mergesort(&arr[middle..], cmp);
    mergesort_helper(left, right, cmp)
}

fn mergesort_helper<T, F>(left: Vec<T>, right: Vec<T>, cmp: &F) -> Vec<T>
where
    T: PartialOrd + Copy + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mut result = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if cmp(&left[i], &right[j]) == Ordering::Less {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }
    result
}
