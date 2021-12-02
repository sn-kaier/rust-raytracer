#[cfg(test)]
mod geometry_test {
    use glam::Vec3A;
    use crate::geometry::ray::Ray;
    use crate::geometry::sphere::Sphere;
    use crate::geometry::traceable::Traceable;
    use crate::scene::material::Material;

    #[test]
    fn sphere_intersects_ray() {
        let ray = Ray {
            dir: Vec3A::new(1.0, 1.0, 0.0).normalize(),
            org: Vec3A::new(0.0, 0.0, 0.0),
        };

        let sphere = Sphere::create(Vec3A::new(4.0, 1.0, 0.0), 3.5, Material::create(Vec3A::ONE, 0.1));
        let mut t: f32 = 0.0;
        let intersects = sphere.intersect(&ray, &mut t);
        let intersection_point = ray.point_at(t);
        assert_eq!(intersects, true);
        assert_eq!(t > 0.0, true);
        assert_eq!(intersection_point.length() > 0.0, true);
    }
}