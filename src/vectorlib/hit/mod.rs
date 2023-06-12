use std::rc::Rc;

use crate::vectorlib::{point3::*, ray::*, vector3::*};
use crate::material::*;

#[derive(Clone)]
pub struct HitData<'a>{
    pub at: Point3,
    pub normal: Vector3f,
    pub t: f32,
    pub hit_front_face: bool,
    pub material : &'a dyn Material,
}

impl<'a> HitData<'a>{
    // outward_normal most likely will be same as normal but incase
    pub fn new(t: f32, hit_at: Point3, normal: Vector3f, ray_direction: &Vector3f, outward_normal: &Vector3f, material : &'a dyn Material) -> HitData<'a>
    {
        let mut hit_data = HitData {
            at: hit_at,
            normal,
            t,
            // Set to some default for now
            hit_front_face: (false),
            material : material,
        };

        // Determine if we hit front or back

        // We hit the front face if the dot product of the outwards normal and ray is negative, else if 0 or above hit backface
        hit_data.hit_front_face = ray_direction.dot(outward_normal) < 0.0;
        hit_data.normal = match hit_data.hit_front_face {
            true => outward_normal.clone(),
            false => -outward_normal.clone(),
        };
        return hit_data;
    }

}

pub trait Hittable<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData>;
}

pub struct HittableList<'a> {
    objects: Vec<Rc<Box<dyn Hittable<'a> + 'a>>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: impl Hittable<'a> + 'a) {
        self.objects.push(Rc::new(Box::new(object)));
    }

    pub fn clear_all(&mut self){
        self.objects.clear();
    }

    pub fn hit(&self,ray : &Ray, t_min : f32, t_max : f32) -> Option<HitData>{
        let mut closest_hit : Option<HitData> = None;
        let mut closest_t : f32 = t_max;

        for object in self.objects.iter(){
            let hit = object.hit(ray, t_min, closest_t);

            // If hit nothing pass
            if hit.is_none(){
                continue;
            }

            // If hit something and less than previous (or t_max if no prev) save it as new best, continue
            let hit: HitData = hit.unwrap();
            if hit.t < closest_t {
                closest_hit = Some(hit.clone());
                closest_t = hit.t;
                continue;
            }
        }

        return closest_hit;
    }
}