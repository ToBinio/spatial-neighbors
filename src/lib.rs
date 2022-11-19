//! # Spatial Neighbors
//!
//! Simple to use implementation of certain Spatial-Partitioning Algorithms | DataStructures
//!
//! Useful for finding all objects  in a given area
//!
//! DataStructures
//! - [QuadTree]
//! - Grid | SpatialHash
//!
//! [QuadTree]: https://en.wikipedia.org/wiki/Quadtree

pub(crate) mod util;
pub mod grid;
pub mod quad_tree;

extern crate core;

use std::ops::Range;

pub trait SpatialPartitioner<Data: Copy> {
    /// create a new SpatialPartitioner which can handle positions in the given ranges
    ///
    /// # Arguments
    ///
    /// * `x`: min_x..max_x defines the area in wich data can be inserted
    /// * `y`: min_y..max_y defines the area in wich data can be inserted
    ///

    fn new(x: Range<f64>, y: Range<f64>) -> Self;
    /// inserts an obj and checks if position is out of bounce
    ///
    /// # Arguments
    ///
    /// * `position`: position of the data (x,y)
    /// * `data`: the DataValue which will be return when queued
    /// e.g index of an other list which stores the "real" data
    ///
    fn insert(&mut self, position: (f64, f64), data: Data);
    /// inserts an obj and DOES NOT check if position is out of bounce
    ///
    /// # Arguments
    ///
    /// * `position`: position of the data (x,y)
    /// * `data`: the DataValue which will be return when queued.
    /// e.g index of an other list which stores the "real" data
    ///
    fn insert_unchecked(&mut self, position: (f64, f64), data: Data);

    /// returns the count of DataValues which are currently stored
    fn count(&self) -> usize;

    fn clear(&mut self);

    /// gets all DataValues in the given search circle
    ///
    /// # Arguments
    ///
    /// * `position`: center of the search circle
    /// * `radius`: radius of the search circle
    ///
    /// returns: Vec<Data>
    ///
    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data>;
}