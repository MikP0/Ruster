use crate::buffer::color::Color;
use crate::buffer::pixel::Pixel;
use crate::buffer::Buffer;

use crate::buffer::math::int3::Int3;
use crate::buffer::math::mat4::Mat4;
use crate::buffer::math::matrix::Matrix;
use crate::buffer::math::vec3::Vec3;
use crate::buffer::math::vec4::Vec4;
use crate::buffer::math::vector::MulVectorMatrix;
use crate::buffer::math::vector::VecOps;
use crate::buffer::math::vector::Vector;

#[derive(Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

pub struct Mesh {
    pub v_size: u32,
    pub t_size: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Int3>,
}

impl Mesh {
    pub fn construct() -> Mesh {
        Mesh {
            v_size: 0,
            t_size: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

pub trait Triangle {
    fn new(&mut self);
}

pub trait Cube {
    fn new(&mut self);
}

pub trait Sphere {
    fn new(&mut self, vert: u32, horiz: u32);
}

pub trait Render {
    fn render(&mut self, buf: &mut Buffer);
}

pub trait Cone {
    fn new(&mut self, vert: u32, r: f32, h: f32);
}

pub trait Torus {
    fn new(&mut self, sides: u32, cs_sides: u32, radius: f32, cs_radius: f32);
}

impl Render for Mesh {
    fn render(&mut self, buf: &mut Buffer) {
        for i in 0..self.t_size {
            buf.draw_triangle(
                self.vertices[self.indices[i as usize][0] as usize].position,
                self.vertices[self.indices[i as usize][1] as usize].position,
                self.vertices[self.indices[i as usize][2] as usize].position,
                Color { r: 255, g: 0, b: 0 },
                Color { r: 0, g: 255, b: 0 },
                Color { r: 0, g: 0, b: 255 },
            )
        }
        buf.clear_object_matrices();
    }
}

impl Triangle for Mesh {
    fn new(&mut self) {
        self.v_size = 3;
        self.t_size = 1;
        self.vertices = vec![
            Vertex {
                position: [0., 0., 0.],
                normal: [0., 0., 0.]
            };
            self.v_size as usize
        ];
        self.indices = vec![[0, 0, 0]; self.t_size as usize];

        self.vertices[0].position = [-0.5, 0., 0.];
        self.vertices[1].position = [0., 0.5, 0.];
        self.vertices[2].position = [0.5, 0., 0.];

        self.indices[0] = [0, 1, 2];
    }
}

impl Cube for Mesh {
    fn new(&mut self) {
        self.v_size = 8;
        self.t_size = 12;
        self.vertices = vec![
            Vertex {
                position: [0., 0., 0.],
                normal: [0., 0., 0.]
            };
            self.v_size as usize
        ];
        self.indices = vec![[0, 0, 0]; self.t_size as usize];

        self.vertices[0].position = [-0.5, 0.5, 0.5];
        self.vertices[1].position = [0.5, 0.5, 0.5];
        self.vertices[2].position = [0.5, -0.5, 0.5];
        self.vertices[3].position = [-0.5, -0.5, 0.5];
        self.vertices[4].position = [-0.5, 0.5, -0.5];
        self.vertices[5].position = [0.5, 0.5, -0.5];
        self.vertices[6].position = [0.5, -0.5, -0.5];
        self.vertices[7].position = [-0.5, -0.5, -0.5];

        self.indices[0] = [0, 1, 3];
        self.indices[1] = [1, 2, 3];
        self.indices[2] = [4, 0, 3];
        self.indices[3] = [7, 4, 3];
        self.indices[4] = [2, 1, 5];
        self.indices[5] = [2, 5, 6];
        self.indices[6] = [0, 4, 5];
        self.indices[7] = [5, 1, 0];
        self.indices[8] = [6, 7, 3];
        self.indices[9] = [3, 2, 6];
        self.indices[10] = [7, 5, 4];
        self.indices[11] = [7, 6, 5];
    }
}

impl Sphere for Mesh {
    fn new(&mut self, vert: u32, horiz: u32) {
        self.v_size = vert * (horiz + 2);
        self.t_size = 2 * vert * horiz;
        self.vertices = vec![
            Vertex {
                position: [0., 0., 0.],
                normal: [0., 0., 0.]
            };
            self.v_size as usize
        ];
        self.indices = vec![[0, 0, 0]; self.t_size as usize];

        for yy in 0..horiz + 2 {
            let y = f32::cos(yy as f32 * std::f32::consts::PI / (horiz as f32 + 1.));
            let r = f32::sqrt(1. - (y * y));
            for rr in 0..vert {
                let x = r * f32::cos(2. * std::f32::consts::PI * rr as f32 / vert as f32);
                let z = r * f32::sin(2. * std::f32::consts::PI * rr as f32 / vert as f32);
                self.vertices[(rr + yy * vert) as usize].position = [x, y, z];
            }
        }

        for yy in 0..horiz {
            for rr in 0..vert {
                self.indices[(rr + 2 * yy * vert) as usize] = [
                    (rr + 1) % vert + yy * vert,
                    rr + vert + yy * vert,
                    (rr + 1) % vert + vert + yy * vert,
                ];
                self.indices[(rr + vert + 2 * yy * vert) as usize] = [
                    rr + vert + yy * vert,
                    rr + 2 * vert + yy * vert,
                    (rr + 1) % vert + vert + yy * vert,
                ];
            }
        }
    }
}

impl Cone for Mesh {
    fn new(&mut self, vert: u32, radius: f32, h: f32) {
        let horiz = 1;
        self.v_size = vert * (horiz + 2);
        self.t_size = 2 * vert * horiz;
        self.vertices = vec![
            Vertex {
                position: [0., 0., 0.],
                normal: [0., 0., 0.]
            };
            self.v_size as usize
        ];
        self.indices = vec![[0, 0, 0]; self.t_size as usize];

        for yy in 0..horiz + 1 {
            let y = f32::cos(yy as f32 * std::f32::consts::PI / (horiz as f32 + 1.));
            let r = f32::sqrt(1. - (y * y)) * radius;
            for rr in 0..vert {
                let x = r * f32::cos(2. * std::f32::consts::PI * rr as f32 / vert as f32);
                let z = r * f32::sin(2. * std::f32::consts::PI * rr as f32 / vert as f32);
                self.vertices[(rr + yy * vert) as usize].position = [x, y * h, z];
            }
        }

        for yy in 0..horiz {
            for rr in 0..vert {
                self.indices[(rr + 2 * yy * vert) as usize] = [
                    (rr + 1) % vert + yy * vert,
                    rr + vert + yy * vert,
                    (rr + 1) % vert + vert + yy * vert,
                ];
                self.indices[(rr + vert + 2 * yy * vert) as usize] = [
                    rr + vert + yy * vert,
                    rr + 2 * vert + yy * vert,
                    (rr + 1) % vert + vert + yy * vert,
                ];
            }
        }
    }
}

impl Torus for Mesh {
    fn new(&mut self, slices: u32, loops: u32, inner_rad: f32, outer_rad: f32) {
        self.vertices = Vec::new();
        self.indices = Vec::new();

        for slice in 0..slices + 1 {
            let v = slice as f32 / slices as f32;
            let slice_angle = v * 2. * std::f32::consts::PI;
            let cos_slices = f32::cos(slice_angle);
            let sin_slices = f32::sin(slice_angle);
            let slice_rad = outer_rad + inner_rad * cos_slices;

            for lp in 0..loops + 1 {
                let u = lp as f32 / loops as f32;
                let loop_angle = u * 2. * std::f32::consts::PI;
                let cos_loops = f32::cos(loop_angle);
                let sin_loops = f32::sin(loop_angle);

                let x = slice_rad * cos_loops;
                let y = slice_rad * sin_loops;
                let z = inner_rad * sin_slices;

                self.vertices.push(Vertex {
                    position: [x, y, z],
                    normal: [0., 0., 0.],
                });
            }
        }

        let vertsPerSlice = loops + 1;
        for i in 0..slices {
            let mut v1 = i * vertsPerSlice;
            let mut v2 = v1 + vertsPerSlice;
            /* outer ring */
            for j in 0..loops {
                self.indices.push([v1, v1 + 1, v2]);
                self.indices.push([v2, v1 + 1, v2 + 1]);
                v1 += 1;
                v2 += 1;
            }
        }
        self.v_size = self.vertices.len() as u32;
        self.t_size = self.indices.len() as u32;
    }
}
