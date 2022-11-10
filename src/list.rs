use crate::SpatialPartitioner;
use crate::util::in_range;

pub struct List<Data: Clone> {
    data: Vec<((f64, f64), Data)>,
}

impl<Data: Clone> List<Data> {
    pub fn new() -> List<Data> {
        List {
            data: Vec::new()
        }
    }
}

impl<Data: Clone> Default for List<Data> {
    fn default() -> Self {
        Self::new()
    }
}


impl<Data: Clone> SpatialPartitioner<Data> for List<Data> {
    fn insert(&mut self, position: (f64, f64), data: Data) {
        self.insert_unchecked(position, data);
    }

    fn insert_unchecked(&mut self, position: (f64, f64), data: Data) {
        self.data.push((position, data));
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data> {
        let mut data = Vec::new();

        for element in &self.data {
            if in_range(element.0, position, radius) {
                data.push(element.1.clone());
            }
        };

        data
    }
}