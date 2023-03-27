use std::cmp;

pub struct CollisionGrid {

    // The indices of the input elements are stored in a vector of the same size, and through
    // the indices of the cells_bound vector you can easily access which indices are neighbours
    ordered_indices : Vec<usize>,
    cells_bounds : Vec<usize>,

    cell_size : (f32, f32),
    area_size : (f32, f32),
    cells_number : (usize, usize),
}


impl CollisionGrid {
    pub fn new(cell_size : (f32, f32), area_size : (f32, f32)) -> CollisionGrid {

        // Sanity check:
        if cell_size.0 <= 0. || cell_size.1 <= 0. ||
            area_size.0 <= 0. || area_size.1 <= 0. {
            panic!("wrong inputs in the Collision Grid constructor!")
        }

        let cells_number = ((area_size.0 / cell_size.0).ceil() as usize,
                            (area_size.1 / cell_size.1).ceil() as usize, );

        CollisionGrid {
            ordered_indices : Vec::<usize>::new(),
            cells_bounds: vec![0; cells_number.0 * cells_number.1],
            cell_size,
            area_size,
            cells_number,
        }
    }


    pub fn set_points(&mut self, points : &Vec<(f32, f32)>) {

        // use std::time::Instant;
        // let now = Instant::now();

        // Resizing the container:
        // TODO this could be done in a faster way!
        self.ordered_indices.resize(points.len(), 0); // TBR

        // First preparing the cells bounds:
        self.cells_bounds = vec![0; self.cells_number.0 * self.cells_number.1]; // TBR
        points.iter().for_each(|point| {
            let point_index = self.get_cell_index_from_point(point);
            self.cells_bounds[point_index] += 1;
        });

        // Cumulative sum performed roughly. Not idiomatic but it's such a quick operation.

        for i in 1..self.cells_bounds.len() {
            self.cells_bounds[i] += self.cells_bounds[i-1];
        }

        // Then filling the ordered indices for each element.
        let mut added_cells_counters = vec![0; self.cells_number.0 * self.cells_number.1];
        for (point_index, point) in points.iter().enumerate() {
            let cell_of_point = self.get_cell_index_from_point(point);
            let starting_index = self.get_cell_start(cell_of_point);
            let target_index = starting_index + added_cells_counters[cell_of_point];
            self.ordered_indices[target_index] = point_index;
            added_cells_counters[cell_of_point] += 1;
        }

        // let elapsed = now.elapsed();
        // println!("set_points - elapsed: {:.2?} for {} points", elapsed, points.len());
    }

    pub fn get_cell_coordinates_from_point(&self, point : &(f32, f32)) -> (usize, usize) {
        let temp_pos =
            (((point.0 + self.area_size.0 / 2.) / self.cell_size.0).floor() as usize,
            ((point.1 + self.area_size.1 / 2.) / self.cell_size.1).floor() as usize);
        //println!("point {:?} to coords {:?}", point, temp_pos);
        (cmp::max(0,cmp::min(temp_pos.0, self.cells_number.0 - 1)),
         cmp::max(0,cmp::min(temp_pos.1, self.cells_number.1 - 1)))

    }

    pub fn get_cell_index_from_point(&self, point : &(f32, f32)) -> usize {
        self.get_cell_index_from_coordinates(self.get_cell_coordinates_from_point(point))
    }

    pub fn get_neighbourhood(&self, cell_index : usize)-> Vec<usize> {
        let cell_coords = self.get_cell_coordinates_from_index(cell_index);
        let mut out_vec = self.get_cell_content(cell_index).to_vec();
        for index_w in cell_coords.0.saturating_sub(1)..
            cmp::min(cell_coords.0 + 1, self.cells_number.0 - 1) {

            for index_h in cell_coords.1.saturating_sub(1)..
                cmp::min(cell_coords.1 + 1, self.cells_number.1 - 1) {

                let neighbour_index = self.get_cell_index_from_coordinates((index_w, index_h));
                out_vec.append(&mut self.get_cell_content(neighbour_index).to_vec())
            }
        }

        out_vec
    }

    pub fn get_total_cells_number(&self) -> usize {
        self.cells_bounds.len()
    }

    pub fn get_cell_content(&self, cell_index : usize) -> &[usize] {
        if cell_index == 0 {
            return &self.ordered_indices[0..self.cells_bounds[1]];
        }
        &self.ordered_indices[self.cells_bounds[cell_index - 1]..self.cells_bounds[cell_index]]
    }

    // Privates:

    fn get_cell_index_from_coordinates(&self, cell_coordinates : (usize, usize)) -> usize {
        self.cells_number.0 * cell_coordinates.1 + cell_coordinates.0
    }


    fn get_cell_coordinates_from_index(&self, cell_index : usize) -> (usize, usize) {
        (cell_index % self.cells_number.0, cell_index / self.cells_number.0)
    }


    fn get_cell_start(&self, cell_index : usize) -> usize {
       if cell_index == 0 {
           return 0;
       }
       self.cells_bounds[cell_index - 1]
    }


}