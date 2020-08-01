use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::cmp::Ordering::{self, Less};

// Lomuto partition scheme quicksort
pub fn quicksort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    quicksort_helper(arr, &mut rand::thread_rng(), cmp)
}

fn quicksort_helper<T, F>(arr: &mut [T], rng: &mut ThreadRng, cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }

    let pivot = Uniform::from(0..arr.len()).sample(rng);
    arr.swap(pivot, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() {
        if cmp(&arr[j], &arr[arr.len() - 1]) == Less {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);

    quicksort_helper(&mut arr[..i], rng, cmp);
    quicksort_helper(&mut arr[i + 1..], rng, cmp);
}

#[test]
fn quicksort_test() {
    for _ in 0..1000 {
        let mut test_arr: [u64; 8] = rand::random();
        let mut test_arr2 = test_arr.clone();
        quicksort(&mut test_arr, &(|a, b| a.cmp(b)));
        test_arr2.sort();
        assert_eq!(test_arr, test_arr2);
    }
}
