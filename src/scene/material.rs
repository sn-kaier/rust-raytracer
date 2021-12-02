use std::rc::Rc;
use glam::Vec3A;
use image::RgbImage;

pub struct Material {
    pub color: Vec3A,
    pub reflect: f32,

    pub texture: Option<Box<RgbImage>>,
    pub normal_map: Option<Box<RgbImage>>,
}

impl Material {
    pub fn create(color: Vec3A, reflect: f32) -> Rc<Material> {
        Rc::new(Material {
            color,
            reflect,
            normal_map: None,
            texture: None,
        })
    }
}
