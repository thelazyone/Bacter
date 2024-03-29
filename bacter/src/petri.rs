use crate::cell::*;
use crate::cell::cell::Cell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

// Linear data struct for WASM. It has the lifetime of the Petri object
// so it remains reserved.
pub struct WasmLinearDataStruct {
    pub aggros_vec: Vec<f32>,
    pub sizes_vec: Vec<f32>,
    pub bacters_positions_vec: Vec<f32>,
    pub algae_positions_vec: Vec<f32>,
}

// Low-effort stats for the simulation
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Statistics {
    iterations: u32,
    bacters_number: u32,
    algae_number: u32,
}

// Entry point for the simulator. The Petri dish is where all the action occurrs
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Petri {
    statistics: Statistics,
    dish: dish::Dish,
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

// Non-WASM methods (used by the part after)
impl Petri{
    pub fn get_all_bacters_aggros_vector(&mut self) -> &Vec<f32> {
        self.params.aggros_vec.resize(self.dish.bacters_swap_a.len(), 0.);
        for i in 0..self.dish.bacters_swap_a.len(){
            self.params.aggros_vec[i] = self.dish.bacters_swap_a[i].get_aggro();
        }

        &self.params.aggros_vec
    }

    pub fn get_all_bacters_sizes_vector(&mut self) -> &Vec<f32> {
        self.params.sizes_vec.resize(self.dish.bacters_swap_a.len(), 0.);
        for i in 0..self.dish.bacters_swap_a.len(){
            self.params.sizes_vec[i] = self.dish.bacters_swap_a[i].get_size();
        }

        &self.params.sizes_vec
    }
}

// Public methods to be binded into WASM
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Petri{

    // Default Constructor
    pub fn new() -> Petri {
        Petri{
            dish:dish::Dish::new(cell::Float2D{x: 500  as f64* 2., y: 500 as f64* 2.}, 100),
            statistics: Statistics{iterations: 0, bacters_number: 0, algae_number: 0},
            params: WasmLinearDataStruct{
                aggros_vec: vec![0.],
                sizes_vec: vec![0.],
                bacters_positions_vec: vec![0.],
                algae_positions_vec: vec![0.]}}
    }

    // Default Constructor
    pub fn new_with_params(w: usize, h: usize, number: usize) -> Petri {
        Petri{
            dish:dish::Dish::new(cell::Float2D{x: w  as f64* 2., y: h as f64* 2.}, number as i64),
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
            self.statistics.bacters_number = self.dish.bacters_swap_a.len() as u32;
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

    // TODO temp implementation, should be using the right one! (or maybe we'll use Swap)
    pub fn get_bacters_number(&self) -> u32{
        self.dish.bacters_swap_a.len() as u32
    }

    pub fn get_algae_number(&self) -> u32{
        self.dish.algae.len() as u32
    }



    // Filling the WASM-dedicated linear vectors with floats, and exposing the pointer.
    // As long as the software is synchronous this should be safe as is.

    // Aggros linear vector
    pub fn get_all_bacters_aggros(&mut self) -> *const f32{
        self.get_all_bacters_aggros_vector().as_ptr()
    }

    // Sizes linear vector
    pub fn get_all_bacters_sizes(&mut self) -> *const f32{
        self.get_all_bacters_sizes_vector().as_ptr()

    }

    // Positions (interlaced x-y) linear vector
    pub fn get_all_bacters_position_interlaced(&mut self) -> *const f32{
        self.params.bacters_positions_vec.resize(self.dish.bacters_swap_a.len()*2, 0.);
        for i in 0..self.dish.bacters_swap_a.len(){
            self.params.bacters_positions_vec[i*2] = self.dish.bacters_swap_a[i].get_vector().pos.x as f32;
            self.params.bacters_positions_vec[i*2 + 1] = self.dish.bacters_swap_a[i].get_vector().pos.y as f32;
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