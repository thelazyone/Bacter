use crate::cell::cell;

// Implementing the space where the cells interact
pub struct Dish {
    pub bacters: Vec<cell::Bacter>, // Temporarly pubilc - a getter is unnecessary now
    iter_no: i32,
    boundary: cell::Float2D,
    duration_counter: f64,
    cells_counter: i64,
}

impl Dish{
    pub fn new(i_bound_rect : cell::Float2D, i_cells_number : i64) -> Dish {
        let mut curr_model = Dish {
            bacters: vec![],
            iter_no: 0,
            boundary: i_bound_rect,
            duration_counter: 0.,
            cells_counter: 0,
        };
    
        for idx in 0..i_cells_number {
            curr_model.bacters.push(
                cell::Bacter::new_random(curr_model.boundary, idx));
        };
        curr_model.cells_counter = i_cells_number;
        curr_model
    }


    pub fn interact(&mut self){
        // TODO - Every single time there's a copy here which is HIGHLY inefficient.
        // However this is a problem of the second order.
        let curr_bodies = self.bacters.to_owned();

        // Interacting with the walls
        for i in 0..self.bacters.len() {
            self.bacters[i].bounce_with_box(self.boundary);
        }
        
        // interacting with the other cells
        for i in 0..self.bacters.len() {
            // Todo make an interact_with_cells which calls bounce_with_cells
            // and any other interaction, including fight and eating 
            if let Some(target_idx) = self.bacters[i].bounce_with_cells(&curr_bodies){
                // If interaction happened, proceeding with confrontation, eating and so forth.
                println!("Trying a kill ({} on {})", i, target_idx);
                if self.bacters[i].try_kill_bacter(curr_bodies[target_idx]){
                    self.bacters[target_idx].kill();
                    println!("Bacter {} killed bacter {}", i, target_idx);
                }
                else
                {
                    println!("try kill returned false!");
                }
            }
        }
    
        // Applying the movement after all checks.
        for i in 0..self.bacters.len() {
            self.bacters[i].apply_movement(0.25); // TODO - remove the MAGIC NUMBER
        }

        // Consuming food
        for i in 0..self.bacters.len() {
            // TODO : the time is not the same of the movements! MAGIC NUMBER
            self.bacters[i].consume_food(0.0001); 
        }

        // Reproducing if necessary!
        for i in 0..self.bacters.len() {
            // TODO : the time is not the same of the movements! MAGIC NUMBER
            if let Some(mut offspring) =self.bacters[i].try_reproducing(){
                offspring.set_index(self.cells_counter as i64);
                self.cells_counter += 1;
                self.bacters.push(offspring);

            println!("new Generation for bacter {}! There are {} bacters", i, self.bacters.len());
            }
        }

        // !! IMPORTANT !! This operation must be done last, becaue the order of the elements is not kept!
        // Checking if the cells are still alive, their food resources and reproduction
        for i in (0..self.bacters.len()).rev() {
            if !self.bacters[i].is_alive(){
                self.bacters.swap_remove(i);
            }
        }

        // Alternative - experimental!         
        // self.bacters.drain_filter(|elem| {
        //     !elem.is_alive()
        // });

    }
}

