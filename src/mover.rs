use nannou::geom::vector::Vec2;

#[derive(PartialEq, Debug)]
pub struct Mover {
    area: [f32; 2],
    location: Vec2,
    velocity: Vec2,
}

impl Mover {
    fn verify_limit(&self, location: Vec2) -> Vec2 {
        let mut location = location;
        if location.x > self.area[0] / 2.0 {
            location.x = -self.area[0] / 2.0;
        } else if location.x < -self.area[0] / 2.0 {
            location.x = self.area[0] / 2.0;
        }

        if location.y > self.area[1] / 2.0 {
            location.y = -self.area[1] / 2.0;
        } else if location.y < -self.area[1] / 2.0 {
            location.y = self.area[1] / 2.0;
        }
        location
    }
    pub fn update(self) -> Self {
        Self {
            area: self.area,
            location: self.verify_limit(self.location + self.velocity),
            velocity: self.velocity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    pub fn should_update_mover(area: [f32; 2]) {
        let velocity = Vec2::new(1.0, 2.0);
        let mover = Mover {
            area,
            velocity,
            location: Vec2::new(100.0, -100.0),
        };

        let updated_mover = mover.update();
        let expected_mover = Mover {
            area,
            velocity,
            location: Vec2::new(101.0, -98.0),
        };
        assert!(updated_mover == expected_mover);
    }

    #[rstest]
    pub fn should_remain_within_x_bounds(area: [f32; 2]) {
        let velocity = Vec2::new(1.0, 1.0);
        let mover = Mover {
            area,
            velocity,
            location: Vec2::new(250.0, 0.0),
        };

        let expected_mover = Mover {
            area,
            velocity,
            location: Vec2::new(-250.0, 1.0),
        };
        assert!(mover.update() == expected_mover);

        let velocity = Vec2::new(-1.0, 1.0);
        let mover = Mover {
            area,
            velocity,
            location: Vec2::new(-250.0, 0.0),
        };

        let expected_mover = Mover {
            area,
            velocity,
            location: Vec2::new(250.0, 1.0),
        };
        assert!(mover.update() == expected_mover);
    }

    #[rstest]
    pub fn should_remain_within_y_bounds(area: [f32; 2]) {
        let velocity = Vec2::new(1.0, 1.0);
        let mover = Mover {
            area,
            velocity,
            location: Vec2::new(0.0, 250.0),
        };

        let expected_mover = Mover {
            area,
            velocity,
            location: Vec2::new(1.0, -250.0),
        };
        assert!(mover.update() == expected_mover);

        let velocity = Vec2::new(1.0, -1.0);
        let mover = Mover {
            area,
            velocity,
            location: Vec2::new(0.0, -250.0),
        };

        let expected_mover = Mover {
            area,
            velocity,
            location: Vec2::new(1.0, 250.0),
        };
        assert!(mover.update() == expected_mover);
    }

    #[fixture]
    pub fn area() -> [f32; 2] {
        [500.0, 500.0]
    }
}
