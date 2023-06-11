mod camera;
mod constants;
mod utils;
mod vectorlib;
mod material;

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
    
    // Start timer
    let start = chrono::offset::Local::now();

    meshes.add(Sphere::new(Vector3f::new(0.0, 0.0, -1.0), 0.5));
    meshes.add(Sphere::new(Vector3f::new(0.0, -100.5, -1.0), 100.0));

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

            if QUICK_RENDER && (j%2==0 && i%2==1 || i%2==0 && j%2==1) {Color::min_color().to_24_bit().write(&mut file);continue;}

            let mut pixel_color = Vector3f::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random::<f32>()) as f32 / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + random::<f32>()) as f32 / (IMAGE_HEIGHT - 1) as f32;

                let sample_color = camera.get_ray(u, v).find_color_from_ray_in_world(&meshes,MAX_BOUNCES);
                pixel_color = pixel_color + (sample_color / SAMPLES_PER_PIXEL as f32);
            }
            pixel_color.to_color().gamma_two_correct().to_24_bit().write(&mut file);
        }
    }

    // End timer
    let end = chrono::offset::Local::now();
    let duration = end - start;
    println!("Time taken: {} ms", duration.num_milliseconds());

    return Ok(());
}
