mod cell;
use crate::cell::cell::Cell; // for the Bacter Cell trait.

// WASM Stuff:
mod wasm_utilities;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Linear data struct for WASM. It has the lifetime of the Petri object
// so it remains reserved.
pub struct WasmLinearDataStruct {
    pub aggros_vec: Vec<f32>,
    pub sizes_vec: Vec<f32>,
    pub bacters_positions_vec: Vec<f32>,
    pub algae_positions_vec: Vec<f32>,
}

// Low-effort stats for the simulation
#[wasm_bindgen]
pub struct Statistics {
    iterations: u32,
    bacters_number: u32,
    algae_number: u32,
}

// Entry point for the simulator. The Petri dish is where all the action occurrs
#[wasm_bindgen]
pub struct Petri {
    statistics: Statistics,
    dish: cell::dish::Dish,
    params: WasmLinearDataStruct,
}

use std::fmt;
impl fmt::Display for Petri{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iteration {}: {} cells, {} algae.", 
            self.statistics.iterations, 
            self.statistics.bacters_number, 
            self.statistics.algae_number)
    }
}

// Public methods to be binded into WASM
#[wasm_bindgen]
impl Petri{

    // Default Constructor
    pub fn new() -> Petri {
        Petri{
            dish:cell::dish::Dish::new(cell::cell::Float2D{x: 500  as f64* 2., y: 500 as f64* 2.}, 100),
            statistics: Statistics{iterations: 0, bacters_number: 0, algae_number: 0},
            params: WasmLinearDataStruct{
                aggros_vec: vec![0.], 
                sizes_vec: vec![0.],
                bacters_positions_vec: vec![0.],
                algae_positions_vec: vec![0.]}}
    }

    // Pushes the simulation forward.
    // Right now each steps are 1000 ticks, should do parametric
    pub fn tick(&mut self, steps: u32) {
        for _ in 0..steps { 
        self.dish.simulation_step();
        self.statistics.iterations = self.dish.get_iteration() as u32;
        self.statistics.bacters_number = self.dish.bacters.len() as u32;
        self.statistics.algae_number = self.dish.algae.len() as u32;
        }
    }

    // Get methods for basic statistics.
    pub fn get_stats_string(&self) -> String {
        self.to_string()
    }

    pub fn get_iteration(&self) -> u32{
        self.dish.get_iteration() as u32
    }
    
    pub fn get_bacters_number(&self) -> u32{
       self.dish.bacters.len() as u32
    }
    
    pub fn get_algae_number(&self) -> u32{
        self.dish.algae.len() as u32
    }

    // Filling the WASM-dedicated linear vectors with floats, and exposing the pointer.
    // As long as the software is synchronous this should be safe as is.

    // Aggros linear vector
    pub fn get_all_bacters_aggros(&mut self) -> *const f32{
        self.params.aggros_vec.resize(self.dish.bacters.len(), 0.);
        for i in 0..self.dish.bacters.len(){
            self.params.aggros_vec[i] = self.dish.bacters[i].get_aggro();
        }
        self.params.aggros_vec.as_ptr()
    }

    // Sizes linear vector
    pub fn get_all_bacters_sizes(&mut self) -> *const f32{
        self.params.sizes_vec.resize(self.dish.bacters.len(), 0.);
        for i in 0..self.dish.bacters.len(){
            self.params.sizes_vec[i] = self.dish.bacters[i].get_size();
        }
        self.params.sizes_vec.as_ptr()
    }

    // Positions (interlaced x-y) linear vector
    pub fn get_all_bacters_position_interlaced(&mut self) -> *const f32{
        self.params.bacters_positions_vec.resize(self.dish.bacters.len()*2, 0.);
        for i in 0..self.dish.bacters.len(){
            self.params.bacters_positions_vec[i*2] = self.dish.bacters[i].get_vector().pos.x as f32;
            self.params.bacters_positions_vec[i*2 + 1] = self.dish.bacters[i].get_vector().pos.y as f32;
        }
        self.params.bacters_positions_vec.as_ptr()
    }

    // Positions (interlaced x-y) linear vector
    pub fn get_all_algae_position_interlaced(&mut self) -> *const f32{
        self.params.algae_positions_vec.resize(self.dish.algae.len()*2, 0.);
        for i in 0..self.dish.algae.len(){
            self.params.algae_positions_vec[i*2] = self.dish.algae[i].get_vector().pos.x as f32;
            self.params.algae_positions_vec[i*2 + 1] = self.dish.algae[i].get_vector().pos.y as f32;
        }
        self.params.algae_positions_vec.as_ptr()
    }

}