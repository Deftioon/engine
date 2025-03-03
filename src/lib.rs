pub mod engine;
pub mod window;

pub fn point(x: f32, y: f32, z: f32) -> engine::object::Point {
    engine::object::Point { x, y, z }
}

pub fn space(width: usize, height: usize, fps: usize) -> engine::space::Space {
    engine::space::Space::new(width, height, fps)
}

pub fn rotate_object(space: &mut engine::space::Space, id: usize, x: f32, y: f32, z: f32) {
    space.rotate_object(id, x, y, z);
}

pub fn rotate_all(space: &mut engine::space::Space, x: f32, y: f32, z: f32) {
    space.rotate_all(x, y, z);
}
