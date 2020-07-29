use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::cmp::Ordering::{self, Less};

pub fn quicksort<T: Ord + std::fmt::Debug>(arr: &mut [T]) {
    quicksort_helper(arr, &mut rand::thread_rng(), &(|a, b| a.cmp(b)))
}

fn quicksort_helper<T, F>(nums: &mut [T], rng: &mut ThreadRng, cmp: &F)
where
    T: Ord + std::fmt::Debug,
    F: Fn(&T, &T) -> Ordering,
{
    if nums.len() <= 1 {
        return;
    }

    let pivot = Uniform::from(0..nums.len()).sample(rng);
    nums.swap(pivot, nums.len() - 1);

    let mut i = 0;
    for j in 0..nums.len() {
        if cmp(&nums[j], &nums[nums.len() - 1]) == Less {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, nums.len() - 1);

    quicksort_helper(&mut nums[..i], rng, cmp);
    quicksort_helper(&mut nums[i + 1..], rng, cmp);
}

#[test]
fn quicksort_test() {
    for _ in 0..1000 {
        let mut test_nums: [u64; 8] = rand::random();
        println!("{:?}", test_nums);
        let mut test_nums2 = test_nums.clone();
        quicksort(&mut test_nums);
        test_nums2.sort();
        assert_eq!(test_nums, test_nums2);
    }
}
