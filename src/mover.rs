use nannou::geom::{rect::Rect, vector::Vec2};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

fn constrain_location(boundary: &Rect, location: Vec2) -> Vec2 {
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

impl Mover {
    fn constrain(self, boundary: &Rect) -> [Vec2; 2] {
        let velocity_limit = 5.0;
        let mut velocity = self.velocity + self.acceleration;
        let mut location = self.location + velocity;

        // Set a velocity limit
        if velocity.x > velocity_limit {
            velocity.x = velocity_limit
        }

        if velocity.y > velocity_limit {
            velocity.y = velocity_limit
        }

        // Keep the mover within the window boundaries.
        if location.x < boundary.left() {
            location.x = boundary.right()
        } else if location.x > boundary.right() {
            location.x = boundary.left()
        }

        if location.y < boundary.bottom() {
            location.y = boundary.top()
        } else if location.y > boundary.top() {
            location.y = boundary.bottom()
        }

        [location, velocity]
    }

    pub fn new(location: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Mover {
            location,
            velocity,
            acceleration,
        }
    }

    pub fn update(self, boundary: &Rect) -> Mover {
        let [location, velocity] = self.constrain(boundary);
        Mover {
            location,
            velocity,
            acceleration: self.acceleration,
        }
    }

    pub fn folow_mouse(self, boundary: &Rect, mouse_location: Vec2) -> Mover {
        let dir = Vec2::normalize(mouse_location - self.location());
        let acceleration = dir * 0.5;

        let velocity = self.velocity + acceleration;
        let location = constrain_location(boundary, self.location + velocity);
        Mover {
            location,
            velocity,
            acceleration,
        }
    }

    pub fn location(&self) -> Vec2 {
        self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nannou::geom::range::Range;
    use rstest::*;
    #[rstest]
    pub fn should_follow_mouse_location(boundary: Rect, init_mover: Mover, mouse_location: Vec2) {
        let mover = init_mover;
        let dir = Vec2::normalize(mouse_location - init_mover.location());
        let acceleration = dir * 0.5;

        let mover = mover.folow_mouse(&boundary, mouse_location);

        let expected = Mover::new(acceleration, acceleration, acceleration);
        assert!(mover == expected);
    }

    #[rstest]
    pub fn should_update_with_constant_acceleration(
        boundary: Rect,
        acceleration: [f32; 2],
        init_mover: Mover,
    ) {
        let [acc_x, acc_y] = acceleration;
        let acceleration = Vec2::new(acc_x, acc_y);
        let mover = init_mover;

        let mover = mover.update(&boundary);
        let expected = Mover::new(
            Vec2::new(acc_x, acc_y),
            Vec2::new(acc_x, acc_y),
            acceleration,
        );

        assert!(mover == expected);

        let mover = mover.update(&boundary);
        let expected = Mover::new(
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            Vec2::new(acc_x * 2.0, acc_y * 2.0),
            acceleration,
        );
        assert!(mover == expected);

        let mover = mover.update(&boundary);
        let expected = Mover::new(
            Vec2::new(acc_x * 6.0, acc_y * 6.0),
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            acceleration,
        );

        assert!(mover == expected);
    }

    #[fixture]
    pub fn boundary() -> Rect {
        let range = Range::new(-512.0, 512.0);
        Rect { x: range, y: range }
    }

    #[fixture]
    pub fn acceleration() -> [f32; 2] {
        let acc_x = -0.001;
        let acc_y = 0.01;

        [acc_x, acc_y]
    }

    #[fixture]
    pub fn init_mover(acceleration: [f32; 2]) -> Mover {
        let [acc_x, acc_y] = acceleration;
        let location = Vec2::new(0.0, 0.0);
        let velocity = Vec2::new(0.0, 0.0);
        let acceleration = Vec2::new(acc_x, acc_y);
        Mover::new(location, velocity, acceleration)
    }

    #[fixture]
    pub fn mouse_location() -> Vec2 {
        Vec2::new(62.0, 40.0)
    }
}
