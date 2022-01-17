use nannou::prelude::*;
mod mover;
pub use mover::Mover;

fn main() {
    println!("Hello, world!");
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1024, 1024)
        .run();
}

struct Model {
    mover: Mover,
}

fn model(app: &App) -> Model {
    let boundary = app.window_rect();
    let location = Vec2::new(
        random_range(boundary.left(), boundary.right()),
        random_range(boundary.bottom(), boundary.top()),
    );
    // TODO: replace velocity with Perlin Noise
    let velocity = Vec2::new(random_range(-2.0, 2.0), random_range(-2.0, 2.0));

    Model {
        mover: Mover::new(location, velocity),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let mover = &model.mover;

    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).xy(mover.location);

    draw.to_frame(app, &frame).unwrap();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let velocity = Vec2::new(random_range(-1.0, 2.0), random_range(-2.0, 2.0));
    let Model { mover, .. } = model;
    model.mover = mover.update(velocity);
}
