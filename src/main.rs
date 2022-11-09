use spatial_partition::{SpatialPartitioner};
use spatial_partition::spatial_hash::SpatialHash;

fn main() {
    let mut spatial_hash = SpatialHash::new((-500.0, 500.0), (-500.0, 500.0), 1000, 1000);

    for x in -499..500 {
        for y in -499..500 {
            spatial_hash.insert((x as f64, y as f64), (x + 50) * 100 + y);
        }
    }

    println!("{:?}", spatial_hash.in_circle((0.0, 0.0), 20.0).len());
}