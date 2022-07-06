mod cell;
use cell::cell::Cell;
use nannou::prelude::*;

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Showing them on a plot:
    nannou::app(model).update(update).simple_window(view).size(500, 500).run();
}

struct Model {
    dish : cell::dish::Dish,
}


fn model(_app: &App) -> Model {
    let boundary = _app.window_rect();
    let curr_model = Model {
        dish : cell::dish::Dish::new(
            cell::cell::Float2D{x: boundary.w()  as f64* 2., y: boundary.h() as f64* 2.},
            100)
    };
    curr_model
}


fn update(app: &App, model: &mut Model, _update: Update) {
    for _ in 0..1000 { 
        model.dish.simulation_step();
    }
}


// Nannou Stuff
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // modelduration_counter
    // app.duration.since.as_secs_f64()

    // set background to blue
    draw.background().color(LIGHTGREEN);

    // OPTION 1: drawing cells on screen

    // // Drawing bacters on screen!
    // for bacter in &model.dish.bacters{
    //     draw.ellipse().color(STEELBLUE)
    //     .x_y(
    //         bacter.get_vector().pos.x as f32, 
    //         bacter.get_vector().pos.y as f32)
    //     .radius(10. * bacter.get_size()).resolution(8.);
    // }

    // // Drawing algae on screen
    // for alga in &model.dish.algae {
    //     draw.ellipse().color(DARKGREEN)
    //     .x_y(
    //         alga.get_vector().pos.x as f32, 
    //         alga.get_vector().pos.y as f32)
    //     .radius(10. * alga.get_size()).resolution(8.);
    // }

    // Option 2: plotting on the parameters space
    let boundary = app.window_rect();
    for bacter in &model.dish.bacters{
        draw.rect().color(RED)
        .x_y(
            (bacter.get_aggro() - 0.5) * boundary.w() as f32, 
            (bacter.get_size() - 0.5) * boundary.w() as f32)
        .h(2.)
        .w(2.);
    }
    println!("Iteration {}: there are {} bacters and {} algae",
        model.dish.get_iteration(),
        model.dish.bacters.len(),
        model.dish.algae.len());

    //draw.text(&app.duration.since_prev_update.as_millis().to_string().to_owned());
    //draw.text(&model.dish.bacters.len().to_string().to_owned());

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
