use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_partition::{SpatialPartitioner};
use spatial_partition::list::List;
use spatial_partition::spatial_hash::SpatialHash;

pub fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert");

    for size in [5_000, 20_000, 50_000].iter() {
        group.bench_with_input(BenchmarkId::new("List", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut list = List {
                    data: Vec::new()
                };

                for x in (-size + 1)..size {
                    for y in (-size + 1)..size {
                        list.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet10", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 10, 10);

                for x in (-size + 1)..size {
                    for y in (-size + 1)..size {
                        spatial_hash.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet100", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 100, 100);

                for x in (-size + 1)..size {
                    for y in (-size + 1)..size {
                        spatial_hash.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet1000", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 1000, 1000);

                for x in (-size + 1)..size {
                    for y in (-size + 1)..size {
                        spatial_hash.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);