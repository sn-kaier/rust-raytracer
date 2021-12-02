pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pixels: Vec<i64>
}

impl FrameBuffer {
    pub fn set_pixel(&mut self, x: i32, y: i32, color: i64) {
        self.pixels[(x + y * self.width) as usize] = color;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> i64 {
        return self.pixels[(x + y * self.width) as usize];
    }
}

pub fn create_frame_buffer(width: i32, height: i32) -> FrameBuffer {
    let pixels: Vec<i64> = Vec::with_capacity((width * height) as usize);
    return FrameBuffer {
        width,
        height,
        pixels
    }
}