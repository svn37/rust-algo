use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::cmp::Ordering;

pub enum PartitionScheme {
    Lomuto,
    Hoare,
}

pub fn quicksort<T, F>(arr: &mut [T], cmp: &F, scheme: PartitionScheme)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    match scheme {
        PartitionScheme::Lomuto => quicksort_lomuto(arr, &mut rand::thread_rng(), cmp),
        PartitionScheme::Hoare => quicksort_hoare(arr, &mut rand::thread_rng(), cmp),
    }
}

fn quicksort_lomuto<T, F>(arr: &mut [T], rng: &mut ThreadRng, cmp: &F)
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
        if cmp(&arr[j], &arr[arr.len() - 1]) == Ordering::Less {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);

    quicksort_lomuto(&mut arr[..i], rng, cmp);
    quicksort_lomuto(&mut arr[i + 1..], rng, cmp);
}

fn quicksort_hoare<T, F>(arr: &mut [T], rng: &mut ThreadRng, cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }

    let pivot = 0;
    let (mut i, mut j) = (0, arr.len() - 1);

    loop {
        while cmp(&arr[i], &arr[pivot]) == Ordering::Less {
            i += 1;
        }
        while cmp(&arr[pivot], &arr[j]) == Ordering::Less {
            j -= 1;
        }
        if i >= j {
            break;
        }
        arr.swap(i, j);
    }

    quicksort_hoare(&mut arr[..j + 1], rng, cmp);
    quicksort_hoare(&mut arr[j + 1..], rng, cmp);
}

#[test]
fn quicksort_test() {
    use crate::utils::test_suite;

    test_suite(|arr: &mut [u64], cmp| {
        quicksort(arr, &cmp, PartitionScheme::Lomuto);
        arr.to_vec()
    });

    test_suite(|arr: &mut [u64], cmp| {
        quicksort(arr, &cmp, PartitionScheme::Hoare);
        arr.to_vec()
    });
}
