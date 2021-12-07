use glam::{Vec2, Vec3A};
use image::imageops::FilterType;
use image::{GenericImageView, RgbImage};
use image::io::Reader as ImageReader;

pub fn get_pixel(ref img: &RgbImage, pos: &Vec2) -> Vec3A {
    // Bilinear interpolation
    let iw = img.width();
    let ih = img.height();
    let w = iw as f32;
    let h = ih as f32;
    let xf = pos.x * w;
    assert!(xf >= 0.0, "xf should not underflow 0");
    let yf = pos.y * h;
    assert!(yf >= 0.0, "yf should not underflow 0");
    let x0 = xf as u32;
    let y0 = yf as u32;
    let x1 = (x0 + 1) % iw;
    let y1 = (y0 + 1) % ih;

    let top_left = img.get_pixel(x0, y0);
    let top_right = img.get_pixel(x1, y0);
    let bot_left = img.get_pixel(x0, y1);
    let bot_right = img.get_pixel(x1, y1);

    let dx0 = xf - x0 as f32;
    let dy0 = yf - y0 as f32;
    let dx1 = 1.0 - dx0;
    let dy1 = 1.0 - dy0;

    let mut res = Vec3A::ZERO;
    for dim in 0..3 {
        let top = top_left[dim] as f32 * dx1 / 255.0 + top_right[dim] as f32 * dx0 / 255.0;
        let bot = bot_left[dim] as f32 * dx1 / 255.0 + bot_right[dim] as f32 * dx0 / 255.0;
        res[dim] = top * dy1 + bot * dy0;
    }

    res
}

pub fn load_texture(path: &str, target_width: u32) -> RgbImage {
    let l = ImageReader::open(path);
    if l.is_err() {
        panic!("Problem loading the file: {:?}", l.err());
    }
    let decoded = l.unwrap().decode();
    if decoded.is_err() {
        panic!("Problem loading the file: {:?}", decoded.err());
    }
    let unwrapped = decoded.unwrap();
    if target_width > 0 {
        let w = unwrapped.width();
        let h = unwrapped.height();
        let resized_width = std::cmp::min(target_width, w);
        let resized_height = resized_width * h / w;
        return unwrapped.resize(resized_width, resized_height, FilterType::Gaussian).to_rgb8()
    }
    unwrapped.to_rgb8()
}

