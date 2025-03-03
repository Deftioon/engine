#[derive(Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn multiply(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::identity();

        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        result
    }

    pub fn rotate_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix4x4 {
            data: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rotate_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix4x4 {
            data: [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Edge {
    pub start: usize, // Index of first point
    pub end: usize,   // Index of second point
}

pub struct Triangle {
    pub a: usize, // Index of first point
    pub b: usize, // Index of second point
    pub c: usize, // Index of third point
}

pub struct Object {
    pub id: usize,
    pub points: Vec<Point>,
    pub original_points: Vec<Point>,
    pub edges: Vec<Edge>,
    pub triangles: Vec<Triangle>,
    pub shader: super::shader::Shader,
    pub transform: Matrix4x4,
    pub center: Point,
}

impl Object {
    pub fn new_sphere(id: usize, radius: f32, pos: Point, res: f32) -> Self {
        let mut points = vec![];
        let mut edges = vec![];
        let mut triangles = vec![];

        // Add north pole point
        points.push(Point {
            x: pos.x,
            y: pos.y,
            z: pos.z + radius,
        });

        // Generate points in latitude rings
        for j in 1..res as usize {
            let phi = std::f32::consts::PI * j as f32 / res;
            for i in 0..res as usize {
                let theta = 2.0 * std::f32::consts::PI * i as f32 / res;
                let x = radius * theta.sin() * phi.sin() + pos.x;
                let y = radius * theta.cos() * phi.sin() + pos.y;
                let z = radius * phi.cos() + pos.z;
                points.push(Point { x, y, z });
            }
        }

        // Add south pole point
        points.push(Point {
            x: pos.x,
            y: pos.y,
            z: pos.z - radius,
        });

        // Connect north pole (index 0) with first ring
        for i in 0..res as usize {
            let next_i = (i + 1) % (res as usize);
            let current = i + 1; // +1 because index 0 is the north pole
            let next = next_i + 1;

            // Add edges from north pole
            edges.push(Edge {
                start: 0,
                end: current,
            });

            // Add triangle
            triangles.push(Triangle {
                a: 0,
                b: current,
                c: next,
            });
        }

        // Connect intermediate rings
        let ring_size = res as usize;
        for j in 0..(res as usize - 2) {
            for i in 0..ring_size {
                let current = i + 1 + j * ring_size;
                let below = i + 1 + (j + 1) * ring_size;
                let next_i = (i + 1) % ring_size;
                let next = next_i + 1 + j * ring_size;
                let below_next = next_i + 1 + (j + 1) * ring_size;

                // Add edges
                edges.push(Edge {
                    start: current,
                    end: below,
                });
                edges.push(Edge {
                    start: current,
                    end: next,
                });

                // Add triangles - two per grid cell
                triangles.push(Triangle {
                    a: current,
                    b: next,
                    c: below,
                });
                triangles.push(Triangle {
                    a: next,
                    b: below_next,
                    c: below,
                });
            }
        }

        // Connect south pole (last point) with last ring
        let south_pole_idx = points.len() - 1;
        let last_ring_start = south_pole_idx - ring_size;
        for i in 0..ring_size {
            let current = last_ring_start + i;
            let next = last_ring_start + (i + 1) % ring_size;

            // Add edges to south pole
            edges.push(Edge {
                start: current,
                end: south_pole_idx,
            });

            // Add triangle
            triangles.push(Triangle {
                a: south_pole_idx,
                b: next,
                c: current,
            });
        }

        // Store original points and center
        let original_points = points.clone();
        let center = pos;

        Self {
            id,
            points,
            original_points,
            edges,
            triangles,
            shader: super::shader::Shader::new(),
            transform: Matrix4x4::identity(),
            center,
        }
    }

    pub fn new_cube(id: usize, size: f32, pos: Point) -> Self {
        let half_size = size / 2.0;

        // Define the 8 vertices of the cube
        let mut points = vec![
            // Front face
            Point {
                x: pos.x - half_size,
                y: pos.y - half_size,
                z: pos.z + half_size,
            }, // 0: front-bottom-left
            Point {
                x: pos.x + half_size,
                y: pos.y - half_size,
                z: pos.z + half_size,
            }, // 1: front-bottom-right
            Point {
                x: pos.x + half_size,
                y: pos.y + half_size,
                z: pos.z + half_size,
            }, // 2: front-top-right
            Point {
                x: pos.x - half_size,
                y: pos.y + half_size,
                z: pos.z + half_size,
            }, // 3: front-top-left
            // Back face
            Point {
                x: pos.x - half_size,
                y: pos.y - half_size,
                z: pos.z - half_size,
            }, // 4: back-bottom-left
            Point {
                x: pos.x + half_size,
                y: pos.y - half_size,
                z: pos.z - half_size,
            }, // 5: back-bottom-right
            Point {
                x: pos.x + half_size,
                y: pos.y + half_size,
                z: pos.z - half_size,
            }, // 6: back-top-right
            Point {
                x: pos.x - half_size,
                y: pos.y + half_size,
                z: pos.z - half_size,
            }, // 7: back-top-left
        ];

        // Define the 12 edges of the cube
        let edges = vec![
            // Front face
            Edge { start: 0, end: 1 },
            Edge { start: 1, end: 2 },
            Edge { start: 2, end: 3 },
            Edge { start: 3, end: 0 },
            // Back face
            Edge { start: 4, end: 5 },
            Edge { start: 5, end: 6 },
            Edge { start: 6, end: 7 },
            Edge { start: 7, end: 4 },
            // Connecting edges
            Edge { start: 0, end: 4 },
            Edge { start: 1, end: 5 },
            Edge { start: 2, end: 6 },
            Edge { start: 3, end: 7 },
        ];

        // Define the 12 triangles (2 per face) of the cube
        let triangles = vec![
            // Front face
            Triangle { a: 0, b: 1, c: 2 },
            Triangle { a: 0, b: 2, c: 3 },
            // Back face
            Triangle { a: 5, b: 4, c: 7 },
            Triangle { a: 5, b: 7, c: 6 },
            // Left face
            Triangle { a: 4, b: 0, c: 3 },
            Triangle { a: 4, b: 3, c: 7 },
            // Right face
            Triangle { a: 1, b: 5, c: 6 },
            Triangle { a: 1, b: 6, c: 2 },
            // Top face
            Triangle { a: 3, b: 2, c: 6 },
            Triangle { a: 3, b: 6, c: 7 },
            // Bottom face
            Triangle { a: 4, b: 5, c: 1 },
            Triangle { a: 4, b: 1, c: 0 },
        ];

        // Store original points and center
        let original_points = points.clone();

        Self {
            id,
            points,
            original_points,
            edges,
            triangles,
            shader: super::shader::Shader::new(),
            transform: Matrix4x4::identity(),
            center: pos,
        }
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.transform = self.transform.multiply(&Matrix4x4::rotate_x(angle));
        self.apply_transform();
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.transform = self.transform.multiply(&Matrix4x4::rotate_y(angle));
        self.apply_transform();
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.transform = self.transform.multiply(&Matrix4x4::rotate_z(angle));
        self.apply_transform();
    }

    fn apply_transform(&mut self) {
        // Reset points to original positions
        self.points = self.original_points.clone();

        // Apply the transformation matrix to each point
        for i in 0..self.points.len() {
            // Translate point to origin
            let mut x = self.points[i].x - self.center.x;
            let mut y = self.points[i].y - self.center.y;
            let mut z = self.points[i].z - self.center.z;

            // Apply the transformation matrix
            let new_x = self.transform.data[0][0] * x
                + self.transform.data[0][1] * y
                + self.transform.data[0][2] * z;
            let new_y = self.transform.data[1][0] * x
                + self.transform.data[1][1] * y
                + self.transform.data[1][2] * z;
            let new_z = self.transform.data[2][0] * x
                + self.transform.data[2][1] * y
                + self.transform.data[2][2] * z;

            // Translate back
            self.points[i].x = new_x + self.center.x;
            self.points[i].y = new_y + self.center.y;
            self.points[i].z = new_z + self.center.z;
        }
    }
}
