use glam::Vec3A;

pub struct Light {
    pub org: Vec3A,
    pub dir: Vec3A,
    /**
    between 0 and 1. 1: only a point
     */
    pub direction_sensitivity: f32,
    pub color: Vec3A,
}

