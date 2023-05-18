use crate::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered_direction = rec.normal + Vec3::random_unit_vector();
        let scattered = Ray {
            origin: rec.p,
            direction: if scattered_direction.near_zero() {
                rec.normal
            } else {
                scattered_direction
            },
        };
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

// fuzz: 0 is no fuzziness, it is equal to the radius of the sphere made to randomize the scattered ray.
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_direction = Vec3::reflect(ray_in.direction.normalized(), rec.normal);

        let scattered = Ray {
            origin: rec.p,
            direction: reflected_direction + self.fuzz * Vec3::random_in_unit_sphere(),
        };
        let attenuation = self.albedo;

        if Vec3::dot(scattered.direction, rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
