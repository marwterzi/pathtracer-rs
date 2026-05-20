mod vector3;
mod ray;
mod interval;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod material;
mod camera;

use rand::RngExt;
use std::sync::Arc;

use vector3::{Vec3, Point3};
use color::Color;
use hittable_list::HittableList;
use sphere::Sphere;
use material::{Lambertian, Metal, Dielectric};
use camera::Camera;

use hittable::Material; 

fn main() {
    let mut world = HittableList::new();
    let mut rng   = rand::rng();

    let floor = Arc::new(Metal::new(Color::new(0.15, 0.15, 0.18), 0.05));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, floor)));

    let back_wall = Arc::new(Lambertian::new(Color::new(0.05, 0.05, 0.08)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -28.0), 20.0, back_wall)));

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.8, 0.0), 1.8, glass)));

    let glass_inner = Arc::new(Dielectric::new(1.0 / 1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.8, 0.0), 1.55, glass_inner)));

    let orbit_radius = 4.2;
    let orbit_colors = [
        Color::new(0.9, 0.2, 0.2),   
        Color::new(0.2, 0.8, 0.3),   
        Color::new(0.2, 0.3, 0.9),   
        Color::new(0.9, 0.7, 0.1),   
        Color::new(0.8, 0.2, 0.8),   
        Color::new(0.1, 0.8, 0.8),   
    ];

    let count = orbit_colors.len();
    for (i, &col) in orbit_colors.iter().enumerate() {
        let angle = std::f64::consts::TAU * i as f64 / count as f64;
        let x     = orbit_radius * angle.cos();
        let z     = orbit_radius * angle.sin() - 1.0;  
        let mat: Arc<dyn Material> = Arc::new(Lambertian::new(col));
        world.add(Arc::new(Sphere::new(Point3::new(x, 0.8, z), 0.8, mat)));
    }

    let metal_radius = 3.3;
    for i in 0..count {
        let angle = std::f64::consts::TAU * (i as f64 + 0.5) / count as f64;
        let x     = metal_radius * angle.cos();
        let z     = metal_radius * angle.sin() - 1.0;

        let tint: f64   = rng.random_range(0.6..0.95);
        let fuzz: f64   = rng.random_range(0.0..0.15);
        let mat: Arc<dyn Material> = Arc::new(Metal::new(
            Color::new(tint, tint * 0.9, tint * 0.8),
            fuzz,
        ));
        world.add(Arc::new(Sphere::new(Point3::new(x, 0.35, z), 0.35, mat)));
    }

    for _ in 0..40 {
        let x:   f64 = rng.random_range(-7.0..7.0);
        let z:   f64 = rng.random_range(-6.0..2.0);
        let r:   f64 = rng.random_range(0.08..0.22);
        
        if (x*x + z*z).sqrt() < 2.2 { continue; }
        let mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(Point3::new(x, r, z), r, mat)));
    }

    let accent_positions = [
        ( 3.5_f64, 0.6_f64, -0.5_f64, 0.6_f64, 1.7_f64),   
        (-3.5_f64, 0.5_f64,  0.5_f64, 0.5_f64, 2.4_f64),   
        ( 0.0_f64, 0.4_f64,  3.5_f64, 0.4_f64, 1.3_f64),   
    ];
    for (x, y, z, r, ri) in accent_positions {
        let mat: Arc<dyn Material> = Arc::new(Dielectric::new(ri));
        world.add(Arc::new(Sphere::new(Point3::new(x, y, z), r, mat)));
    }

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 600;      
    cam.max_depth         = 60;       

    cam.vfov     = 28.0;              
    cam.lookfrom = Point3::new(0.0, 2.8, 9.5);  
    cam.lookat   = Point3::new(0.0, 1.2, 0.0);  
    cam.vup      = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.8;          
    cam.focus_dist    = 9.0;          

    cam.render(&world);
}
