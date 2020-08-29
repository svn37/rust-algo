#[cfg(test)]
mod tests {
    use rand::distributions::Standard;
    use rand::thread_rng;
    use rand::Rng;
    use std::cmp::Ordering;

    fn test_suite(sort_fn: impl Fn(&mut [i32], Box<dyn Fn(&i32, &i32) -> Ordering>) -> Vec<i32>) {
        // generate tests like https://github.com/rust-lang/rust/blob/847ba835ce411d47364a93ddf0b4a5c0f27928a9/library/alloc/tests/slice.rs
        let rng = thread_rng();
        for len in (2..25).chain(500..510) {
            for &modulus in &[5, 10, 100, 1000] {
                for _ in 0..10 {
                    let orig: Vec<_> = rng
                        .sample_iter::<i32, _>(&Standard)
                        .map(|x| x % modulus)
                        .take(len)
                        .collect();

                    let mut v = orig.clone();
                    let cmp = |a: &i32, b: &i32| -> Ordering { a.cmp(b) };
                    let sorted = sort_fn(&mut v, Box::new(cmp));
                    assert!(sorted.windows(2).all(|w| w[0] <= w[1]));

                    let mut v = orig.clone();
                    let cmp = |a: &i32, b: &i32| -> Ordering { b.cmp(a) };
                    let sorted = sort_fn(&mut v, Box::new(cmp));
                    assert!(sorted.windows(2).all(|w| w[0] >= w[1]));
                }
            }
        }
    }

    #[test]
    fn heapsort_test() {
        use crate::heap::heapsort;

        test_suite(|arr: &mut [i32], cmp| {
            heapsort(arr, &cmp);
            arr.to_vec()
        });
    }

    #[test]
    fn std_heapsort_test() {
        use crate::std_heap::std_heapsort;

        test_suite(|arr: &mut [i32], cmp| {
            std_heapsort(arr, &cmp);
            arr.to_vec()
        });
    }

    #[test]
    fn mergesort_test() {
        use crate::mergesort::mergesort;

        test_suite(|arr: &mut [i32], cmp| mergesort(arr, &cmp));
    }

    #[test]
    fn quicksort_test() {
        use crate::qsort::{quicksort, PartitionScheme};

        test_suite(|arr: &mut [i32], cmp| {
            quicksort(arr, &cmp, PartitionScheme::Lomuto);
            arr.to_vec()
        });

        test_suite(|arr: &mut [i32], cmp| {
            quicksort(arr, &cmp, PartitionScheme::Hoare);
            arr.to_vec()
        });
    }

    #[test]
    fn bubblesort_test() {
        use crate::simple::bubble_sort;

        test_suite(|arr: &mut [i32], cmp| {
            bubble_sort(arr, &cmp);
            arr.to_vec()
        });
    }

    #[test]
    fn insertionsort_test() {
        use crate::simple::insertion_sort;

        test_suite(|arr: &mut [i32], cmp| {
            insertion_sort(arr, &cmp);
            arr.to_vec()
        });
    }

    #[test]
    fn selectionsort_test() {
        use crate::simple::selection_sort;

        test_suite(|arr: &mut [i32], cmp| {
            selection_sort(arr, &cmp);
            arr.to_vec()
        });
    }
}
