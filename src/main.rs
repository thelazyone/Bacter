pub mod cell;
pub mod collision_grid;
pub mod petri;

// WASM Stuff:
pub mod wasm_utilities;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    // Creating a dish
    let mut universe = petri::Petri::new_with_params(2048, 2048, 1000);

    for counter in 0..10000 {
        universe.tick(1);
        if counter % 1000 == 0 {
            println!("iterations {} reached", counter);
            println!("{} bacters, {} algae", universe.get_bacters_number(), universe.get_algae_number());
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
