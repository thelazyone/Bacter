use crate::cell::cell;
use rand::Rng;
use crate::cell::cell::Bacter;


// MAIN TODOS
// * Setup a module for parameters and settings (reading from .ini file?)
// * Use a single random generator - right now there are far too many


pub enum SwapState {
    A,
    B
}

// Implementing the space where the cells interact
pub struct Dish {

    // Swapping logic for the buffers, to prevent unnecessary copying.
    bacters_swap_a: Vec<Bacter>, // TODO consider making this private?
    bacters_swap_b: Vec<Bacter>,
    pub swap_counter : SwapState,
    pub algae: Vec<cell::Alga>,

    // Internal Components
    iter_no: i32,
    boundary: cell::Float2D,
    _duration_counter: f64,
    cells_counter: i64,
    algae_counter: i64,
}


impl Dish{
    pub fn new(i_bound_rect : cell::Float2D, i_cells_number : i64) -> Dish {
        let mut curr_model = Dish {
            bacters_swap_a: vec![],
            bacters_swap_b: vec![],
            swap_counter: SwapState::A,
            algae: vec![],
            iter_no: 0,
            boundary: i_bound_rect,
            _duration_counter: 0.,
            cells_counter: 0,
            algae_counter: 0,
        };
    
        for idx in 0..i_cells_number {
            curr_model.bacters_swap_a.push(
                Bacter::new_random(curr_model.boundary, idx));
            curr_model.bacters_swap_b = curr_model.bacters_swap_a.clone();
        };
        curr_model.cells_counter = i_cells_number;
        curr_model
    }


    pub fn simulation_step(&mut self){

        self.iter_no += 1;

        // Adding Algae to the vector.
        self.grow_algae();

        // Implementing a swap buffer. It is now useless, but it will become
        // useful when working with the GPU.
        let curr_iteration :&mut Vec<Bacter>;
        let prev_iteration :&Vec<Bacter>;
        match self.swap_counter {
            SwapState::A => {
                curr_iteration = & mut self.bacters_swap_a;
                prev_iteration = &self.bacters_swap_b;
            },
            _ => {
                curr_iteration = & mut self.bacters_swap_b;
                prev_iteration = &self.bacters_swap_a;
            },
        }

        // The cloning is necessary, even though the size is similar.
        //*curr_iteration = prev_iteration.to_owned();

        // Resizing and reassigning. It's basically the same of copying.
        curr_iteration.resize(prev_iteration.len(), Bacter::new_stub());
        for i in 0..prev_iteration.len() {
            curr_iteration[i].copy(&prev_iteration[i]);
        }

        // interacting with the other cells. Only elements of "current" will be updated!
        for i in 0..prev_iteration.len() {

            // TODO make an interact_with_cells which calls bounce_with_cells
            // and any other interaction, including fight and eating
            if let Some(target_idx) = prev_iteration[i].bounce_with_cells(&mut curr_iteration[i], &prev_iteration){
                // If interaction happened, proceeding with confrontation, eating and so forth.
                if curr_iteration[i].try_kill_bacter(&prev_iteration[target_idx]){
                    curr_iteration[target_idx].kill();
                }
            }

            // Interacting with the algae
            if let Some(target_idx) = prev_iteration[i].bounce_with_cells(&mut curr_iteration[i], &self.algae){
                // If interaction happened, proceeding with confrontation, eating and so forth.
                //println!("Trying to eat alga ({} on {})", i, target_idx);
                if curr_iteration[i].try_eat_alga( self.algae[target_idx]){
                    self.algae[target_idx].kill();
                }
            }
        }

        // Interacting with the walls
        for i in 0..curr_iteration.len() {
            curr_iteration[i].bounce_with_box(self.boundary);
        }

        // Applying the movement after all checks.
        for i in 0..curr_iteration.len() {
            curr_iteration[i].apply_movement(0.25); // TODO - remove the MAGIC NUMBER
        }

        // Consuming food
        for i in 0..curr_iteration.len() {
            // TODO : the time is not the same of the movements! MAGIC NUMBER
            curr_iteration[i].consume_food(0.01);
        }

        // Reproducing if necessary!
        for i in 0..curr_iteration.len() {
            // TODO : the time is not the same of the movements! MAGIC NUMBER
            if let Some(mut offspring) =curr_iteration[i].try_reproducing(){
                offspring.set_index(self.cells_counter as i64);
                self.cells_counter += 1;
                curr_iteration.push(offspring);

            //println!("new Generation for bacter {}! There are {} curr_iteration", i, curr_iteration.bacters.len());
            }
        }

        // !! IMPORTANT !! This operation must be done last, because the order of the elements is not kept!
        // Checking if the cells and algae are still alive, their food resources and reproduction
        for i in (0..curr_iteration.len()).rev() {
            if !curr_iteration[i].is_alive(){
                curr_iteration.swap_remove(i);
            }
        }
        for i in (0..self.algae.len()).rev() {
            if !self.algae[i].is_alive(){
                self.algae.swap_remove(i);
            }
        }

        // Finally swapping between the two!
        // TODO i'm sure there's a smarter way to do this!
        match self.swap_counter {
            SwapState::A => {
                self.swap_counter = SwapState::B;
                self.bacters_swap_b = self.bacters_swap_a.clone();
            },
            SwapState::B => {
                self.swap_counter = SwapState::A;
                self.bacters_swap_a = self.bacters_swap_b.clone();
            },
        };
    }


    pub fn get_iteration(&self) -> i32 {
        self.iter_no
    }

    // Private Methods

    fn grow_algae(&mut self)
    {
        // growing chance: 10% // TODO FIX MAGIC NUMBER
        let mut rng = rand::thread_rng(); // TODO handle the random without that many threads!
        if rng.gen::<f64>() < 0.05 && self.algae.len () < 5000 {
            self.algae.push(
                cell::Alga::new_random(self.boundary, self.algae_counter));
            self.algae_counter += 1;
        }
    }

}

