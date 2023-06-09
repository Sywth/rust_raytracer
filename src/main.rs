mod camera;
mod constants;
mod utils;
mod vectorlib;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use chrono;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use camera::*;
use constants::*;
use vectorlib::{color::*, hit::*, point3::*, ray::*, sphere::*, vector3::*};

pub fn get_color(ray: &Ray, meshes: &HittableList) -> Vector3f {
    let hit = meshes.hit(ray, 0.0, std::f32::INFINITY);
    if hit.is_some() {
        let color: Vector3f = (hit.unwrap().normal + Vector3f::one()) * 0.5;
        return color;
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0); // Moves t from range [-1,1] to [0,1]

    let white = Vector3f::new(1.0, 1.0, 1.0);
    let sky_blue = Vector3f::new(0.5, 0.7, 1.0);

    return white.lerp(&sky_blue, t);
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

    let mut meshes: HittableList = HittableList::new();

    let mut rng: StdRng = StdRng::seed_from_u64(SEED);

    /* 
    let x_pos_range = &(-5.0..5.0);
    let y_pos_range = &(-0.5..1.2);
    let z_pos_range = &(-3.0..-1.50);
    let radius_range = &(0.01..0.80);

    for _ in 0..75 {
        let sphere = Sphere::new(
            Vector3f::new(
                rng.gen_range(x_pos_range.clone()),
                rng.gen_range(y_pos_range.clone()),
                rng.gen_range(z_pos_range.clone()),
            ),
            rng.gen_range(radius_range.clone()),
        );
        meshes.add(sphere)
    }
    */

    meshes.add(Sphere::new(Vector3f::new(0.0, 0.0, -2.0), 1.0));
    meshes.add(Sphere::new(Vector3f::new(0.0, -104.0, -5.0), 100.0));

    let camera = Camera::new(
        VIEWPORT_WIDTH,
        VIEWPORT_HEIGHT,
        FOCAL_LENGTH,
        Vector3f::zero(),
    );

    //iterate throught width and height of image
    for j in (0..IMAGE_HEIGHT).rev() {
        println!(
            "...{:.2}%",
            ((IMAGE_HEIGHT - j) as f32 / IMAGE_HEIGHT as f32) * 100 as f32
        );
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vector3f::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) as f32 / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random::<f32>()) as f32 / (IMAGE_HEIGHT - 1) as f32;

                let sample_color = camera.get_ray(u, v).find_color_from_ray_in_world(&meshes);
                pixel_color = pixel_color + (sample_color / SAMPLES_PER_PIXEL as f32);
            }

            pixel_color.to_color().to_24_bit().write(&mut file);
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
