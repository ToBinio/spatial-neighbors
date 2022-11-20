use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_neighbors::{SpatialPartitioner};
use spatial_neighbors::grid::Grid;

pub fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InsertGrid");

    for size in [1_000, 10_000, 20_000].iter() {
        group.bench_with_input(BenchmarkId::new("Grid10", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = Grid::with_cell_count(-size as f64..size as f64, -size as f64..size as f64, (10, 10));

                for x in -size..size {
                    for y in -size..size {
                        spatial_hash.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("Grid100", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = Grid::with_cell_count(-size as f64..size as f64, -size as f64..size as f64,(100, 100));

                for x in -size..size {
                    for y in -size..size {
                        spatial_hash.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("Grid1000", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut spatial_hash = Grid::with_cell_count(-size as f64..size as f64, -size as f64..size as f64, (1000, 1000));

                for x in -size..size {
                    for y in -size..size {
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