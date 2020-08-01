use std::cmp::Ordering::{self, Less};

pub fn bubble_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if cmp(&arr[j + 1], &arr[j]) == Less {
                arr.swap(j, j + 1)
            }
        }
    }
}

pub fn insertion_sort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        for j in (1..=i).rev() {
            if cmp(&arr[j], &arr[j - 1]) == Less {
                arr.swap(j, j - 1)
            } else {
                break;
            }
        }
    }
}

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
fn bubblesort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [u64], cmp| {
        bubble_sort(arr, &cmp);
        arr.to_vec()
    });

    test_suite(|arr: &mut [u64], cmp| {
        bubble_sort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn insertionsort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [u64], cmp| {
        insertion_sort(arr, &cmp);
        arr.to_vec()
    });

    test_suite(|arr: &mut [u64], cmp| {
        insertion_sort(arr, &cmp);
        arr.to_vec()
    });
}

#[test]
fn selectionsort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [u64], cmp| {
        selection_sort(arr, &cmp);
        arr.to_vec()
    });

    test_suite(|arr: &mut [u64], cmp| {
        selection_sort(arr, &cmp);
        arr.to_vec()
    });
}
