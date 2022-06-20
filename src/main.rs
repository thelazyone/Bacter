mod cell;
use cell::Bacter;
use cell::Cell;
use nannou::prelude::*;


fn main() {
    // Showing them on a plot:
    nannou::app(model).update(update).simple_window(view).size(500, 500).run();
}

struct Model {
    bacters: Vec<Bacter>,
    iter_no: i32,
    boundary: cell::Float2D,
    duration_counter: f64,
}


fn model(_app: &App) -> Model {
    let mut curr_model = Model {
        bacters: vec![],
        iter_no: 0,
        boundary: cell::Float2D{x:0., y:0.},
        duration_counter: 0.,
    };

    let boundary = _app.window_rect();
    curr_model.boundary = cell::Float2D{x: boundary.w() as f64, y: boundary.h() as f64};
    for idx in 0..100 {
        curr_model.bacters.push(
            cell::Bacter::new_random(curr_model.boundary, idx));
    };
    curr_model
}



fn update(app: &App, model: &mut Model, _update: Update) {

    for _ in 0..5{
        let curr_bodies = model.bacters.to_owned();

        // Interacting with the walls
        for i in 0..model.bacters.len() {
            model.bacters[i].bounce_with_box(model.boundary);
        }
        
        // interacting with the other cells
        for i in 0..model.bacters.len() {
            // Todo make an interact_with_cells which calls bounce_with_cells
            // and any other interaction, including fight and eating 
            model.bacters[i].bounce_with_cells(&curr_bodies);
        }

        // Checking if the cells are still alive, their food resources and reproduction
        // TODO

        // Applying the movement after all checks.
        for i in 0..model.bacters.len() {
            model.bacters[i].apply_movement(1.);
        }
    }
    // Add bacters logic here
}


// Nannou Stuff
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // modelduration_counter
    // app.duration.since.as_secs_f64()

    // set background to blue
    draw.background().color(GREEN);

    // Drawing them on screen!
    for bacter in &model.bacters{
        draw.ellipse().color(STEELBLUE)
        .x_y(
            bacter.get_vector().pos.x as f32, 
            bacter.get_vector().pos.y as f32)
        .radius(5.);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
