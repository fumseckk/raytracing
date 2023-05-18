use rand;
use rand::Rng;
use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

// Formatting: prints the 3 struct members, separated with a space, no newline.
impl Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)?;
        Ok(())
    }
}

// Unary Vec3 operations
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Vec3 * Vec3 operations
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

// f64 * Vec3 operations
impl Add<Vec3> for f64 {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Vec3(other.0 + self, other.1 + self, other.2 + self)
    }
}

impl Sub<Vec3> for f64 {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Vec3(self - other.0, self - other.1, self - other.2)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Vec3(other.0 * self, other.1 * self, other.2 * self)
    }
}

impl Div<Vec3> for f64 {
    type Output = Color;

    fn div(self, other: Color) -> Self::Output {
        Vec3(self / other.0, self / other.1, self / other.2)
    }
}

// Vec3 * f64 operations
impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self(self.0 + other, self.1 + other, self.2 + other)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self(self.0 - other, self.1 - other, self.2 - other)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self * (1.0 / other)
    }
}

// Assignment Vec3 *= Vec3 operations
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other
    }
}

// Assignment Vec3 *= f64 operations
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = *self + other
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = *self - other
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = *self * other
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other
    }
}

impl Vec3 {
    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn squared_norm(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.squared_norm().sqrt()
    }

    pub fn squared_dist(self, other: Vec3) -> f64 {
        (self - other).squared_norm()
    }

    pub fn dist(self, other: Vec3) -> f64 {
        self.squared_dist(other).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalized(self) -> Vec3 {
        self / self.norm()
    }

    pub fn random() -> Vec3 {
        Vec3(rand::random(), rand::random(), rand::random())
    }

    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3(
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
            rand::thread_rng().gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1., 1.);
            if p.squared_norm() >= 1. {
                continue;
            };
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    // Return true if the vector is close to zero in all directions
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.0 < s) && (self.1 < s) && (self.2 < s)
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * Vec3::dot(self, n) * n
    }
}

impl Color {
    pub fn r(self) -> f64 {
        self.0
    }

    pub fn g(self) -> f64 {
        self.1
    }

    pub fn b(self) -> f64 {
        self.2
    }

    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            return min;
        }
        if x > max {
            return max;
        }
        return x;
    }

    pub fn write_color(
        file: &mut File,
        color: Color,
        samples_per_pixel: u32,
    ) -> std::io::Result<()> {
        let scaled_color = color / samples_per_pixel as f64;
        let r = scaled_color.r().sqrt();
        let g = scaled_color.g().sqrt();
        let b = scaled_color.b().sqrt();

        writeln!(
            file,
            "{} {} {}",
            (256. * Self::clamp(r, 0.0, 0.999)) as u8,
            (256. * Self::clamp(g, 0.0, 0.999)) as u8,
            (256. * Self::clamp(b, 0.0, 0.999)) as u8
        )?;
        Ok(())
    }

    pub fn write_color_stdout(color: Color) {
        println!(
            "{} {} {}",
            (255.999 * color.r()) as u8,
            (255.999 * color.g()) as u8,
            (255.999 * color.b()) as u8
        );
    }
}
