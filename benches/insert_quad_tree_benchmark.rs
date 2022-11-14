use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_neighbors::{SpatialPartitioner};
use spatial_neighbors::quad_tree::QuadTree;

pub fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InsertQuadTree");

    for size in [500, 2_000, 5_000].iter() {
        group.bench_with_input(BenchmarkId::new("QuadTree10", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut quad_tree = QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 10);

                for x in -size..size {
                    for y in -size..size {
                        quad_tree.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree25", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut quad_tree = QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 25);

                for x in -size..size {
                    for y in -size..size {
                        quad_tree.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree50", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            b.iter(|| {
                let mut quad_tree = QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 50);

                for x in -size..size {
                    for y in -size..size {
                        quad_tree.insert((x as f64, y as f64), 1);
                    }
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, insert_benchmark);
criterion_main!(benches);