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
    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        bubble_sort(&mut test_arr, &(|a, b| a.cmp(b)));
        test_arr2.sort();

        assert_eq!(test_arr, test_arr2);
    }

    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        bubble_sort(&mut test_arr, &(|a, b| b.cmp(a)));
        test_arr2.sort_by(|a, b| b.cmp(a));

        assert_eq!(test_arr, test_arr2);
    }
}

#[test]
fn insertionsort_test() {
    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        insertion_sort(&mut test_arr, &(|a, b| a.cmp(b)));
        test_arr2.sort();

        assert_eq!(test_arr, test_arr2);
    }

    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        insertion_sort(&mut test_arr, &(|a, b| b.cmp(a)));
        test_arr2.sort_by(|a, b| b.cmp(a));

        assert_eq!(test_arr, test_arr2);
    }
}

#[test]
fn selectionsort_test() {
    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        selection_sort(&mut test_arr, &(|a, b| a.cmp(b)));
        test_arr2.sort();

        assert_eq!(test_arr, test_arr2);
    }

    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();

        selection_sort(&mut test_arr, &(|a, b| b.cmp(a)));
        test_arr2.sort_by(|a, b| b.cmp(a));

        assert_eq!(test_arr, test_arr2);
    }
}
