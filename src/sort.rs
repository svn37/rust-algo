use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::cmp::Ordering;
use std::collections::BinaryHeap as StdBinaryHeap;

use crate::heap::BinaryHeap;
use crate::utils::Rev;

// ============ quicksort ============

pub enum PartitionScheme {
    Lomuto,
    Hoare,
}

pub fn quicksort<T, F>(arr: &mut [T], cmp: &F, scheme: PartitionScheme)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    match scheme {
        PartitionScheme::Lomuto => quicksort_lomuto(arr, &mut rand::thread_rng(), cmp),
        PartitionScheme::Hoare => quicksort_hoare(arr, &mut rand::thread_rng(), cmp),
    }
}

fn quicksort_lomuto<T, F>(arr: &mut [T], rng: &mut ThreadRng, cmp: &F)
where
    T: PartialOrd,
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
    T: PartialOrd,
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
        while cmp(&arr[j], &arr[pivot]) == Ordering::Greater {
            j -= 1;
        }
        if i >= j {
            break;
        }
        if arr[i] == arr[pivot] && arr[j] == arr[pivot] {
            i += 1;
            continue;
        }
        arr.swap(i, j);
    }

    quicksort_hoare(&mut arr[..j], rng, cmp);
    quicksort_hoare(&mut arr[j + 1..], rng, cmp);
}

// ============ mergesort ============

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

// ============ heapsort ============

pub fn heapsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: PartialOrd + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    BinaryHeap::from_vec(arr.to_vec(), cmp)
        .into_iter()
        .enumerate()
        .for_each(|(i, elem)| arr[i] = elem)
}

pub fn std_heapsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    let mut heap = StdBinaryHeap::with_capacity(arr.len());
    for i in 0..arr.len() {
        let elem = unsafe { std::ptr::read(&arr[i]) };
        heap.push(Rev { elem, cmp });
    }
    for i in 0..arr.len() {
        let Rev { elem, .. } = heap.pop().unwrap();
        arr[i] = elem
    }
}

// ============ simplest sorting algorithms ============

pub fn bubblesort<T, F>(arr: &mut [T], cmp: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if cmp(&arr[j + 1], &arr[j]) == Ordering::Less {
                arr.swap(j, j + 1)
            }
        }
    }
}

pub fn insertionsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        for j in (1..=i).rev() {
            if cmp(&arr[j], &arr[j - 1]) == Ordering::Less {
                arr.swap(j, j - 1)
            } else {
                break;
            }
        }
    }
}

pub fn selectionsort<T, F>(arr: &mut [T], cmp: &F)
where
    T: PartialOrd,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 0..arr.len() {
        let mut swap = i;
        for j in i..arr.len() {
            if cmp(&arr[j], &arr[swap]) == Ordering::Less {
                swap = j
            }
        }
        arr.swap(i, swap)
    }
}
