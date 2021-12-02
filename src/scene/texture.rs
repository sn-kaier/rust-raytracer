use glam::{Vec2, Vec3A};
use image::RgbImage;
use image::io::Reader as ImageReader;

pub fn get_pixel(ref img: &RgbImage, pos: &Vec2) -> Vec3A {
    let w = img.width() - 1;
    let h = img.height() - 1;
    let p = img.get_pixel((pos.x * w as f32).round() as u32, (pos.y * h as f32).round() as u32);
    Vec3A::new(p[0] as f32 / 255.0, p[1] as f32 / 255.0, p[2] as f32 / 255.0)
}

pub fn load_texture(path: &str) -> RgbImage {
    let l = ImageReader::open(path);
    if l.is_err() {
        panic!("Problem loading the file: {:?}", l.err());
    }
    let decoded = l.unwrap().decode();
    if decoded.is_err() {
        panic!("Problem loading the file: {:?}", decoded.err());
    }
    decoded.unwrap().to_rgb8()
}

