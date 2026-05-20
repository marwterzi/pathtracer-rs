use rand::RngExt;
use std::sync::Arc;
use crate::vector3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Material};
use crate::color::Color;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered  = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}


pub struct Metal {
    albedo: Color,
    fuzz:   f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = r_in.direction().reflect(rec.normal);
        
        let reflected = reflected.unit_vector() + self.fuzz * Vec3::random_unit_vector();

        *scattered   = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(rec.normal) > 0.0
    }
}


pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
       
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta      = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta      = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rand::rng().random() {
            unit_direction.reflect(rec.normal)      
        } else {
            unit_direction.refract(rec.normal, ri)  
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}
