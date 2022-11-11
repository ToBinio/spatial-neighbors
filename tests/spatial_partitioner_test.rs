use spatial_partition::{SpatialPartitioner};
use spatial_partition::quad_tree::QuadTree;
use spatial_partition::grid::Grid;

fn fill_spatial_partitioner(spatial_hash: &mut Grid<i32>, quad_tree: &mut QuadTree<i32>) {
    for x in -49..50 {
        for y in -49..50 {
            if ((x + 50) * 100 + y) % 3 == 0 || ((x + 50) * 100 + y) % 4 == 0 {
                continue;
            }

            spatial_hash.insert((x as f64, y as f64), (x + 50) * 100 + y);
            quad_tree.insert((x as f64, y as f64), (x + 50) * 100 + y);
        }
    }
}

#[test]
fn in_circle_different_cell_counts() {
    let cell_count = [4, 7, 100, 1000];

    for cell_count in cell_count {
        let mut spatial_hash = Grid::new((-50.0, 50.0), (-50.0, 50.0), cell_count, cell_count / 2);
        let mut quad_tree = QuadTree::new((-50.0, 50.0), (-50.0, 50.0), cell_count as u16);

        fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

        assert_eq!(spatial_hash.in_circle((0.0, 0.0), 10.0).len(), quad_tree.in_circle((0.0, 0.0), 10.0).len());
    }
}

#[test]
fn in_circle_different_location() {
    let location = [(0.0, 0.0), (-10.0, 0.0), (-20.0, -20.0), (15.0, 10.0), (-20.0, 10.0), (15.0, 10.0)];

    let mut spatial_hash = Grid::new((-50.0, 50.0), (-50.0, 50.0), 500, 50);
    let mut quad_tree = QuadTree::new((-50.0, 50.0), (-50.0, 50.0), 4);

    fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

    for location in location {
        assert_eq!(spatial_hash.in_circle(location, 2.0).len(), quad_tree.in_circle(location, 2.0).len());
    }
}

#[test]
fn in_circle_different_sizes() {
    let sizes = [1, 7, 100, 200];

    let mut spatial_hash = Grid::new((-50.0, 50.0), (-50.0, 50.0), 50, 50);
    let mut quad_tree = QuadTree::new((-50.0, 50.0), (-50.0, 50.0), 4);

    fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

    for size in sizes {
        assert_eq!(spatial_hash.in_circle((0.0, 0.0), size as f64).len(), quad_tree.in_circle((0.0, 0.0), size as f64).len());
    }
}

#[test]
fn in_circle_on_borders() {
    let location = [(-49.5, 0.0), (48.3, -49.9), (12.0, 49.7), (49.0, 49.0), (-49.0, -49.0)];

    let mut spatial_hash = Grid::new((-50.0, 50.0), (-50.0, 50.0), 50, 50);
    let mut quad_tree = QuadTree::new((-50.0, 50.0), (-50.0, 50.0), 4);

    fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

    for location in location {
        assert_eq!(spatial_hash.in_circle(location, 2.0).len(), quad_tree.in_circle(location, 2.0).len());
    }
}

#[test]
fn count_and_clear() {
    let mut spatial_hash = Grid::new((-50.0, 50.0), (-50.0, 50.0), 50, 50);
    let mut quad_tree = QuadTree::new((-50.0, 50.0), (-50.0, 50.0), 4);

    fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

    assert_eq!(spatial_hash.count(), quad_tree.count());

    spatial_hash.clear();
    quad_tree.clear();

    assert_eq!(spatial_hash.count(), quad_tree.count());

    fill_spatial_partitioner(&mut spatial_hash, &mut quad_tree);

    assert_eq!(spatial_hash.count(), quad_tree.count());
}