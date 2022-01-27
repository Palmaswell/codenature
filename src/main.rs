use nannou::prelude::*;
mod mover;
pub use mover::Mover;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1024, 1024)
        .run();
}

struct Model {
    mover: Mover,
}

fn model(_app: &App) -> Model {
    let location = Vec2::new(0.0, 0.0);
    let velocity = Vec2::new(0.0, 0.0);
    let acceleration = Vec2::new(-0.001, 0.01);
    Model {
        mover: Mover::new(location, velocity, acceleration),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let mover = &model.mover;

    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).xy(mover.location());

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();

    let Model { mover, .. } = model;
    model.mover = mover.update(&boundary);
}
