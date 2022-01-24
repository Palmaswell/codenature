use nannou::geom::vector::Vec2;

#[derive(PartialEq, Debug)]
pub struct Mover {
    pub location: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Mover {
    pub fn new(location: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Mover {
            location,
            velocity,
            acceleration,
        }
    }

    pub fn update(self) -> Mover {
        let current_velocity = self.velocity + self.acceleration;
        let current_location = self.location + current_velocity;
        Mover {
            location: current_location,
            velocity: current_velocity,
            acceleration: self.acceleration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    pub fn should_update_with_constant_acceleration() {
        let acc_x = -0.001;
        let acc_y = 0.01;
        let location = Vec2::new(0.0, 0.0);
        let velocity = Vec2::new(0.0, 0.0);
        let acceleration = Vec2::new(acc_x, acc_y);
        let mover = Mover::new(location, velocity, acceleration);
        let mover = mover.update();
        let expected = Mover::new(
            Vec2::new(acc_x, acc_y),
            Vec2::new(acc_x, acc_y),
            acceleration,
        );
        assert!(mover == expected);

        let mover = mover.update();
        let expected = Mover::new(
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            Vec2::new(acc_x * 2.0, acc_y * 2.0),
            acceleration,
        );
        assert!(mover == expected);

        let mover = mover.update();
        let expected = Mover::new(
            Vec2::new(acc_x * 6.0, acc_y * 6.0),
            Vec2::new(acc_x * 3.0, acc_y * 3.0),
            acceleration,
        );

        assert!(mover == expected);
    }
}
