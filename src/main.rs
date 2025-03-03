use engine;
use std::time::{Duration, Instant};

fn main() {
    let mut space = engine::space(800, 600, 60);
    let sphere_id = space.add_sphere(50.0, 0.0, 100.0, 30.0, 20.0);
    let cube_id = space.add_cube(0.0, 50.0, 300.0, 40.0);
    let another_sphere_id = space.add_sphere(-50.0, 20.0, 120.0, 10.0, 15.0);

    let mut last_update = Instant::now();

    while space.view.window.is_open() && !space.view.window.is_key_down(engine::window::Key::Escape)
    {
        // Calculate elapsed time for smooth animation
        let now = Instant::now();
        let elapsed = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Rotate the sphere - adjust rotation speeds as desired
        engine::rotate_object(
            &mut space,
            sphere_id,
            0.2 * elapsed,
            0.3 * elapsed,
            0.1 * elapsed,
        );

        engine::rotate_object(
            &mut space,
            cube_id,
            0.1 * elapsed,
            -0.5 * elapsed,
            0.2 * elapsed,
        );

        engine::rotate_object(
            &mut space,
            another_sphere_id,
            -0.1 * elapsed,
            -0.5 * elapsed,
            0.2 * elapsed,
        );

        space.update();

        // Add a small delay to limit CPU usage
        std::thread::sleep(Duration::from_millis(10));
    }
}
