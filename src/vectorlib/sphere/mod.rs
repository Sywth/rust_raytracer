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

impl Hittable for Sphere {
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
        if (root < t_min || t_max < root) {
            // try positive root 
            root = (-b_half + root_discriminant) / a;
            if (root < t_min || t_max < root) {
                // both are not in range so return None
                return None;
            }
        }

        let t = root;
        let hit_point = ray.at(t);
        // Vector from center of circle to point of intersection turnt into a unit vector 
        let normal = (hit_point - self.center) / self.radius;

        return Some(HitData::new(t,hit_point,normal));
    }
}
