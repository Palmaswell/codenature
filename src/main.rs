use nannou::prelude::*;
mod mover;

use mover::Mover;

struct Model {
    mover: Mover,
}

fn main() {
    println!("Hello, world!");
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1024, 1024)
        .run();
}

fn model(app: &App) -> Model {
    let boundary = app.window_rect();
    let location = Vec2::new(
        random_range::<f32>(boundary.left(), boundary.right()),
        random_range::<f32>(boundary.bottom(), boundary.top()),
    );
    let velocity_range = random_range::<f32>(-2.0, 2.0);
    let velocity = Vec2::new(velocity_range, velocity_range);

    Model {
        mover: Mover::new(location, velocity),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let velocity_range = random_range::<f32>(-2.0, 2.0);
    let velocity = Vec2::new(velocity_range, velocity_range);
    let _ = model.mover.update(velocity);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse()
        .color(STEELBLUE)
        .x_y(model.mover.location.x, model.mover.location.y);
    draw.to_frame(app, &frame).unwrap();
}
