use rand::distributions::{Distribution, Standard};
use std::cmp::Ordering;
use std::fmt::Debug;

type CompareFunc<T> = Box<dyn Fn(&T, &T) -> Ordering>;

pub fn test_suite<T, const N: usize>(sort_fn: &dyn Fn(&mut [T], CompareFunc<T>) -> Vec<T>)
where
    T: Ord + Clone + Debug,
    Standard: Distribution<[T; N]>,
{
    for _ in 0..1000 {
        let mut test_arr = rand::random();
        let mut test_arr2 = test_arr.clone();

        let cmp = |a: &T, b: &T| -> Ordering { a.cmp(b) };
        let test_arr = sort_fn(&mut test_arr, Box::new(cmp));
        test_arr2.sort();

        assert_eq!(test_arr, test_arr2);
    }

    for _ in 0..1000 {
        let mut test_arr = rand::random();
        let mut test_arr2 = test_arr.clone();

        let cmp = |a: &T, b: &T| -> Ordering { b.cmp(a) };
        let test_arr = sort_fn(&mut test_arr, Box::new(cmp));
        test_arr2.sort_by(&cmp);

        assert_eq!(test_arr, test_arr2);
    }
}
