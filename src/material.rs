use std::io::Write;

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
}

impl Metal {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
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
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}
