use crate::cell::cell;

// Implementing the space where the cells interact
pub struct Dish {
    pub bacters: Vec<cell::Bacter>, // Temporarly pubilc - a getter is unnecessary now
    iter_no: i32,
    boundary: cell::Float2D,
    duration_counter: f64,
}

impl Dish{
    pub fn new(i_bound_rect : cell::Float2D, i_cells_number : i64) -> Dish {
        let mut curr_model = Dish {
            bacters: vec![],
            iter_no: 0,
            boundary: cell::Float2D{x:0., y:0.},
            duration_counter: 0.,
        };
    
        curr_model.boundary = i_bound_rect;
        for idx in 0..i_cells_number {
            curr_model.bacters.push(
                cell::Bacter::new_random(curr_model.boundary, idx));
        };
        curr_model
    }


    pub fn interact(&mut self){
        // TODO - Every single time there's a copy here which is HIGHLY inefficient.
        let curr_bodies = self.bacters.to_owned();

        // Interacting with the walls
        for i in 0..self.bacters.len() {
            self.bacters[i].bounce_with_box(self.boundary);
        }
        
        // interacting with the other cells
        for i in 0..self.bacters.len() {
            // Todo make an interact_with_cells which calls bounce_with_cells
            // and any other interaction, including fight and eating 
            self.bacters[i].bounce_with_cells(&curr_bodies);
        }
    
        // Checking if the cells are still alive, their food resources and reproduction
        // TODO
    
        // Applying the movement after all checks.
        for i in 0..self.bacters.len() {
            self.bacters[i].apply_movement(1.);
        }
    }
}
