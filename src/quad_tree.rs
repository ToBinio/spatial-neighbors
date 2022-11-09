use crate::SpatialPartitioner;
use crate::util::in_range;

pub struct QuadTree<Data: Copy> {
    node: QuadTreeNode<Data>,
    x: (f64, f64),
    y: (f64, f64),
    count: usize,
    capacity: u16,
}

impl<Data: Copy> QuadTree<Data> {
    pub fn new(x: (f64, f64), y: (f64, f64), capacity: u16) -> QuadTree<Data> {
        QuadTree {
            node: QuadTreeNode::new(((x.1 - x.0) / 2.0, (y.1 - y.0) / 2.0), ((x.1 - x.0), (y.1 - y.0)), capacity),
            x,
            y,
            count: 0,
            capacity,
        }
    }
}

impl<Data: Copy> SpatialPartitioner<Data> for QuadTree<Data> {
    fn insert(&mut self, position: (f64, f64), data: Data) {
        self.insert_unchecked(position, data);
    }

    fn insert_unchecked(&mut self, position: (f64, f64), data: Data) {
        self.count += 1;
        self.node.insert(position, data);
    }

    fn count(&self) -> usize {
        self.count
    }

    fn clear(&mut self) {
        self.node = QuadTreeNode::new(((self.x.1 - self.x.0) / 2.0, (self.y.1 - self.y.0) / 2.0), ((self.x.1 - self.x.0), (self.y.1 - self.y.0)), self.capacity);
        self.count = 0;
    }

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data> {
        let mut data = Vec::new();

        self.node.in_circle(position, radius, &mut data);

        data
    }
}

struct QuadTreeNode<Data: Copy> {
    data: Vec<((f64, f64), Data)>,
    capacity: u16,
    is_full: bool,

    nodes: Option<Box<[QuadTreeNode<Data>; 4]>>,

    center: (f64, f64),
    size: (f64, f64),
}

impl<Data: Copy> QuadTreeNode<Data> {
    fn new(center: (f64, f64), size: (f64, f64), capacity: u16) -> QuadTreeNode<Data> {
        QuadTreeNode {
            data: Vec::new(),
            nodes: None,
            center,
            size,
            capacity,
            is_full: false,
        }
    }

    fn insert(&mut self, location: (f64, f64), data: Data) {
        if !self.is_full {
            self.data.push((location, data));

            if self.data.len() >= self.capacity as usize {
                self.is_full = true;

                self.nodes = Some(Box::new([
                    QuadTreeNode::new((self.center.0 - self.size.0 / 2.0, self.center.1 - self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity),
                    QuadTreeNode::new((self.center.0 - self.size.0 / 2.0, self.center.1 + self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity),
                    QuadTreeNode::new((self.center.0 + self.size.0 / 2.0, self.center.1 - self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity),
                    QuadTreeNode::new((self.center.0 + self.size.0 / 2.0, self.center.1 + self.size.1 / 2.0), (self.size.0 / 2.0, self.size.1 / 2.0), self.capacity),
                ]))
            }
        } else {
            if self.center.0 > location.0 {
                if self.center.1 > location.1 {
                    self.nodes.as_mut().unwrap()[0].insert(location, data);
                } else {
                    self.nodes.as_mut().unwrap()[1].insert(location, data);
                }
            } else {
                if self.center.1 > location.1 {
                    self.nodes.as_mut().unwrap()[2].insert(location, data);
                } else {
                    self.nodes.as_mut().unwrap()[3].insert(location, data);
                }
            }
        }
    }

    fn in_circle(&self, position: (f64, f64), radius: f64, data_vec: &mut Vec<Data>) {
        for data in &self.data {
            if in_range(data.0, position, radius) {
                data_vec.push(data.1)
            }
        }

        if self.nodes.is_none() {
            return;
        }

        //todo fix

        if self.center.0 > position.0 {
            if self.center.1 > position.1 {
                self.nodes.as_ref().unwrap()[0].in_circle(position, radius, data_vec);
            } else {
                self.nodes.as_ref().unwrap()[1].in_circle(position, radius, data_vec);
            }
        } else {
            if self.center.1 > position.1 {
                self.nodes.as_ref().unwrap()[2].in_circle(position, radius, data_vec);
            } else {
                self.nodes.as_ref().unwrap()[3].in_circle(position, radius, data_vec);
            }
        }
    }
}