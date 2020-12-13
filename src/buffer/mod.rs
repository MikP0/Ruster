extern crate image;

pub mod color;
use color::Color;

pub mod math;
use math::mat4::Mat4;
use math::matrix::Matrix;

use math::vec3::Vec3;
use math::vec4::Vec4;
use math::vector::MulVectorMatrix;
use math::vector::VecOps;
use math::vector::Vector;

pub mod pixel;
use pixel::Pixel;

pub mod mesh;
use mesh::Mesh;

pub trait Savable {
    fn save_to_png(&self, path: &str);
}

pub struct Buffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Pixel>,
    pub depth: Vec<f32>,
    pub proj: Mat4,
    pub world: Mat4,
    pub obj: Mat4,
    pub obj2proj: Mat4,
    pub obj2world: Mat4,
}

impl Buffer {
    pub fn new(_width: u32, _height: u32, _proj: Mat4, _world: Mat4) -> Buffer {
        Buffer {
            width: _width,
            height: _height,
            data: Vec::new(),
            depth: Vec::new(),
            proj: _proj,
            world: _world,
            obj: Mat4::identity(),
            obj2proj: Mat4::identity(),
            obj2world: Mat4::identity(),
        }
    }

    pub fn clear_object_matrices(&mut self) {
        self.obj = Mat4::identity();
    }

    pub fn clear_color(&mut self, c: Color) {
        self.data.clear();

        for _ in 0..self.width {
            for _ in 0..self.height {
                self.data.push(Pixel::new(c.r, c.g, c.b));
            }
        }
    }

    pub fn clear_depth(&mut self, value: f32) {
        self.depth.clear();

        for _ in 0..self.width {
            for _ in 0..self.height {
                self.depth.push(value);
            }
        }
    }

    fn tr(&mut self, vec: Vec4) -> Vec4 {
        let result: Vec4 = vec.mul_matrix_left(&self.obj2proj);
        [
            result[0] / result[3],
            result[1] / result[3],
            result[2] / result[3],
            1.,
        ]
    }

    pub fn translate(&mut self, vec: Vec3) {
        let mut m: Mat4 = Mat4::identity();
        m[3] = vec[0];
        m[7] = vec[1];
        m[11] = vec[2];
        self.obj = m.mul(&self.obj);
    }

    pub fn scale(&mut self, vec: Vec3) {
        let mut m: Mat4 = Mat4::identity();
        m[0] = vec[0];
        m[5] = vec[1];
        m[10] = vec[2];

        self.obj = m.mul(&self.obj);
    }

    pub fn rotate(&mut self, a: f32, vec: Vec3) {
        let s = f32::sin(a * std::f32::consts::PI / 180.);
        let c = f32::cos(a * std::f32::consts::PI / 180.);
        vec.normalize(vec);

        let mut m: Mat4 = [
            vec[0] * vec[0] * (1. - c) + c,
            vec[1] * vec[0] * (1. - c) + vec[2] * s,
            vec[0] * vec[2] * (1. - c) - vec[1] * s,
            0.,
            vec[0] * vec[1] * (1. - c) - vec[2] * s,
            vec[1] * vec[1] * (1. - c) + c,
            vec[1] * vec[2] * (1. - c) + vec[0] * s,
            0.,
            vec[2] * vec[2] * (1. - c) + vec[1] * s,
            vec[1] * vec[2] * (1. - c) - vec[0] * s,
            vec[2] * vec[2] * (1. - c) + c,
            0.,
            0.,
            0.,
            0.,
            1.,
        ];

        self.obj = m.mul(&self.obj);
    }

    pub fn draw_triangle(
        &mut self,
        va: Vec3,
        vb: Vec3,
        vc: Vec3,
        mut c1: Color,
        mut c2: Color,
        mut c3: Color,
    ) {
        self.obj2world = self.world.mul(&self.obj);
        self.obj2proj = self.proj.mul(&self.obj2world);

        let mut veca: Vec4 = self.tr([va[0], va[1], va[2], 1.0]);
        let mut vecb: Vec4 = self.tr([vb[0], vb[1], vb[2], 1.0]);
        let mut vecc: Vec4 = self.tr([vc[0], vc[1], vc[2], 1.0]);

        veca = veca.scale(1. / veca[3]);
        vecb = vecb.scale(1. / vecb[3]);
        vecc = vecc.scale(1. / vecc[3]);

        let maxx = clamp(f32::max(veca[0], vecb[0]).max(vecc[0]), -1., 1.);
        let maxy = clamp(f32::max(veca[1], vecb[1]).max(vecc[1]), -1., 1.);
        let minx = clamp(f32::min(veca[0], vecb[0]).min(vecc[0]) as f32, -1., 1.);
        let miny = clamp(f32::min(veca[1], vecb[1]).min(vecc[1]) as f32, -1., 1.);

        let half_width = self.width as f32 * 0.5;
        let half_height = self.height as f32 * 0.5;

        let min_width = ((minx + 1.) * half_width) as u32;
        let max_width = ((maxx + 1.) * half_width) as u32;
        let min_height = ((miny + 1.) * half_height) as u32;
        let max_height = ((maxy + 1.) * half_height) as u32;

        let dxab = veca[0] - vecb[0];
        let dxbc = vecb[0] - vecc[0];
        let dxca = vecc[0] - veca[0];
        let dyab = veca[1] - vecb[1];
        let dybc = vecb[1] - vecc[1];
        let dyca = vecc[1] - veca[1];

        let c1_n = c1.normalize();
        let c2_n = c2.normalize();
        let c3_n = c3.normalize();

        let l1d = 1. / (-dybc * dxca + dxbc * dyca);
        let l2d = 1. / (dyca * dxbc - dxca * dybc);

        let topleft1 = dyab < 0. || (dyab == 0. && dxab > 0.);
        let topleft2 = dybc < 0. || (dybc == 0. && dxbc > 0.);
        let topleft3 = dyca < 0. || (dyca == 0. && dxca > 0.);

        for h in min_width..max_width + 1 {
            let x: f32 = ((h as f32) / half_width) - 1.;
            for w in min_height..max_height + 1 {
                let y: f32 = ((w as f32) / half_height) - 1.;

                let l1 = (dybc * (x - vecc[0]) - dxbc * (y - vecc[1])) * l1d;
                let l2 = (dyca * (x - vecc[0]) - dxca * (y - vecc[1])) * l2d;

                let l3: f32 = 1.0 - l1 - l2;

                let rc: f32 = l1 * c1_n.0 + l2 * c2_n.0 + l3 * c3_n.0;
                let gc: f32 = l1 * c1_n.1 + l2 * c2_n.1 + l3 * c3_n.1;
                let bc: f32 = l1 * c1_n.2 + l2 * c2_n.2 + l3 * c3_n.2;

                let base = h + self.width * w;

                let depth = l1 * veca[2] as f32 + l2 * vecb[2] as f32 + l3 * vecc[2] as f32;

                let inside = ((((dxab) * (y as f32 - veca[1])) - ((dyab) * (x as f32 - veca[0]))
                    > 0.)
                    && !topleft1
                    || (((dxab) * (y as f32 - veca[1])) - ((dyab) * (x as f32 - veca[0])) >= 0.)
                        && topleft1)
                    && ((((dxbc) * (y as f32 - vecb[1])) - ((dybc) * (x as f32 - vecb[0])) > 0.)
                        && !topleft2
                        || (((dxbc) * (y as f32 - vecb[1])) - ((dybc) * (x as f32 - vecb[0]))
                            >= 0.)
                            && topleft2)
                    && ((((dxca) * (y as f32 - vecc[1])) - ((dyca) * (x as f32 - vecc[0])) > 0.)
                        && !topleft3
                        || (((dxca) * (y as f32 - vecc[1])) - ((dyca) * (x as f32 - vecc[0]))
                            >= 0.)
                            && topleft3);

                let on_top = depth < self.depth[base as usize];

                if inside && on_top {
                    self.data[base as usize] =
                        Pixel::new((rc * 255.0) as u8, (gc * 255.0) as u8, (bc * 255.0) as u8);
                    self.depth[base as usize] = depth;
                }
            }
        }
    }

    fn data_as_u8_vec(&self) -> Vec<u8> {
        let mut u8_vec = Vec::<u8>::new();
        for el in &self.data {
            u8_vec.push(el.color.r);
            u8_vec.push(el.color.g);
            u8_vec.push(el.color.b);
        }
        u8_vec
    }

    pub fn data_as_u32_vec(&self) -> Vec<u32> {
        let mut u32_vec = Vec::<u32>::new();
        for el in &self.data {
            u32_vec.push(el.color.r as u32);
            u32_vec.push(el.color.g as u32);
            u32_vec.push(el.color.b as u32);
        }
        u32_vec
    }
}

impl Savable for Buffer {
    fn save_to_png(&self, path: &str) {
        image::save_buffer(
            path,
            self.data_as_u8_vec().as_slice(),
            self.width,
            self.height,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
