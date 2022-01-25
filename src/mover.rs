use nannou::geom::{range::Range, rect::Rect, vector::Vec2};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mover {
    location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Mover {
    fn constrain(self, boundary: &Rect) -> [Vec2; 2] {
        let velocity = self.velocity + self.acceleration;
        let mut location = self.location + velocity;

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

    pub fn location(&self) -> Vec2 {
        self.location
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    pub fn should_update_with_constant_acceleration(
        boundary_fixture: Rect,
        acceleration_fixture: [f32; 2],
        init_mover_fixture: Mover,
    ) {
        let [acc_x, acc_y] = acceleration_fixture;
        let acceleration = Vec2::new(acc_x, acc_y);
        let mover = init_mover_fixture;

        let mover = mover.update(&boundary_fixture);
        let expected = Mover::new(
            Vec2::new(acc_x, acc_y),
            Vec2::new(acc_x, acc_y),
            acceleration,
        );

        assert!(mover == expected);

        let mover = mover.update(&boundary_fixture);
        let expected = Mover::new(
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            Vec2::new(acc_x * 2.0, acc_y * 2.0),
            acceleration,
        );
        assert!(mover == expected);

        let mover = mover.update(&boundary_fixture);
        let expected = Mover::new(
            Vec2::new(acc_x * 6.0, acc_y * 6.0),
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            acceleration,
        );

        assert!(mover == expected);
    }

    #[fixture]
    pub fn boundary_fixture() -> Rect {
        let range = Range::new(-512.0, 512.0);
        Rect { x: range, y: range }
    }

    #[fixture]
    pub fn acceleration_fixture() -> [f32; 2] {
        let acc_x = -0.001;
        let acc_y = 0.01;

        [acc_x, acc_y]
    }

    #[fixture]
    pub fn init_mover_fixture(acceleration_fixture: [f32; 2]) -> Mover {
        let [acc_x, acc_y] = acceleration_fixture;
        let location = Vec2::new(0.0, 0.0);
        let velocity = Vec2::new(0.0, 0.0);
        let acceleration = Vec2::new(acc_x, acc_y);
        Mover::new(location, velocity, acceleration)
    }
}
