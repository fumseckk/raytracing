use crate::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn origin(self) -> Point3 {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
