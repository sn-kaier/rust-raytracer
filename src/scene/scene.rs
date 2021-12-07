use std::f32::consts::PI;
use std::rc::Rc;
use glam::Vec3A;
use crate::geometry::ray::Ray;
use crate::geometry::sphere::Sphere;
use crate::geometry::traceable::Traceable;
use crate::scene::camera::Camera;
use crate::scene::light::Light;
use crate::scene::material::Material;
use crate::scene::texture::{get_pixel, load_texture};

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Box<dyn Traceable>>,
    lights: Vec<Light>,
    pub width: i32,
    pub height: i32,
    sky: Option<Sphere>,
}

pub fn create_test_scene(scene: &mut Scene) {
    let mat_mirror = Material::create(Vec3A::new(0.3, 0.3, 0.3), 0.95);
    // let mat_blue = Material::create(Vec3A::new(0.3, 0.3, 1.0), 0.6);
    let mat_green = Material::create(Vec3A::new(0.2, 1.0, 0.1), 0.3);
    // let mat_mirror = Material::create(Vec3A::new(0.2, 1.0, 0.1), 1.0);

    let mat_bricks = Rc::new(Material {
        color: Vec3A::new(1.0, 1.0, 1.0),
        reflect: 0.2,
        texture: Some(Box::new(load_texture("assets/stone_wall/baseColor.png", 1024))),
        normal_map: Some(Box::new(load_texture("assets/stone_wall/normal.png", 1024))),
    });

    // let magic_material = Rc::new(Material {
    //     color: Vec3A::new(1.0, 1.0, 1.0),
    //     reflect: 0.05,
    //     texture: Some(Box::new(load_texture("assets/magic_stone/emissive.png"))),
    //     normal_map: Some(Box::new(load_texture("assets/magic_stone/normal.png"))),
    // });

    let magic_reflector = Rc::new(Material {
        color: Vec3A::new(0.1, 0.1, 0.1),
        reflect: 0.7,
        texture: None,
        normal_map: Some(Box::new(load_texture("assets/stone_wall/normal.png", 1024))),
    });

    let stone_castle = Rc::new(Material {
        color: Vec3A::new(1.0, 1.0, 1.0),
        reflect: 0.05,
        texture: Some(Box::new(load_texture("assets/stone_castle/baseColor.png", 1024))),
        normal_map: Some(Box::new(load_texture("assets/stone_castle/normal.png", 1024))),
    });

    scene.add_sphere(Sphere::create(Vec3A::new(6.0, 0.0, 16.0), 3.0, mat_bricks));
    // scene.add_sphere(Sphere::create(Vec3A::new(-6.0, 0.5, 19.0), 3.0, mat_red.clone()));
    scene.add_sphere(Sphere::create(Vec3A::new(-6.0, 0.0, 16.0), 3.0, stone_castle.clone()));
    // scene.add_sphere(Sphere::create(Vec3A::new(0.0, -0.5, 22.0), 1.5, mat_blue.clone()));
    // scene.add_sphere(Sphere::create(Vec3A::new(20.0, 0.0, 0.0), 2.5, mat_blue.clone()));
    // scene.add_sphere(Sphere::create(Vec3A::new(-20.0, 0.0, 0.0), 0.5, mat_green));
    scene.add_sphere(Sphere::create(Vec3A::new(0.0, 0.0, 22.0), 3.0, mat_mirror.clone()));
    scene.add_sphere(Sphere::create(Vec3A::new(0.0, -6.0, 16.0), 3.0, mat_green.clone()));
    scene.add_sphere(Sphere::create(Vec3A::new(-3.0, 6.0, 12.0), 4.0, magic_reflector.clone()));

    scene.add_light(Light {
        dir: Vec3A::new(0., 1., 1.).normalize(),
        direction_sensitivity: 0.0,
        color: Vec3A::new(1.0, 1.0, 1.0) * 2.0,
        org: Vec3A::new(0.0, 10.0, -10.0),
    });
    // scene.add_light(Light {
    //     dir: Vec3A::new(0., 1., 1.).normalize(),
    //     direction_sensitivity: 0.0,
    //     color: Vec3A::new(1.0, 1.0, 1.0) * 1.5,
    //     org: Vec3A::new(0.0, 10.0, 0.0),
    // });
    // scene.add_light(Light {
    //     dir: Vec3A::new(0., -1., 0.).normalize(),
    //     direction_sensitivity: 0.3,
    //     color: Vec3A::new(1.0, 0.0, 0.0) * 1.0,
    //     org: Vec3A::new(20.0, 20.0, 10.0),
    // });
    // scene.add_light(Light {
    //     dir: Vec3A::new(0., 1., 1.).normalize(),
    //     direction_sensitivity: 0.3,
    //     color: Vec3A::new(0.2, 0.2, 1.0) * 5.0,
    //     org: Vec3A::new(0.0, -20.0, 10.0),
    // });
}

impl Scene {
    fn find_collision(&self, ray: &Ray) -> Option<(f32, &Box<dyn Traceable>)> {
        let epsilon = 0.00001;
        let mut t_min = f32::MAX;
        let mut obj: Option<&Box<dyn Traceable>> = None;
        for collision_obj in self.objects.iter() {
            let mut t: f32 = 0.0;
            if collision_obj.intersect(&ray, &mut t) {
                if t < t_min && t > epsilon {
                    t_min = t;
                    obj = Some(collision_obj);
                }
            }
        }
        if let Some(collision_obj) = obj {
            return Some((t_min, collision_obj));
        }
        return None;
    }

    fn shoot_ray(&self, ray: &Ray, iterations: i32) -> Vec3A {
        if iterations <= 0 {
            return Vec3A::ZERO;
        }
        if let Some((t, collision_obj)) = self.find_collision(ray) {
            let collision = ray.point_at(t);
            // normal of the object at the ray
            let tex_coord = collision_obj.get_texture_coord(&collision);

            let (mut normal, mut reflection) = collision_obj.intersection_normal(&ray, collision);
            let mat = collision_obj.get_mat();

            // check normal map
            if let Some(normal_map) = &mat.normal_map {
                let normal_pixel = get_pixel(normal_map, &tex_coord);
                let l = (normal_pixel - Vec3A::new(0.5, 0.5, 0.5)).normalize();

                // coord system from the normal
                let to_right = normal.cross(-Vec3A::Y);
                let to_up = normal.cross(-to_right);

                normal = l.z * normal + to_up * l.y + to_right * l.x;
                reflection = l.z * reflection + to_up * l.y + to_right * l.x;
            }

            let angle = normal.angle_between(-ray.dir) / PI;
            let angle_comp = f32::max(0.5 - angle, 0.0) / 0.5;
            // use the angle to calc the ambient light

            // texture
            let color = if let Some(texture) = &mat.texture {
                get_pixel(texture, &tex_coord)
            } else {
                mat.color
            };
            let ambient_color = color * angle_comp;

            let mut light_color = Vec3A::ZERO;
            // shoot towards lights
            for light in self.lights.iter() {
                let dir_to_light = (light.org - collision).normalize();
                let ray_to_light = Ray {
                    org: collision,
                    dir: dir_to_light,
                };
                if let Some(_) = self.find_collision(&ray_to_light) {
                    // blocked -> no light
                } else {
                    // goes through to light
                    let angle_light_dir_ray = light.dir.angle_between(-dir_to_light) / PI;
                    let dir_angle_comp = light.direction_sensitivity * f32::max(0.5 - angle_light_dir_ray, 0.0) * 2.0;

                    let specular = 0.1;
                    let specular_reflection = f32::max(specular - dir_to_light.angle_between(reflection) / PI, 0.0) / specular;

                    let diffusion_angle = normal.angle_between(dir_to_light) / PI;
                    let diffusion = 0.5;
                    let diffusion_comp = f32::max(diffusion - diffusion_angle, 0.0) / diffusion;

                    let ambient_component = 1.0 - light.direction_sensitivity;
                    let clr = light.color * ((dir_angle_comp + ambient_component) * diffusion_comp + specular_reflection * 0.1);

                    light_color.x += clr.x * color.x;
                    light_color.y += clr.y * color.y;
                    light_color.z += clr.z * color.z;
                }
            }

            // reflect ray
            let refection_ray = Ray {
                org: collision,
                dir: reflection,
            };
            let shot = self.shoot_ray(&refection_ray, iterations - 1);

            let non_reflect = 1.0 - mat.reflect;

            return 0.3 * ambient_color * non_reflect + light_color * 0.55 * non_reflect + 1.0 * shot * mat.reflect;
        }
        if let Some(sky) = &self.sky {
            let t = sky.mat.clone();
            if let Some(texture) = &t.texture {
                let tex_coord = sky.get_texture_coord(&ray.dir);
                return get_pixel(&texture, &tex_coord);
            }
        }
        Vec3A::ZERO
    }

    pub fn render(&self, screen: &mut [u8]) {
        let mut chunks = screen.chunks_mut(4);
        self.camera.render(self.width, self.height, |ray| {
            let pixel = chunks.next().unwrap();

            let color_vec = self.shoot_ray(ray, 4);
            let color = [(color_vec.x * 255.0) as u8, (color_vec.y * 255.0) as u8, (color_vec.z * 255.0) as u8, 0xff];
            pixel.copy_from_slice(&color)
        })
    }

    pub fn create(width: i32, height: i32) -> Self {
        let sky = Sphere::create(Vec3A::ZERO, 1.0, Rc::new(Material {
            color: Vec3A::ZERO,
            reflect: 0.0,
            normal_map: None,
            texture: Some(Box::new(load_texture("assets/skybox.jpg", 1024))),
        }));
        Scene {
            camera: Camera {
                org: Vec3A::new(0.0, 5.0, -5.0),
                dir: Vec3A::Z,
                zoom: 1.5,
                screen_dist: 4.0,
            },
            width,
            height,
            objects: Vec::new(),
            lights: Vec::new(),
            sky: Some(sky),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.objects.push(Box::new(sphere));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn update(&mut self) {
        for x in self.objects.iter_mut() {
            x.update();
        }
    }
}

