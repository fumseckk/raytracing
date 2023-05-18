use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16. / 9.;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let origin = Point3::from(0., 0., 0.);
        let horizontal = Vec3(viewport_width, 0., 0.);
        let vertical = Vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 - Point3::from(0., 0., focal_length);

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
