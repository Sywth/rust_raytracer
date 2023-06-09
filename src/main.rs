mod vectorlib;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::rngs::StdRng;
use vectorlib::{hit::*,point3::*,color::*,ray::*,vector3::*,sphere::*};

use chrono;

use rand::prelude::*;
use rand::SeedableRng;

const FOLDER_NAME: &str = "./render";
const IMAGE_WIDTH: u16 = 800;
const IMAGE_HEIGHT: u16 = 450;

const ASPECT_RATIO: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

pub trait IntersectsSphere {
    fn intersection_with(&self, center: &Point3, radius: f32) -> Option<f32>;
}
impl IntersectsSphere for Ray {
    fn intersection_with(&self, center: &Point3, radius: f32) -> Option<f32> {
        let ray_origin_to_center = self.origin().clone() - center.clone();

        let a = self.direction().dot(self.direction());
        let b_half = ray_origin_to_center.dot(self.direction());
        let c = ray_origin_to_center.dot(&ray_origin_to_center) - radius * radius;

        let discriminant = b_half * b_half - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let real_solution = (-b_half - (discriminant).sqrt()) / a;
        return Some(real_solution);
    }
}

pub fn get_color_from_ray(ray: &Ray) -> Color24b {
    let circle_center = &Point3::new(0.0, 0.0, -1.0);
    let intersection = ray.intersection_with(circle_center, 0.5);
    if intersection.is_some() {
        let normal = (ray.at(intersection.unwrap()) - *circle_center).normalize();
        let color = 0.5
            * Vector3f::new(
                normal.x + 1.0 + intersection.unwrap(),
                normal.y + 1.0,
                normal.z + 1.0,
            );
        return Color::from_vec(color).to_24_bit();
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0); // Moves t from range [-1,1] to [0,1]

    let white = Vector3f::new(1.0, 1.0, 1.0);
    let sky_blue = Vector3f::new(0.5, 0.7, 1.0);

    return white.lerp(&sky_blue, t).to_color().to_24_bit();
}

pub fn get_color_from_ray_2(ray: &Ray, spheres : &[Sphere]) -> Color24b{
    let mut closest_hit : Option<HitData> = None;
    for sphere in spheres.iter(){
        let hit = sphere.hit(ray, 0.001, 10.0);
        if hit.is_none() {continue;}

        if closest_hit.is_none(){
            closest_hit = hit;
            continue;
        }

        if hit.unwrap().t < closest_hit.unwrap().t {
            closest_hit = hit;
        }
    }

    if closest_hit.is_some(){
        let hit = closest_hit.unwrap();
        let normal = hit.normal;
        // let color = 0.5
        //     * Vector3f::new(
        //         normal.x + 1.0 + hit.t,
        //         normal.y + 1.0,
        //         normal.z + 1.0,
        //     );
        // return Color::from_vec(color).to_24_bit();
        let light1 = Vector3f::new(0.0,1.0,0.9);
        let mut brightness = hit.normal.dot(&light1);
        let mut color = Vector3f::new(brightness * 1.0, brightness * 1.0, brightness * 1.0);

        let light2 = Vector3f::new(0.4,-1.0,-0.7);
        brightness = hit.normal.dot(&light2);
        color = color + Vector3f::new(brightness * 1.0, brightness * 1.0, brightness * 1.0);

        // println!("{}",brightness);
        return color.to_color().to_24_bit();
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0); // Moves t from range [-1,1] to [0,1]

    let white = Vector3f::new(1.0, 1.0, 1.0);
    let sky_blue = Vector3f::new(0.5, 0.7, 1.0);

    return white.lerp(&sky_blue, t).to_color().to_24_bit();
}

fn main() -> std::io::Result<()> {
    let file_name: String = format!(
        "image_{}.ppm",
        chrono::offset::Local::now().format("%d_%m_%Y_T%H_%M_%S")
    );
    let full_path = Path::new("").join(FOLDER_NAME).join(file_name);

    let mut file = File::create(full_path.clone()).expect(
        format!(
            "[ERR] Unable to create file! {}",
            full_path.to_str().unwrap_or("[ERR] Cannot unwrap")
        )
        .as_str(),
    );

    file.write(b"P3\n").expect("[ERR] Cannot write to file!");
    file.write(format!("{} {}\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("[ERR] Cannot write to file!");
    file.write(b"255\n").expect("[ERR] Cannot write to file!");

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3f::new(VIEWPORT_WIDTH as f32, 0.0, 0.0);
    let vertical = Vector3f::new(0.0, VIEWPORT_HEIGHT as f32, 0.0);
    let lower_left_corner = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - Vector3f::new(0.0, 0.0, FOCAL_LENGTH as f32);
    print!("Origin {}", origin);
    print!("Origin {}", origin);
    print!("Origin {}", origin);



    let seed : u64 = 53;
    let mut rng = StdRng::seed_from_u64(seed);

    let x_pos_range = &(-5.0..5.0);
    let y_pos_range = &(-0.5..1.2);
    let z_pos_range = &(-3.0..-1.00);
    let radius_range = &(0.01..0.80);

    let mut spheres: Vec<Sphere>  = Vec::new();
    for _ in 0..10{
        let sphere = Sphere::new(Vector3f::new(
            rng.gen_range(x_pos_range.clone()),
            rng.gen_range(y_pos_range.clone()),
            rng.gen_range(z_pos_range.clone()),
        ), 
        rng.gen_range(radius_range.clone()));
        spheres.push(sphere);
    }



    //iterate throught width and height of image
    for j in (0..IMAGE_HEIGHT).rev() {
        println!(
            "...{:.2}%",
            ((IMAGE_HEIGHT - j) as f32 / IMAGE_HEIGHT as f32) * 100 as f32
        );
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let target_screen_pos = lower_left_corner + u * horizontal + v * vertical;
            let ray = Ray::new(origin, (target_screen_pos) - origin);
            let color = get_color_from_ray_2(&ray,&spheres);

            color.write(&mut file);
        }
    }

    let x = Vector3f::new(1.0, 1.0, 1.0);
    let y = Vector3f::new(1.0, -1.0, -1.0);
    println!("{} = 0 -2 2", y.cross(&x).to_string());
    println!("{} = 0 2 -2", x.cross(&y).to_string());
    println!("{} ", (y / 3.0).to_string());
    println!("{} ", (&x.square_magnitude()).to_string());
    println!("{} ", (&x.magnitude()).to_string());
    println!("{} ", y.dot(&x).to_string());
    println!("{} ", x.dot(&y).to_string());

    println!("{}", Color::new(253.0, -421.0, 45215.0));

    return Ok(());
}
