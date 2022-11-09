use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_partition::{List, SpatialHash, SpatialPartitioner};

pub fn in_circle_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InCircle");

    for radius in [5, 10, 20].iter() {
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

criterion_group!(benches, in_circle_benchmark);
criterion_main!(benches);