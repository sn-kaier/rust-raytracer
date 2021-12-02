use std::rc::Rc;
use glam::{Vec2, Vec3A};
use crate::geometry::ray::Ray;
use crate::scene::material::Material;

pub trait Traceable {
    fn intersect(&self, ray: &Ray, t: &mut f32) -> bool;
    /// returns (normal, reflection)
    fn intersection_normal(&self, ray: &Ray, intersection_point: Vec3A) -> (Vec3A, Vec3A);
    fn update(&mut self);
    fn get_mat(&self) -> Rc<Material>;
    fn get_texture_coord(&self, position: &Vec3A) -> Vec2;
}
