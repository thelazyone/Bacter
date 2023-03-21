pub mod cell;
pub mod collision_grid;
pub mod petri;

// WASM Stuff:
pub mod wasm_utilities;
use std::time::Instant;


fn main() {
    // Creating a dish
    let mut universe = petri::Petri::new_with_params(2048, 2048, 1000);

    for counter in 0..1000000 {
        universe.tick(1);
        if counter % 1000 == 0 {
            // Clearing the screen and putting the cursor at top.
            // Then, writing statistics and plotting histogram
            println!("iterations {} reached", counter);
            println!("{} bacters, {} algae", universe.get_bacters_number(), universe.get_algae_number());

            // Retrieving statistics
            let aggros = universe.get_all_bacters_aggros_vector().clone();
            let sizes = universe.get_all_bacters_sizes_vector().clone();
            let mut histogram = vec![vec![0; 10]; 10];
            for (a, s) in aggros.iter().zip(sizes) {
                histogram
                    [((a*10.).floor() as usize).clamp( 0, 9)]
                    [((s*10.).floor() as usize).clamp( 0, 9)]
                    += 1;
            }

            // Forming the 2D histogram.
            let mut histogram_string = "".to_string();
            for a_idx in 0..10 {
                for s_idx in 0..10 {
                    histogram_string.push(match histogram[a_idx][s_idx] {
                        0 => ' ',
                        n if n < 10 => '.',
                        n if n < 50 => 'o',
                        _ => '@',
                    });
                }
                histogram_string.push('\n');
            }
            println!("{}", histogram_string);
        }
    }
}
