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
        // Newtons secons law is that acceleration equals force divided by mass.
        let f = force / self.mass;
        println!("f: {:?}))))))))", f);
        self.acceleration += f;
    }

    pub fn new(location: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Mover {
            location,
            velocity,
            acceleration,
            mass: 10.0,
        }
    }

    pub fn update(self, boundary: &Rect) -> Mover {
        let velocity = constrain(boundary, self.velocity + self.acceleration);
        let location = constrain(boundary, self.location + velocity);
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
}

fn constrain(boundary: &Rect, location: Vec2) -> Vec2 {
    let mut location = location;
    if location.x > boundary.right() {
        location.x = boundary.right();
    } else if location.x < boundary.left() {
        location.x = boundary.left();
    }

    if location.y > boundary.top() {
        location.y = boundary.top();
    } else if location.y < boundary.bottom() {
        location.y = boundary.bottom();
    }

    Vec2::new(location.x, location.y)
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
        assert_eq!(mover.acceleration, Vec2::new(0.299, 0.31));
        assert_eq!(mover.location(), Vec2::new(0.0, 0.0));
        let mover = mover.update(&boundary);
        assert_eq!(mover.acceleration, Vec2::new(0.0, 0.0));
        assert_eq!(mover.location(), Vec2::new(0.299, 0.31));
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
    pub fn init_mover(init_acc: Vec2) -> Mover {
        let location = Vec2::new(0.0, 0.0);
        let velocity = Vec2::new(0.0, 0.0);
        Mover::new(location, velocity, init_acc)
    }

    #[fixture]
    pub fn mouse_location() -> Vec2 {
        Vec2::new(62.0, 40.0)
    }
}
