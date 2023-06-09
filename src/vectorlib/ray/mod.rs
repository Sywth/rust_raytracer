use crate::vectorlib::{point3::Point3, vector3::Vector3f};

use super::{hit::HittableList, vector3::{Normalize, Lerp}};

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

pub trait Fireable {
    fn find_color_from_ray_in_world(&self, meshes : &HittableList) -> Vector3f;
}

impl Fireable for Ray{
    fn find_color_from_ray_in_world(&self, meshes : &HittableList) -> Vector3f {
        let hit = meshes.hit(self, 0.0, std::f32::INFINITY);
        if hit.is_some() {
            let color: Vector3f = (hit.unwrap().normal + Vector3f::one()) * 0.5 ;
            return color;
        }
    
        // Hit nothing so get background color

        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0); // Moves t from range [-1,1] to [0,1]
    
        let white = Vector3f::new(1.0, 1.0, 1.0);
        let sky_blue = Vector3f::new(0.5, 0.7, 1.0);
    
        return white.lerp(&sky_blue, t);
    }
}