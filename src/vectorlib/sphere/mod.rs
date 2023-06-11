use rand::{random, rngs::ThreadRng};

use crate::vectorlib::{hit::*, point3::*, ray::*, vector3::*};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f32) -> Sphere {
        return Sphere { center, radius };
    }
}

impl<'a> Hittable<'a> for Sphere {
    fn hit(self: &Self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData> {
        let oc = ray.origin().clone() - self.center;

        let a = ray.direction().square_magnitude();
        let b_half = oc.dot(ray.direction());
        let c = oc.square_magnitude() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;
        // If no intersections return
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in acceptable range
        let root_discriminant = discriminant.sqrt();

        // try negative root 
        let mut root = (-b_half - root_discriminant) / a;
        if root < t_min || t_max < root {
            // try positive root 
            root = (-b_half + root_discriminant) / a;
            if root < t_min || t_max < root {
                // both are not in range so return None
                return None;
            }
        }

        let t = root;
        let hit_point = ray.at(t);
        // Vector from center of circle to point of intersection turnt into a unit vector 
        let normal = (hit_point - self.center) / self.radius;
        
        return Some(HitData::new(t, ray.at(t), normal, ray.direction(), &normal));
    }
}

const INVERSE_SQRT_THREE : f32 = 0.5773502691896257; // (1.0/3.0).sqrt()
const TWICE_INVERSE_SQRT_THREE : f32 = 1.1547005383792515; // (1.0/3.0).sqrt() * 2

fn random_in_unit_sphere_hack() -> Vector3f{
    // v is a vector in sphere of radius 
    return Vector3f::new(
        (rand::random::<f32>() * TWICE_INVERSE_SQRT_THREE) - INVERSE_SQRT_THREE, 
        (rand::random::<f32>() * TWICE_INVERSE_SQRT_THREE) - INVERSE_SQRT_THREE, 
        (rand::random::<f32>() * TWICE_INVERSE_SQRT_THREE) - INVERSE_SQRT_THREE,
    );
}

fn random_in_unit_sphere() -> Vector3f{
    loop {
        let v = Vector3f::new(
            (rand::random::<f32>()*2.0)-1.0,
            (rand::random::<f32>()*2.0)-1.0,
            (rand::random::<f32>()*2.0)-1.0
        );

        if v.magnitude() > 1.0 {
            continue;
        }
        
        return v;
    }
}

fn random_in_unit_lambertian() -> Vector3f{
    return random_in_unit_sphere().unit_vector();
}

pub fn random_vec_in_unit_sphere() ->Vector3f{
    if (crate::constants::QUICK_RENDER) {
        return random_in_unit_sphere_hack();
    }
    return random_in_unit_sphere();
}

pub fn random_in_hemisphere(normal : &Vector3f) -> Vector3f{
    let in_unit_sphere : Vector3f = random_in_unit_sphere_hack();

    // If its in the hemisphere opposite the normal
    if in_unit_sphere.dot(normal) < 0.0 {
        return -in_unit_sphere;
    }
    return in_unit_sphere;
}