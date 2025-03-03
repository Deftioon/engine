#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Shader {}

impl Shader {
    pub fn new() -> Self {
        Shader {}
    }

    pub fn render(
        object: &super::object::Object,
        color: &Color,
        cam: &super::space::Camera,
    ) -> Vec<Color> {
        let mut buffer = vec![
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            };
            cam.width * cam.height
        ];

        // Create depth buffer for Z-sorting
        let mut depth_buffer = vec![f32::INFINITY; cam.width * cam.height];

        // Convert FOV from degrees to radians
        let fov_rad = cam.fov * std::f32::consts::PI / 180.0;
        let aspect_ratio = cam.width as f32 / cam.height as f32;
        let tan_half_fov = (fov_rad / 2.0).tan();

        // Project all points to screen space and track which are visible
        let mut screen_points = Vec::with_capacity(object.points.len());
        let mut point_visible = vec![false; object.points.len()];

        for (i, point) in object.points.iter().enumerate() {
            let rel_x = point.x - cam.pos.x;
            let rel_y = point.y - cam.pos.y;
            let rel_z = point.z - cam.pos.z;

            // Early culling
            if rel_z <= 1.0 {
                screen_points.push((0, 0, 0.0)); // Dummy value
                continue;
            }

            // Frustum culling
            let half_width = rel_z * tan_half_fov * aspect_ratio;
            let half_height = rel_z * tan_half_fov;

            if rel_x.abs() > half_width || rel_y.abs() > half_height {
                screen_points.push((0, 0, 0.0)); // Dummy value
                continue;
            }

            // Project to screen space
            let screen_x =
                ((rel_x / rel_z / (tan_half_fov * aspect_ratio)) * 0.5 + 0.5) * cam.width as f32;
            let screen_y = ((rel_y / rel_z / tan_half_fov) * 0.5 + 0.5) * cam.height as f32;

            let sx = screen_x as usize;
            let sy = screen_y as usize;

            if sx < cam.width && sy < cam.height {
                point_visible[i] = true;
                screen_points.push((sx, sy, rel_z));

                // Draw the point
                let idx = sy * cam.width + sx;
                if idx < buffer.len() && rel_z < depth_buffer[idx] {
                    buffer[idx] = *color;
                    depth_buffer[idx] = rel_z;
                }
            } else {
                screen_points.push((0, 0, 0.0)); // Dummy value
            }
        }

        // Draw wireframe lines for edges
        let line_color = Color {
            r: 200,
            g: 200,
            b: 200,
            a: 255,
        };
        for edge in &object.edges {
            if point_visible[edge.start] && point_visible[edge.end] {
                let (start_x, start_y, start_z) = screen_points[edge.start];
                let (end_x, end_y, end_z) = screen_points[edge.end];

                Self::draw_line(
                    &mut buffer,
                    &mut depth_buffer,
                    cam.width,
                    cam.height,
                    start_x,
                    start_y,
                    start_z,
                    end_x,
                    end_y,
                    end_z,
                    line_color,
                );
            }
        }

        buffer
    }

    pub fn draw_line(
        buffer: &mut [Color],
        depth_buffer: &mut [f32],
        width: usize,
        height: usize,
        start_x: usize,
        start_y: usize,
        start_z: f32,
        end_x: usize,
        end_y: usize,
        end_z: f32,
        color: Color,
    ) {
        // Bresenham's line algorithm
        let dx = end_x as isize - start_x as isize;
        let dy = end_y as isize - start_y as isize;

        let abs_dx = dx.abs();
        let abs_dy = dy.abs();

        let mut x = start_x as isize;
        let mut y = start_y as isize;

        // Calculate the number of steps to take
        let steps = if abs_dx > abs_dy { abs_dx } else { abs_dy };

        if steps == 0 {
            // Single pixel
            let idx = (y as usize) * width + (x as usize);
            if idx < buffer.len() {
                let z = start_z;
                if z < depth_buffer[idx] {
                    buffer[idx] = color;
                    depth_buffer[idx] = z;
                }
            }
            return;
        }

        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;
        let z_inc = (end_z - start_z) / steps as f32;

        let mut x_f = start_x as f32;
        let mut y_f = start_y as f32;
        let mut z = start_z;

        for _ in 0..=steps {
            let px = x_f as usize;
            let py = y_f as usize;

            if px < width && py < height {
                let idx = py * width + px;
                if idx < buffer.len() && z < depth_buffer[idx] {
                    buffer[idx] = color;
                    depth_buffer[idx] = z;
                }
            }

            x_f += x_inc;
            y_f += y_inc;
            z += z_inc;
        }
    }

    pub fn blend(background: u32, rgba: Color) -> u32 {
        let r = rgba.r as u32;
        let g = rgba.g as u32;
        let b = rgba.b as u32;
        let a = rgba.a as u32;

        let r = (r * a + (255 - a) * (background & 0xFF)) / 255;
        let g = (g * a + (255 - a) * ((background >> 8) & 0xFF)) / 255;
        let b = (b * a + (255 - a) * ((background >> 16) & 0xFF)) / 255;

        (r | (g << 8) | (b << 16)) as u32
    }
}
