use std::ops::Range;
use crate::SpatialPartitioner;
use crate::util::in_range;

pub struct Grid<Data: Copy> {
    cells: Vec<Vec<((f64, f64), Data)>>,

    cell_count: (usize, usize),
    cell_size: (f64, f64),

    x: Range<f64>,
    y: Range<f64>,

    count: usize,
}

impl<Data: Copy> Grid<Data> {
    ///
    /// # Arguments
    ///
    /// * `x`: min_x..max_x defines the area in wich data can be inserted
    /// * `y`: min_y..max_y defines the area in wich data can be inserted
    /// * `cell_count`: (count_x, count_y) defines how many cell should be present
    ///
    pub fn new(x: Range<f64>, y: Range<f64>, cell_count: (usize, usize)) -> Grid<Data> {
        let mut cells = Vec::new();

        for i in 0..(cell_count.0 * cell_count.1) {
            cells.insert(i as usize, Vec::new());
        };

        Grid {
            cells,
            cell_count,
            cell_size: ((x.end - x.start) as f64 / cell_count.0 as f64, (y.end - y.start) as f64 / cell_count.1 as f64),
            x,
            y,
            count: 0,
        }
    }

    ///
    /// # Arguments
    ///
    /// * `indexes`: (x,y) cord of the cell
    ///
    /// returns: &Vec<((f64, f64), Data), Global> the data stored in the cell
    ///
    pub fn cell(&self, indexes: (usize, usize)) -> &Vec<((f64, f64), Data)> {
        let index = indexes.0 + (indexes.1 * self.cell_count.0);

        self.cells.get(index).unwrap()
    }

    ///
    /// # Arguments
    ///
    /// returns: (usize, usize) the defined cell_count
    ///
    pub fn cell_count(&self) -> (usize, usize) {
        self.cell_count
    }

    ///
    /// # Arguments
    ///
    /// returns: (f64, f64) width and height of each cell
    ///
    pub fn cell_size(&self) -> (f64, f64) {
        self.cell_size
    }

    fn pos_to_index(&self, position: (f64, f64)) -> (usize, usize) {
        let x = ((position.0 - self.x.start) / self.cell_size.0).floor() as usize;
        let y = ((position.1 - self.y.start) / self.cell_size.1).floor() as usize;

        (x, y)
    }

    fn index_to_pos(&self, position: (usize, usize)) -> (f64, f64) {
        let x = self.cell_size.0 * position.0 as f64 + self.x.start;
        let y = self.cell_size.1 * position.1 as f64 + self.y.start;

        (x, y)
    }
}

impl<Data: Copy> SpatialPartitioner<Data> for Grid<Data> {
    ///
    /// create a Grid with a default size of (100,100). More info here [`Grid::new()`]
    ///
    fn new(x: Range<f64>, y: Range<f64>) -> Self {
        Grid::new(x, y, (100, 100))
    }

    fn insert(&mut self, position: (f64, f64), data: Data) {
        if position.0 < self.x.start || position.0 >= self.x.end || position.1 < self.y.start || position.1 >= self.y.end {
            panic!("tried to insert position into SpatialHash which was out of bounce")
        }

        self.insert_unchecked(position, data);
    }

    fn insert_unchecked(&mut self, position: (f64, f64), data: Data) {
        let index_position = self.pos_to_index(position);

        self.cells.get_mut((index_position.0 + (index_position.1 * self.cell_count.0)) as usize).unwrap().push((position, data));
        self.count += 1;
    }

    fn count(&self) -> usize {
        self.count
    }

    fn clear(&mut self) {
        self.count = 0;
        self.cells.iter_mut().for_each(|cell| cell.clear())
    }

    fn in_circle(&self, position: (f64, f64), radius: f64) -> Vec<Data> {
        let radius_x = (radius / ((self.x.end - self.x.start) as f64 / self.cell_count.0 as f64)).ceil().min(self.cell_count.0 as f64) as i32;
        let radius_y = (radius / ((self.y.end - self.y.start) as f64 / self.cell_count.1 as f64)).ceil().min(self.cell_count.1 as f64) as i32;

        let index_position = self.pos_to_index(position);

        let mut data = Vec::new();

        for x in -radius_x..(radius_x + 1) {
            for y in -radius_y..(radius_y + 1) {
                let x = index_position.0 as i32 + x;
                let y = index_position.1 as i32 + y;

                if x < 0 || x >= self.cell_count.0 as i32 || y < 0 || y >= self.cell_count.1 as i32 {
                    continue;
                }

                let index = x + (y * self.cell_count.0 as i32);

                match self.cells.get(index as usize) {
                    None => {}
                    Some(elements) => {
                        if elements.len() > 4 {
                            let pos = self.index_to_pos((x as usize, y as usize));

                            if in_range(position, (pos.0 + self.cell_size.0, pos.1), radius)
                                && in_range(position, (pos.0 + self.cell_size.0, pos.1 + self.cell_size.1), radius)
                                && in_range(position, (pos.0, pos.1), radius)
                                && in_range(position, (pos.0, pos.1 + self.cell_size.1), radius) {
                                data.extend(elements.iter().map(|x| x.1));

                                continue;
                            }
                        }

                        for element in elements {
                            if in_range(element.0, position, radius) {
                                data.push(element.1);
                            }
                        }
                    }
                };
            }
        }

        data
    }
}