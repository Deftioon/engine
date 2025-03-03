use std::collections::HashMap;

pub struct Camera {
    pub pos: super::object::Point,
    pub fov: f32,
    pub width: usize,
    pub height: usize,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        Camera {
            pos: super::object::Point {
                x: 0.0,
                y: 0.0,
                z: -50.0,
            },
            fov: 60.0,
            width,
            height,
        }
    }
}

pub struct Space {
    pub view: crate::window::View,
    objects: HashMap<usize, super::object::Object>,
    camera: Camera,
}

impl Space {
    pub fn new(width: usize, height: usize, fps: usize) -> Self {
        let view = crate::window::View::new(width, height, fps);
        let objects = HashMap::new();
        let camera = Camera::new(width, height);

        Self {
            view,
            objects,
            camera,
        }
    }

    pub fn add_object(&mut self, object: super::object::Object) {
        self.objects.insert(object.id, object);
    }

    pub fn add_sphere(&mut self, x: f32, y: f32, z: f32, radius: f32, res: f32) -> usize {
        let id = self.objects.len();
        let pos = super::object::Point { x, y, z };
        let object = super::object::Object::new_sphere(id, radius, pos, res);
        self.add_object(object);
        id
    }

    pub fn add_cube(&mut self, x: f32, y: f32, z: f32, size: f32) -> usize {
        let id = self.objects.len();
        let pos = super::object::Point { x, y, z };
        let object = super::object::Object::new_cube(id, size, pos);
        self.add_object(object);
        id
    }

    pub fn rotate_object(&mut self, id: usize, x_angle: f32, y_angle: f32, z_angle: f32) {
        if let Some(obj) = self.objects.get_mut(&id) {
            if x_angle != 0.0 {
                obj.rotate_x(x_angle);
            }
            if y_angle != 0.0 {
                obj.rotate_y(y_angle);
            }
            if z_angle != 0.0 {
                obj.rotate_z(z_angle);
            }
        }
    }

    pub fn rotate_all(&mut self, x_angle: f32, y_angle: f32, z_angle: f32) {
        for (_, obj) in self.objects.iter_mut() {
            if x_angle != 0.0 {
                obj.rotate_x(x_angle);
            }
            if y_angle != 0.0 {
                obj.rotate_y(y_angle);
            }
            if z_angle != 0.0 {
                obj.rotate_z(z_angle);
            }
        }
    }

    pub fn update(&mut self) {
        // Clear the buffer
        self.view.buffer.fill(0);

        for object in &self.objects {
            let color = super::shader::Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            };

            let buffer = super::shader::Shader::render(&object.1, &color, &self.camera);

            // Make sure we're copying to the correct buffer size
            for (i, color) in buffer.iter().enumerate() {
                if i < self.view.buffer.len() {
                    self.view.buffer[i] = super::shader::Shader::blend(self.view.buffer[i], *color);
                }
            }
        }

        self.view.update();
    }
}
