use std::cmp::Ordering;

pub fn bubble_sort<T, F>(arr: &mut [T], cmp: &F)
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

pub fn insertion_sort<T, F>(arr: &mut [T], cmp: &F)
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

pub fn selection_sort<T, F>(arr: &mut [T], cmp: &F)
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
