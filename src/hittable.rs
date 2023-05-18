use crate::{Material, Point3, Ray, Vec3};
use std::rc::Rc;

// front_face is true if the normal is pointing outwards.
// #[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> Vec3 {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        if front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    // The returned normal vector will always be in the opposite direction as the ray (e.g if ray hits from outside, normal will point outward)
}

// #[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a = ray.direction().squared_norm();
        let half_b = Vec3::dot(ray.direction(), ray.origin() - self.center);
        let c = Vec3::squared_dist(ray.origin(), self.center) - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;
        let sqrdt = discrim.sqrt();

        if discrim < 0. {
            return None;
        }

        // Find the nearest root between t_min and t_max
        let mut root = (-half_b - sqrdt) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrdt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let outward_normal = (ray.at(root) - self.center) / self.radius;

        Some(HitRecord {
            t: root,
            p: ray.at(root),
            normal: HitRecord::set_face_normal(&ray, outward_normal),
            front_face: Vec3::dot(ray.direction(), outward_normal) < 0.0,
            mat_ptr: Rc::clone(&self.mat_ptr),
        })
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                res = Some(rec);
            }
        }

        res
    }
}
