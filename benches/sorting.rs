use algorithms::sort::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn random_array(length: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

fn bench_sort_func<F>(c: &mut Criterion, group_name: &str, mut f: F)
where
    F: FnMut(&mut [i64]),
{
    let mut group = c.benchmark_group(group_name);
    for n in (1..4).map(|i| 10_usize.pow(i)) {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            b.iter(|| {
                f(&mut random_array(n));
            });
        });
    }
}

fn bubblesort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "bubble sort", |v| {
        bubblesort(v, &|a, b| a.partial_cmp(b).unwrap())
    });
}

fn insertionsort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "insertion sort", |v| {
        insertionsort(v, &|a, b| a.partial_cmp(b).unwrap())
    });
}

fn selectionsort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "selection sort", |v| {
        selectionsort(v, &|a, b| a.partial_cmp(b).unwrap())
    });
}

fn heapsort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "heap sort", |v| {
        heapsort(v, &|a, b| a.partial_cmp(b).unwrap())
    });
}

fn std_heapsort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "std heap sort", |v| {
        std_heapsort(v, &|a, b| a.partial_cmp(b).unwrap())
    });
}

fn mergesort_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "merge sort", |v| {
        let _ = mergesort(v, &|a, b| a.partial_cmp(b).unwrap());
    });
}

fn quicksort_lomuto_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "quicksort lomuto", |v| {
        quicksort(
            v,
            &|a, b| a.partial_cmp(b).unwrap(),
            PartitionScheme::Lomuto,
        )
    });
}

fn quicksort_hoare_benchmark(c: &mut Criterion) {
    bench_sort_func(c, "quicksort hoare", |v| {
        quicksort(v, &|a, b| a.partial_cmp(b).unwrap(), PartitionScheme::Hoare)
    });
}

criterion_group!(
    benches,
    bubblesort_benchmark,
    insertionsort_benchmark,
    selectionsort_benchmark,
    heapsort_benchmark,
    std_heapsort_benchmark,
    mergesort_benchmark,
    quicksort_lomuto_benchmark,
    quicksort_hoare_benchmark,
);
criterion_main!(benches);
