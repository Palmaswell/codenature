use nannou::geom::{rect::Rect, vector::Vec2};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Mover {
    pub fn apply_force(&mut self, force: Vec2) {
        // * Newtons second law is that acceleration equals
        // * accumulated force divided by mass.
        let f = force / self.mass;
        self.acceleration += f;
    }

    pub fn new(location: Vec2, mass: f32) -> Self {
        Mover {
            location,
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            mass,
        }
    }

    pub fn update(self, boundary: &Rect) -> Mover {
        let mut velocity = self.velocity + self.acceleration;
        let location = constrain_location(boundary, self.location + velocity);
        let velocity = constrain_velocity(boundary, &mut velocity, &location);
        Mover {
            location,
            velocity,
            acceleration: Vec2::new(0.0, 0.0),
            mass: self.mass,
        }
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }
}

fn constrain_location(boundary: &Rect, location: Vec2) -> Vec2 {
    let mut location = location;

    if location.x > boundary.right() {
        location.x = boundary.right();
    }

    if location.x < boundary.left() {
        location.x = boundary.left();
    }

    if location.y > boundary.top() {
        location.y = boundary.top();
    }

    if location.y < boundary.bottom() {
        location.y = boundary.bottom();
    }

    Vec2::new(location.x, location.y)
}

fn constrain_velocity(boundary: &Rect, velocity: &mut Vec2, location: &Vec2) -> Vec2 {
    if location.x >= boundary.right() || location.x <= boundary.left() {
        velocity.x = velocity.x * -1.0;
    }

    if location.y >= boundary.top() || location.y <= boundary.bottom() {
        velocity.y = velocity.y * -1.0;
    }

    Vec2::new(velocity.x, velocity.y)
}

pub fn is_positive(n: f32) -> bool {
    n > 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use nannou::geom::range::Range;
    use rstest::*;

    #[rstest]
    pub fn should_accumulate_force(init_mover: Mover, boundary: Rect) {
        let mut mover = init_mover;
        mover.apply_force(Vec2::new(1.0, 1.0));
        mover.apply_force(Vec2::new(2.0, 2.0));
        assert_eq!(mover.acceleration, Vec2::new(0.3, 0.3));
        assert_eq!(mover.location(), Vec2::new(0.0, 0.0));
        let mover = mover.update(&boundary);
        assert_eq!(mover.acceleration, Vec2::new(0.0, 0.0));
        assert_eq!(mover.location(), Vec2::new(0.3, 0.3));
    }
    #[rstest]
    pub fn should_verify_floag() {
        assert!(is_positive(1.0));
        assert!(!is_positive(-1.0));
    }

    #[fixture]
    pub fn init_mover() -> Mover {
        let location = Vec2::new(0.0, 0.0);
        Mover::new(location, 10.0)
    }

    #[fixture]
    pub fn boundary() -> Rect {
        let range = Range::new(-512.0, 512.0);
        Rect { x: range, y: range }
    }

    #[fixture]
    pub fn init_acc() -> Vec2 {
        let acc_x = -0.001;
        let acc_y = 0.01;

        Vec2::new(acc_x, acc_y)
    }

    #[fixture]
    pub fn mouse_location() -> Vec2 {
        Vec2::new(62.0, 40.0)
    }
}
