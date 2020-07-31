// Simplest possible merge sort implementation
pub fn mergesort<T: Ord + Copy + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() < 2 {
        return arr.to_vec();
    }
    let middle = arr.len() / 2;
    let left = mergesort(&arr[..middle]);
    let right = mergesort(&arr[middle..]);
    mergesort_helper(left, right)
}

fn mergesort_helper<T: Ord + Copy + Clone>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
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

        let test_nums = mergesort(&test_nums);
        test_nums2.sort();

        assert_eq!(test_nums, test_nums2);
    }
}
