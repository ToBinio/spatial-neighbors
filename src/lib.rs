extern crate core;

pub struct SpatialHash<Data: Clone> {
    cells: Vec<Vec<((f64, f64), Data)>>,

    x_count: u32,
    y_count: u32,
    x: (f64, f64),
    y: (f64, f64),

    count: usize,
}

impl<Data: Clone> SpatialHash<Data> {
    pub fn new(x: (f64, f64), y: (f64, f64), x_count: u32, y_count: u32) -> SpatialHash<Data> {
        let mut cells = Vec::new();

        for i in 0..(x_count * y_count) {
            cells.insert(i as usize, Vec::new());
        };

        SpatialHash {
            cells,
            x_count,
            y_count,
            x,
            y,
            count: 0,
        }
    }

    fn pos_to_index(&self, position: (f64, f64)) -> (u32, u32) {
        let x = (((position.0 - self.x.0) as f32 / (self.x.1 - self.x.0) as f32) * self.x_count as f32).floor() as u32;
        let y = (((position.1 - self.y.0) as f32 / (self.y.1 - self.y.0) as f32) * self.y_count as f32).floor() as u32;

        return (x, y);
    }
}

impl<Data: Clone> SpatialPartitioner<Data> for SpatialHash<Data> {
    fn insert(&mut self, position: (f64, f64), data: Data) {
        if position.0 < self.x.0 || position.0 >= self.x.1 || position.1 < self.y.0 || position.1 >= self.y.1 {
            //todo better error message
            panic!("tried to insert position with was out of bounce")
        }

        let index_position = self.pos_to_index(position);

        self.cells.get_mut((index_position.0 + (index_position.1 * self.x_count)) as usize).unwrap().push((position, data));
        self.count += 1;
    }

    fn clear(&mut self) {
        self.count = 0;
        self.cells.iter_mut().for_each(|cell| cell.clear())
    }

    fn count(&self) -> usize {
        return self.count;
    }

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data> {
        let radius_x = (radius / ((self.x.1 - self.x.0) as f64 / self.x_count as f64)).ceil().max(self.x_count as f64) as i32;
        let radius_y = (radius / ((self.y.1 - self.y.0) as f64 / self.y_count as f64)).ceil().max(self.y_count as f64) as i32;

        let index_position = self.pos_to_index(position);

        let mut data = Vec::new();

        for x in -radius_x..(radius_x + 1) {
            for y in -radius_y..(radius_y + 1) {
                let x = index_position.0 as i32 - x;
                let y = index_position.1 as i32 - y;

                if x < 0 || x >= self.x_count as i32 || y < 0 || y >= self.y_count as i32 {
                    continue;
                }

                let index = x + (y * self.x_count as i32);

                match self.cells.get(index as usize) {
                    None => {}
                    Some(elements) => {
                        for element in elements {
                            if (element.0.0 - position.0).powi(2) + (element.0.1 - position.1).powi(2) <= radius.powi(2) {
                                data.push(element.1.clone());
                            }
                        }
                    }
                };
            }
        }

        data
    }
}

pub trait SpatialPartitioner<Data: Clone> {
    fn insert(&mut self, position: (f64, f64), data: Data);
    fn clear(&mut self);
    fn count(&self) -> usize;
    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data>;
}