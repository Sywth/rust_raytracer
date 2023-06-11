use rand;
use crate::utils::inv_sqrt;

#[derive(Copy)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait Magnitude {
    type Output;
    fn square_magnitude(&self) -> Self::Output;
    fn magnitude(&self) -> Self::Output;
}

pub trait Normalize {
    fn normalize(& mut self) -> Self;
    fn unit_vector(&self) -> Self;
}

pub trait DotProduct {
    fn dot_product<'a, 'b>(v : &'a Self, u: &'b Self) -> f32;
    fn dot<'a, 'b>(&'a self, other: &'b Self) -> f32;
}

pub trait CrossProduct {
    fn cross<'a>(self, other: &'a Self) -> Self;
    fn cross_product<'a, 'b>(v : &'a Self, u: &'b Self) -> Self;
}

pub trait Lerp{
    fn lerp_self<'a, 'b>(&'a mut self, target: &'b Self, t : f32);
    fn lerp<'a>(self, target: &'a Self, t : f32) -> Self;
}

// use super::{CrossProduct, DotProduct, Magnitude, Normalize, Vector3f};
use std::{fmt, ops};

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        return Vector3f { x, y, z };
    }
    pub fn uniform(s : f32) -> Vector3f{
        return Vector3f::new(s,s,s);
    }

    pub fn lerp<'a, 'b>(start : &'a Vector3f, target: &'b Vector3f, t : f32) -> Vector3f {
        let diff = target.clone() - start.clone();
        return start.clone() + diff * t;
    }

    pub fn faster_unit_vec(self) -> Self{
        return self * inv_sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    pub fn one() -> Vector3f{
        return Vector3f::new(1.0, 1.0, 1.0);
    }
    pub fn zero() -> Vector3f{
        return Vector3f::new(0.0, 0.0, 0.0);
    }
    pub fn random() -> Vector3f{
        return Vector3f::new(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
    }

    pub fn get_reflected(&self, normal : &Vector3f) -> Vector3f{
        return self.clone() - ((2.0 * self.dot(normal)) * normal.clone());
    }

    pub fn random_range(min : f32, max : f32) -> Vector3f{
        return (Vector3f::random() * (max-min)) + Vector3f::uniform(min);
    }

    pub fn unit_x() -> Vector3f{
        return Vector3f::new(1.0, 0.0, 0.0);
    }
    pub fn unit_y() -> Vector3f{
        return Vector3f::new(0.0, 1.0, 0.0);
    }
    pub fn unit_z() -> Vector3f{
        return Vector3f::new(0.0, 0.0, 1.0);
    }

    pub fn multiply_element_wise(&self, other : &Vector3f) -> Self{
        return Vector3f::new(self.x * other.x, self.y * other.y, self.z * other.z);
    }

}

impl ops::Add<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn add(mut self, rhs: Vector3f) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;

        return self;
    }
}

impl ops::Sub<Vector3f> for Vector3f {
    type Output = Vector3f;
    fn sub(mut self, rhs: Vector3f) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        return self;
    }
}

impl ops::Mul<f32> for Vector3f {
    type Output = Vector3f;
    fn mul(mut self, lambda: f32) -> Self::Output {
        self.x *= lambda; 
        self.y *= lambda; 
        self.z *= lambda;
        return self;
    }
}

impl ops::Mul<Vector3f> for f32 {
    type Output = Vector3f;
    fn mul(self, mut v: Vector3f) -> Self::Output {
        v.x *= self; 
        v.y *= self; 
        v.z *= self;
        return v;
    }
}

impl ops::Div<f32> for Vector3f {
    type Output = Vector3f;
    fn div(mut self, lambda: f32) -> Self::Output {
        self.x /= lambda;
        self.y /= lambda; 
        self.z /= lambda;
        return self;
    }
}

impl ops::Neg for Vector3f {
    type Output = Vector3f;
    fn neg(self) -> Self::Output {
        return Vector3f::new(-self.x, -self.y, -self.z);
    }
}

impl fmt::Display for Vector3f {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(formatter, "[{}, {}, {}]", self.x, self.y, self.z);
    }
}

impl Clone for Vector3f{
    fn clone(&self) -> Vector3f{
        return Vector3f::new(self.x, self.y, self.z);
    }
}

impl Magnitude for Vector3f {
    type Output = f32;

    fn square_magnitude(&self) -> Self::Output {
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    fn magnitude(&self) -> Self::Output {
        return self.square_magnitude().sqrt();
    }
}

impl Normalize for Vector3f {
    fn normalize(self : &mut Self) -> Self {
        let magnitude = self.magnitude();
        *self = self.clone() / magnitude;
        return *self;
    }

    fn unit_vector(&self) -> Self {
        let magnitude = self.magnitude();
        return self.clone() / magnitude;
    }
}

impl DotProduct for Vector3f {
    fn dot_product<'a, 'b>(v : &'a Self, u: &'b Self) -> f32 {
        return v.x * u.x + v.y * u.y + v.z * u.z;
    }

    fn dot<'a, 'b>(&'a self, other: &'b Self) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl CrossProduct for Vector3f {
    fn cross<'a>(mut self, other: &'a Self) -> Self {
        self.x = self.y * other.z - self.z * other.y;
        self.y = self.z * other.x - self.x * other.z;
        self.z = self.x * other.y - self.y * other.x;
        return self;
    }

    fn cross_product<'a, 'b>(v : &'a Self, u: &'b Self) -> Self {
        return v.clone().cross(u);
    }
}

impl Lerp for Vector3f{
    fn lerp_self<'a, 'b>(self : &'a mut Self, target: &'b Self, t : f32) {
        let diff = target.clone() - *self;
        *self = *self + (diff * t);
    }

    fn lerp<'a>(self, target: &'a Self, t : f32) -> Vector3f {
        let mut v = self.clone();
        v.lerp_self(target, t);
        return v;
    }
}