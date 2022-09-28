mod cell;
use std::time::{Instant};
use std::env;

// WASM Stuff:
mod wasm_utilities;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Statistics {
    iterations: u32,
    bacters_number: u32,
    algae_number: u32,
}

#[wasm_bindgen]
pub struct Petri {
    statistics: Statistics,
    dish: cell::dish::Dish,
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

#[wasm_bindgen]
impl Petri{
    pub fn new() -> Petri {
        Petri{
            dish:cell::dish::Dish::new(cell::cell::Float2D{x: 500  as f64* 2., y: 500 as f64* 2.}, 100),
            statistics: Statistics{iterations: 0, bacters_number: 0, algae_number: 0}}
    }

    pub fn tick(&mut self) {
        for _ in 0..100 { 
        self.dish.simulation_step();
        self.statistics.iterations = self.dish.get_iteration() as u32;
        self.statistics.bacters_number = self.dish.bacters.len() as u32;
        self.statistics.algae_number = self.dish.algae.len() as u32;
            }
    }

    pub fn get_stats(&self) -> String {
        self.to_string()
    }
}

// Starting an async model that is NOT linked to any GUI environment. Then, accessing the information
// from the interface asynchronously.
fn main() {
    // This gives a stack trace when the binary hits an error
    env::set_var("RUST_BACKTRACE", "1");

    // Starting a new petri dish.
    // TODO - remove the magic numbers, of course.
    let mut dish = cell::dish::Dish::new(
            cell::cell::Float2D{x: 500  as f64* 2., y: 500 as f64* 2.},
            100);

    // Temp - for now iterating and outputting some info.
    let start = Instant::now();
    for _ in 0..100 { 
        for _ in 0..1000 { 
        dish.simulation_step();
        }
        println!("Iteration {}: there are {} bacters and {} algae.", dish.get_iteration(), dish.bacters.len(), dish.algae.len());
    }
    println!("Test run executed in {:?}", start.elapsed());
}
