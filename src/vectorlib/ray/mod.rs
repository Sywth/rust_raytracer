use crate::vectorlib::{point3::Point3, vector3::Vector3f, sphere::*};

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
    fn find_color_from_ray_in_world(&self, meshes : &HittableList, recursions_left : u16) -> Vector3f;
}

const SHADOW_ACNE_TOLERANCE : f32 = 0.0001;
impl Fireable for Ray{
    fn find_color_from_ray_in_world(&self, meshes : &HittableList, bounces_left : u16) -> Vector3f {
        if bounces_left < 1{
            // Return black as we hit nothing that emits light
            return Vector3f::zero();
        }

        let maybe_hit: Option<super::hit::HitData> = meshes.hit(self, SHADOW_ACNE_TOLERANCE, std::f32::INFINITY);
        if maybe_hit.is_some() {
            let hit = maybe_hit.unwrap();

            let attenuation : &Vector3f;
            let scattered_ray : Ray;
            {
                let attenuation_and_scattered_ray = hit.material.scatter(self, &hit);
                attenuation = attenuation_and_scattered_ray.0;
                scattered_ray = attenuation_and_scattered_ray.1;
            }

            // With every bounce we lose half the energy contribution to color 
            return attenuation.multiply_element_wise(&scattered_ray.find_color_from_ray_in_world(meshes, bounces_left - 1));
        }
    
        // Hit nothing so get naturally emissive background color
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0); // Moves t from range [-1,1] to [0,1]
    
        let white = Vector3f::new(1.0, 1.0, 1.0);
        let sky_blue = Vector3f::new(0.5, 0.7, 1.0);
    
        return white.lerp(&sky_blue, t);
    }
}