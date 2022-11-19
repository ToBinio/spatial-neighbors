use std::ops::Range;
use crate::SpatialPartitioner;
use crate::util::in_range;

pub struct QuadTree<Data: Copy> {
    node: QuadTreeNode<Data>,
    min_size: Option<f64>,
    capacity: u16,

    x: Range<f64>,
    y: Range<f64>,

    count: usize,

}

impl<Data: Copy> QuadTree<Data> {
    ///
    /// # Arguments
    ///
    /// * `x`: min_x..max_x defines the area in wich points can be inserted
    /// * `y`: min_y..max_y defines the area in wich points can be inserted
    /// * `capacity`: capacity of each TreeNode
    ///
    pub fn new(x: Range<f64>, y: Range<f64>, capacity: u16) -> QuadTree<Data> {
        QuadTree {
            node: QuadTreeNode::new(((x.end - x.start) / 2.0, (y.end - y.start) / 2.0), ((x.end - x.start), (y.end - y.start)), capacity, None),
            min_size: None,
            capacity,
            x,
            y,
            count: 0,
        }
    }

    ///
    /// # Arguments
    ///
    /// * `x`: min_x..max_x defines the area in wich points can be inserted
    /// * `y`: min_y..max_y defines the area in wich points can be inserted
    /// * `capacity`: capacity of each TreeNode
    /// * `min_size`: defines a size which a node_size can not fall below. Useful to optimize a QuadTree if you know the you will always hava a search radius of at leased min_size.
    ///
    pub fn new_min_size(x: Range<f64>, y: Range<f64>, capacity: u16, min_size: f64) -> QuadTree<Data> {
        let mut tree = Self::new(x, y, capacity);
        tree.min_size = Some(min_size);

        tree
    }
}

impl<Data: Copy> SpatialPartitioner<Data> for QuadTree<Data> {
    fn insert(&mut self, position: (f64, f64), data: Data) {
        if position.0 < self.x.start || position.0 >= self.x.end || position.1 < self.y.start || position.1 >= self.y.end {
            panic!("tried to insert position into QuadTree which was out of bounce")
        }

        self.insert_unchecked(position, data);
    }

    fn insert_unchecked(&mut self, position: (f64, f64), data: Data) {
        self.count += 1;
        self.node.insert(position, data, self.min_size);
    }

    fn count(&self) -> usize {
        self.count
    }

    fn clear(&mut self) {
        self.node = QuadTreeNode::new(((self.x.start - self.x.end) / 2.0, (self.y.start - self.y.end) / 2.0), ((self.x.start - self.x.end), (self.y.start - self.y.end)), self.capacity, self.min_size);
        self.count = 0;
    }

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data> {
        let mut data = Vec::new();

        self.node.in_circle(position, radius, &mut data, false);

        data
    }
}

struct QuadTreeNode<Data: Copy> {
    data: Vec<((f64, f64), Data)>,
    capacity: u16,

    is_full: bool,
    is_min_size: bool,

    nodes: Option<Box<[QuadTreeNode<Data>; 4]>>,

    center: (f64, f64),
    size: (f64, f64),
}

impl<Data: Copy> QuadTreeNode<Data> {
    fn new(center: (f64, f64), size: (f64, f64), capacity: u16, min_size: Option<f64>) -> QuadTreeNode<Data> {
        let mut is_min_size = false;

        match min_size {
            None => {}
            Some(min_size) => {
                if size.0 <= min_size || size.1 <= min_size {
                    is_min_size = true;
                }
            }
        }

        QuadTreeNode {
            data: Vec::new(),
            nodes: None,
            center,
            size,
            capacity,

            is_full: false,
            is_min_size,
        }
    }

    fn insert(&mut self, location: (f64, f64), data: Data, min_size: Option<f64>) {
        if !self.is_full {
            self.data.push((location, data));

            if !self.is_min_size && self.data.len() >= self.capacity as usize {
                self.is_full = true;

                self.nodes = Some(Box::new([
                    QuadTreeNode::new((self.center.0 - self.size.0 / 2.0, self.center.1 - self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity, min_size),
                    QuadTreeNode::new((self.center.0 - self.size.0 / 2.0, self.center.1 + self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity, min_size),
                    QuadTreeNode::new((self.center.0 + self.size.0 / 2.0, self.center.1 - self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity, min_size),
                    QuadTreeNode::new((self.center.0 + self.size.0 / 2.0, self.center.1 + self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity, min_size),
                ]))
            }
        } else {
            let i = self.get_index(location);
            self.nodes.as_mut().unwrap()[i].insert(location, data, min_size);
        }
    }

    fn in_circle(&self, position: (f64, f64), radius: f64, data_vec: &mut Vec<Data>, in_circle: bool) {
        if in_circle || (self.data.len() > 4 && self.in_box(position, radius)) {
            data_vec.reserve(self.data.len());

            data_vec.extend(self.data.iter().map(|x| x.1));

            if self.nodes.is_none() {
                return;
            }

            self.nodes.as_ref().unwrap()[0].in_circle(position, radius, data_vec, true);
            self.nodes.as_ref().unwrap()[1].in_circle(position, radius, data_vec, true);
            self.nodes.as_ref().unwrap()[2].in_circle(position, radius, data_vec, true);
            self.nodes.as_ref().unwrap()[3].in_circle(position, radius, data_vec, true);
        } else {
            for data in &self.data {
                if in_range(data.0, position, radius) {
                    data_vec.push(data.1)
                }
            }

            if self.nodes.is_none() {
                return;
            }

            let mut indexes = [false; 4];

            indexes[self.get_index((position.0 - radius, position.1 - radius))] = true;
            indexes[self.get_index((position.0 - radius, position.1 + radius))] = true;
            indexes[self.get_index((position.0 + radius, position.1 - radius))] = true;
            indexes[self.get_index((position.0 + radius, position.1 + radius))] = true;

            for (i, bool) in indexes.iter().enumerate() {
                if *bool {
                    self.nodes.as_ref().unwrap()[i].in_circle(position, radius, data_vec, false);
                }
            }
        }
    }

    fn in_box(&self, location: (f64, f64), radius: f64) -> bool {
        if in_range(location, (self.center.0 - self.size.0, self.center.1 + self.size.1), radius)
            && in_range(location, (self.center.0 + self.size.0, self.center.1 + self.size.1), radius)
            && in_range(location, (self.center.0 - self.size.0, self.center.1 - self.size.1), radius)
            && in_range(location, (self.center.0 + self.size.0, self.center.1 - self.size.1), radius) {
            return true;
        }

        false
    }

    fn get_index(&self, location: (f64, f64)) -> usize {
        if self.center.0 > location.0 {
            if self.center.1 > location.1 {
                0
            } else {
                1
            }
        } else if self.center.1 > location.1 {
            2
        } else {
            3
        }
    }
}