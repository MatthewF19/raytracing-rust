use std::io::Write;

use crate::libs::rand_double;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }
}


pub struct Metal {
    albedo: Color,
    fuzz:   f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self { albedo: *albedo, fuzz: if fuzz < 1.0 { fuzz } else { 1.0 } }
    }
}


pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(ior: f64) -> Self {
        Self { ior }
    }

    // Schlik approximation for reflectance
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0*r0;
        return r0 + (1.0 - r0) * f64::powi(1.0-cosine, 5);
    }
}


impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;
        return true;
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return scattered.direction().dot(&rec.normal) > 0.0;
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {1.0 / self.ior} else {self.ior};

        let unit_dir = r_in.direction().unit_vector();
        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let dir: Vec3;

        if cannot_refract || (Dielectric::reflectance(cos_theta, ri) > rand_double()) {
            dir = Vec3::reflect(&unit_dir, &rec.normal);
        } else {
            dir = Vec3::refract(&unit_dir, &rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, dir);
        return true;
    }
}
