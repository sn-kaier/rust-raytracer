use glam::Vec3A;

#[derive(Debug)]
pub struct Ray {
    pub org: Vec3A,
    pub dir: Vec3A,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3A {
        self.org + self.dir * t
    }
}
