use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use spatial_neighbors::{SpatialPartitioner};
use spatial_neighbors::quad_tree::QuadTree;

pub fn in_circle_radius_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InCircleRadiusQuadTree");

    for radius in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("QuadTree10", radius), radius, |b, radius| {
            let mut quad_tree = black_box(QuadTree::new(-500.0..500.0, -500.0..500.0, 10));

            for x in -500..500 {
                for y in -500..500 {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), *radius as f64)
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree25", radius), radius, |b, radius| {
            let mut quad_tree = black_box(QuadTree::new(-500.0..500.0, -500.0..500.0, 25));

            for x in -500..500 {
                for y in -500..500 {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), *radius as f64)
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree50", radius), radius, |b, radius| {
            let mut quad_tree = black_box(QuadTree::new(-500.0..500.0, -500.0..500.0, 50));

            for x in -500..500 {
                for y in -500..500 {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), *radius as f64)
            })
        });
    }
}

pub fn in_circle_count_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("InCircleCountQuadTree");

    for size in [5_000, 20_000, 50_000].iter() {
        group.bench_with_input(BenchmarkId::new("QuadTree10", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut quad_tree = black_box(QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 10));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), 10.0)
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree25", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut quad_tree = black_box(QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 25));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), 10.0)
            })
        });

        group.bench_with_input(BenchmarkId::new("QuadTree50", size), size, |b, size| {
            let size = black_box((*size as f64).sqrt() as i32);

            let mut quad_tree = black_box(QuadTree::new(-size as f64..size as f64, -size as f64..size as f64, 50));

            for x in (-size + 1)..size {
                for y in (-size + 1)..size {
                    quad_tree.insert((x as f64, y as f64), 1);
                }
            }

            b.iter(|| {
                quad_tree.in_circle((0.0, 0.0), 10.0)
            })
        });
    }

    group.finish();
}

criterion_group!(benches, in_circle_count_benchmark, in_circle_radius_benchmark);
criterion_main!(benches);