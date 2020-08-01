use std::cmp::Ordering::{self, Less};

// Simplest possible merge sort implementation
pub fn mergesort<T, F>(arr: &[T], cmp: &F) -> Vec<T>
where
    T: Ord + Copy + Clone,
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
    T: Ord + Copy + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let mut result = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if cmp(&left[i], &right[j]) == Less {
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

#[test]
fn mergesort_test() {
    for _ in 0..1000 {
        let test_nums: [u64; 8] = rand::random();
        let mut test_nums2 = test_nums.clone();

        let test_nums = mergesort(&test_nums, &(|a, b| a.cmp(b)));
        test_nums2.sort();

        assert_eq!(test_nums, test_nums2);
    }
}
