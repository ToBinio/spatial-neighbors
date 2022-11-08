use spatial_partition::{SpatialHash, SpatialPartitioner};

fn main() {
    let mut spatial_hash = SpatialHash::new((-50.0, 50.0), (-50.0, 50.0), 5, 5);

    for x in -50..50 {
        for y in -50..50 {
            spatial_hash.insert((x as f64, y as f64), (x + 50) * 100 + y);
        }
    }

    println!("{:?}", spatial_hash.in_circle((0.0, 0.0), 10.0).len());
}