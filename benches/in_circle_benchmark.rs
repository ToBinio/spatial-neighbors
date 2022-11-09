use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_partition::{SpatialPartitioner};
use spatial_partition::list::List;
use spatial_partition::spatial_hash::SpatialHash;

pub fn in_circle_radius_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InCircleRadius");

    for radius in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("List", radius), radius, |b, radius| {
            let mut list = black_box(List {
                data: Vec::new()
            });

            for x in (-499)..500 {
                for y in (-499)..500 {
                    list.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                list.in_circle((0.0, 0.0), *radius as f64)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet10", radius), radius, |b, radius| {
            let mut spatial_hash = black_box(SpatialHash::new((-500 as f64, 500 as f64), (-500 as f64, 500 as f64), 10, 10));

            for x in (-499)..500 {
                for y in (-499)..500 {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), *radius as f64)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet100", radius), radius, |b, radius| {
            let mut spatial_hash = black_box(SpatialHash::new((-500 as f64, 500 as f64), (-500 as f64, 500 as f64), 100, 100));

            for x in (-499)..500 {
                for y in (-499)..500 {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), *radius as f64)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet1000", radius), radius, |b, radius| {
            let mut spatial_hash = SpatialHash::new((-500 as f64, 500 as f64), (-500 as f64, 500 as f64), 1000, 1000);

            for x in (-499)..500 {
                for y in (-499)..500 {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), *radius as f64)
            })
        });
    }

    group.finish();
}

pub fn in_circle_count_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InCircleCount");

    for size in [5_000, 20_000, 50_000].iter() {
        group.bench_with_input(BenchmarkId::new("List", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut list = black_box(List {
                data: Vec::new()
            });

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    list.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                list.in_circle((0.0, 0.0), 10.0)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet10", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut spatial_hash = black_box(SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 10, 10));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), 10.0)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet100", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut spatial_hash = black_box(SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 100, 100));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), 10.0)
            })
        });

        group.bench_with_input(BenchmarkId::new("HashSet1000", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut spatial_hash = black_box(SpatialHash::new((-size as f64, size as f64), (-size as f64, size as f64), 1000, 1000));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    spatial_hash.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                spatial_hash.in_circle((0.0, 0.0), 10.0)
            })
        });
    }

    group.finish();
}

criterion_group!(benches, in_circle_count_benchmark, in_circle_radius_benchmark);
criterion_main!(benches);