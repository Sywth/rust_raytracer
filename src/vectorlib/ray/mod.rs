use crate::vectorlib::{point3::Point3, vector3::Vector3f};

pub struct Ray {
    origin: Point3,
    direction: Vector3f,
}

// use super::{Point3, Ray, Vector3f};
impl Ray {
    pub fn new(origin: Point3, direction: Vector3f) -> Ray {
        return Ray{ 
            origin : (origin), 
            direction : (direction)
        };
    }

    pub fn at(&self, t: f32) -> Point3 {
        return self.origin.clone() + (t * self.direction.clone());
    }

    pub fn origin(&self) -> &Point3 {
        return &self.origin;
    }

    pub fn direction(&self) -> &Point3 {
        return &self.direction;
    }
}
