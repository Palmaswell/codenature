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
    movers: Vec<Mover>,
}

const NUM_MOVERS: usize = 10;

fn model(app: &App) -> Model {
    let mut movers: Vec<Mover> = Vec::new();
    let boundary = app.window_rect();
    for _ in 0..NUM_MOVERS {
        let location = Vec2::new(
            random_range(boundary.left(), boundary.right()),
            random_range(boundary.bottom(), boundary.top()),
        );
        let mass = random_range(1.0, 5.0);

        let mover = Mover::new(location, mass);
        movers.push(mover);
    }

    Model { movers }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);
    for mover in model.movers.iter() {
        draw.ellipse()
            .radius(mover.mass())
            .color(STEELBLUE)
            .xy(mover.location());
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut movers: Vec<Mover> = Vec::new();
    let boundary = app.window_rect();
    let wind = Vec2::new(-0.01, 0.0);
    let gravity = Vec2::new(0.0, -0.01);
    // Drag force
    // f = ||v||^ * c * v * -1
    let c = 0.1;
    for mover in model.movers.iter_mut() {
        let v = mover.velocity();
        let speed = (v.x.powf(2.0) + v.y.powf(2.0)).sqrt();
        let drag_mag = c * speed.powf(2.0);
        let drag = v * -1.0 * drag_mag;
        // TODO: fix drag normalization that returns NAN
        // let drag = drag.normalize() * drag_mag;
        mover.apply_force(drag);
        mover.apply_force(wind);
        mover.apply_force(gravity);
        movers.push(mover.update(&boundary));
    }
    model.movers = movers;
}
