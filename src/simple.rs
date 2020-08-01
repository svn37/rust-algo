use std::cmp::Ordering::{self, Less};

pub fn selection_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        let mut swap = i;
        for j in i..arr.len() {
            if cmp(&arr[j], &arr[swap]) == Less {
                swap = j
            }
        }
        arr.swap(i, swap)
    }
}

#[test]
fn selection_test() {
    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();
        selection_sort(&mut test_arr, &(|a, b| a.cmp(b)));
        test_arr2.sort();
        assert_eq!(test_arr, test_arr2);
    }
}
