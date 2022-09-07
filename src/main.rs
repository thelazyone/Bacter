mod cell;
use std::time::{Instant};

use std::env;

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
