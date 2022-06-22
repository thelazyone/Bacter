mod cell;
use cell::cell::Cell;
use nannou::prelude::*;

// For Threads:
use std::thread;
use std::time::Duration;

fn main() {
    // // // // Preparing the environment.

    // // // // Starting the simulation.
    // // // thread::spawn(|| {
    // // //     for i in 1..10 {
    // // //         println!("hi number {} from the spawned thread!", i);
    // // //         thread::sleep(Duration::from_millis(1));
    // // //     }
    // // // });

    // Showing them on a plot:
    nannou::app(model).update(update).simple_window(view).size(500, 500).run();
}

struct Model {
    dish : cell::dish::Dish,
}


fn model(_app: &App) -> Model {
    let boundary = _app.window_rect();
    let mut curr_model = Model {
        dish : cell::dish::Dish::new(cell::cell::Float2D{x: boundary.w() as f64, y: boundary.h() as f64}, 100)
    };
    curr_model
}


fn update(app: &App, model: &mut Model, _update: Update) {
   model.dish.interact();
}


// Nannou Stuff
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // modelduration_counter
    // app.duration.since.as_secs_f64()

    // set background to blue
    draw.background().color(GREEN);

    // Drawing them on screen!
    for bacter in &model.dish.bacters{
        draw.ellipse().color(STEELBLUE)
        .x_y(
            bacter.get_vector().pos.x as f32, 
            bacter.get_vector().pos.y as f32)
        .radius(5.);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
