use crate::vectorlib::{point3::*,vector3::*,ray::*};

#[derive(Copy, Clone)]
pub struct HitData{
    pub at : Point3,
    pub normal : Vector3f,
    pub t : f32,
}

impl HitData{
    pub fn new(t : f32, at : Point3, normal : Vector3f) -> HitData{
        return HitData{
            at : (at),
            normal : (normal),
            t : (t)
        };
    }
}

pub trait Hittable {
    fn hit(self : &Self,ray : &Ray, t_min : f32, t_max : f32) -> Option<HitData>;
}