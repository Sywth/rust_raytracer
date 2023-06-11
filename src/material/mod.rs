use crate::vectorlib::{color::Color, hit::HitData, ray::{Ray, self}, vector3::*, sphere::*};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (&Vector3f,Ray);
    fn get_albedo(&self) -> &Vector3f;
}

const VECTOR3F_NEAR_ZERO_TOLERANCE : f32 = 1e-8;
impl Vector3f{
    pub fn near_zero(&self) -> bool{
        return self.x.abs() < VECTOR3F_NEAR_ZERO_TOLERANCE && self.y.abs() < VECTOR3F_NEAR_ZERO_TOLERANCE && self.z.abs() < VECTOR3F_NEAR_ZERO_TOLERANCE;
    }
}

struct Lambertian {
    albedo: Vector3f,
}

impl Material for Lambertian{
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (&Vector3f , Ray) {
        let mut scater_direction = hit_data.normal + random_vec_in_unit_sphere();

        if scater_direction.near_zero(){
            scater_direction = hit_data.normal;
        }

        let scattered_ray = Ray::new(hit_data.at, scater_direction);
        return (&self.albedo, scattered_ray);
    }

    fn get_albedo(&self) -> &Vector3f {
        return &self.albedo;
    }
}

struct Metal {
    albedo: Vector3f,
}

impl Material for Metal{
    fn scatter(&self, ray_in: &Ray, hit_data: &HitData) -> (&Vector3f , Ray) {
        let reflected = ray_in.direction().get_reflected(&hit_data.normal);
        let scattered_ray = Ray::new(hit_data.at, reflected);

        return (&self.albedo, scattered_ray);
    }

    fn get_albedo(&self) -> &Vector3f {
        return &self.albedo;
    }
}