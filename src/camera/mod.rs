use crate::vectorlib::{point3::*, vector3::*, ray::*};

pub struct Camera {
    origin : Point3,
    horizontal : Vector3f,
    vertical : Vector3f,
    bottom_left : Point3,
}

impl Camera {
    pub fn new(viewport_width : f32, viewport_height : f32, focal_length : f32, origin : Point3) -> Camera{
        let horizontal : Vector3f = Vector3f::new(viewport_width,0.0,0.0);
        let vertical : Vector3f = Vector3f::new(0.0,viewport_height,0.0);
        let bottom_left = (origin - (horizontal/2.0) - (vertical/2.0)) - Vector3f::new(0.0, 0.0, focal_length);

        return Camera { 
            origin: (origin),
            horizontal: (horizontal),
            vertical: (vertical),
            bottom_left: (bottom_left),
        }
    }

    // u and v should be in range [0,1]
    pub fn get_ray(& self,u : f32, v : f32) -> Ray{
        return Ray::new(self.origin, self.bottom_left + (u*self.horizontal) + (self.vertical*v) - self.origin);
    }
}