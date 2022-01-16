use nannou::geom::vector::Vec2;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Mover {
    pub location: Vec2,
    velocity: Vec2,
}

impl Mover {
    pub fn new(location: Vec2, velocity: Vec2) -> Self {
        Mover { location, velocity }
    }

    pub fn update(self, velocity: Vec2) -> Self {
        Self {
            location: self.location + velocity,
            velocity: velocity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    #[rstest]
    pub fn should_update_mover() {
        let velocity = Vec2::new(1.0, 2.0);
        let mover = Mover::new(Vec2::new(100.0, -100.0), velocity);

        let updated_mover = mover.update(velocity);
        let expected_mover = Mover::new(Vec2::new(101.0, -98.0), velocity);
        assert!(updated_mover == expected_mover);
    }
}
