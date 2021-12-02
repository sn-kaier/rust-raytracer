use core::f32::consts::PI;
use std::rc::Rc;
use glam::{Vec2, Vec3A};
use rand::Rng;
use rand::rngs::OsRng;
use crate::geometry::ray::Ray;
use crate::geometry::traceable::Traceable;
use crate::scene::material::Material;

pub fn uv_map(n: &Vec3A) -> (f32, f32) {
    let u = n.x.atan2(n.z) / (2.0 * PI) + 0.5;
    let v = -n.y * 0.5 + 0.5;
    (u, v)
}

pub struct Sphere {
    pub center: Vec3A,
    org_center: Vec3A,
    pub r: f32,
    pub r2: f32,
    t: i32,
    pub mat: Rc<Material>,
}

impl Sphere {
    pub fn create(center: Vec3A, r: f32, mat: Rc<Material>) -> Sphere {
        Sphere {
            center,
            org_center: center,
            r,
            r2: r * r,
            t: OsRng::default().gen_range(0..100),
            mat,
        }
    }
}

impl Traceable for Sphere {
    fn intersect(&self, ray: &Ray, t: &mut f32) -> bool {
        let l = self.center - ray.org;
        let t_ca = l.dot(ray.dir);
        if t_ca < 0.0 {
            return false;
        }
        let d2 = l.length_squared() - t_ca * t_ca;
        if d2 > self.r2 {
            return false;
        }
        let thc = (self.r2 - d2).sqrt();
        let mut t0 = t_ca - thc;
        let mut t1 = t_ca + thc;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }

        *t = t0;
        t0 > 0.00001
    }

    fn intersection_normal(&self, ray: &Ray, intersection_point: Vec3A) -> (Vec3A, Vec3A) {
        let normal = (intersection_point - self.center).normalize();

        let dot = ray.dir.dot(normal);
        let reflection = ray.dir - 2.0 * dot * normal;
        (normal, reflection)
    }

    fn update(&mut self) {
        self.t = self.t + 1;
        let fract = self.t as f32 / 10.0;
        let scale = 1.0;
        let elevation = Vec3A::Y * fract.cos() * scale;
        self.center = self.org_center + elevation;
    }

    fn get_mat(&self) -> Rc<Material> {
        self.mat.clone()
    }

    fn get_texture_coord(&self, position: &Vec3A) -> Vec2 {
        let normal = (*position - self.center).normalize();
        let (u, v) = uv_map(&normal);
        Vec2::new(u, v)
    }
}
