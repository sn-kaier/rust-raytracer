use glam::{Mat3A, Vec3A, Vec3};
use crate::geometry::ray::Ray;

pub struct Camera {
    pub org: Vec3A,
    pub dir: Vec3A,
    pub zoom: f32,
    pub screen_dist: f32,
}

impl Camera {
    pub fn change_zoom(&mut self, change: f32) {
        self.zoom = self.zoom * change;
    }

    pub fn turn(&mut self, p0: isize, p1: isize) {
        let inverse_speed = 800.0;
        let rot_horizontal = Mat3A::from_rotation_y(p0 as f32 / inverse_speed);
        let rot = Mat3A::from_axis_angle(Vec3::from(self.dir.cross(Vec3A::Y)), p1 as f32 / inverse_speed) * rot_horizontal;
        self.dir = (rot * self.dir).normalize();
    }

    fn go_x(&mut self, dist: f32) {
        let left = Vec3A::Y.cross(self.dir).normalize();
        self.org += left * -dist;
    }

    pub fn go_left(&mut self) {
        self.go_x(-1.0);
    }

    pub fn go_right(&mut self) {
        self.go_x(1.0);
    }
    pub fn go_forward(&mut self) {
        self.org += self.dir;
    }

    pub fn go_backward(&mut self) {
        self.org -= self.dir;
    }

    pub fn render<T: FnMut(&Ray)>(&self, width: i32, height: i32, mut trace: T) {
        let screen_center = self.org + self.dir * self.screen_dist;
        let to_left = self.dir.cross(Vec3A::Y).normalize();
        let to_top = self.dir.cross(to_left).normalize();

        let wh = width / 2;
        let hh = height / 2;

        let hf = height as f32 / self.zoom;

        for y in -hh..hh {
            let dy = y as f32 * 2.0 / hf;
            let top_offset = dy * to_top;
            for x in -wh..wh {
                let dx = x as f32 * 2.0 / hf;
                let sp = screen_center + dx * to_left + top_offset;
                let dir = (sp - self.org).normalize();
                trace(&Ray { org: sp, dir });
            }
        }
    }
}
