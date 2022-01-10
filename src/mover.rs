use nannou::geom::vector::Vec2;

#[derive(PartialEq, Debug)]
pub struct Mover {
    location: Vec2,
    velocity: Vec2,
}

impl Mover {
    pub fn update(self) -> Self {
        Self {
            location: self.location + self.velocity,
            velocity: self.velocity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_update_mover() {
        let velocity = Vec2::new(1.0, 2.0);
        let mover = Mover {
            location: Vec2::new(100.0, -100.0),
            velocity,
        };
        let updated_mover = mover.update();
        let expected_mover = Mover {
            location: Vec2::new(101.0, -98.0),
            velocity,
        };
        assert!(updated_mover == expected_mover);
    }
}
