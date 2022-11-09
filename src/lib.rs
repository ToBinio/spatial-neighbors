pub(crate) mod util;
pub mod spatial_hash;
pub mod list;
pub mod quad_tree;

extern crate core;

pub trait SpatialPartitioner<Data: Clone> {
    fn insert(&mut self, position: (f64, f64), data: Data);
    fn insert_unchecked(&mut self, position: (f64, f64), data: Data);

    fn count(&self) -> usize;

    fn clear(&mut self);

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data>;
}