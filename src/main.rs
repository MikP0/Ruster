use minifb::{Key, Window, WindowOptions, MouseMode};

mod buffer;

use crate::buffer::color::Color;
use crate::buffer::math::mat4::Mat4;
use crate::buffer::math::mat4::ProjectionMatrix;
use crate::buffer::math::mat4::WorldMatrix;
use crate::buffer::math::slice_ops::*;
use crate::buffer::math::vec3::Vec3;
use crate::buffer::math::vector::VecOps;
use crate::buffer::mesh::*;

// Consts
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let proj: Mat4 = Mat4::create_perspective(45., (WIDTH / HEIGHT) as f32, 0.1, 1000.);

    let speed = 2.0;
    let mut cam_y = 0.;
    let mut cam_x = -5.;
    let mut cam_z = 0.;
    let mut first_mouse = true;
    let mut last_x = WIDTH as f32/2.;
    let mut last_y = HEIGHT as f32/2.;

    let mut yaw = -90.;
    let mut pitch = 0.;

    let mut window = Window::new(
        "Ruster",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut camera_pos: Vec3 = [cam_x, cam_y, cam_z];
        let mut camera_front: Vec3 = [0., 0., -1.];
        let mut camera_up: Vec3 = [0., 1., 0.];

        let (mut x_pos, mut y_pos) = window.get_mouse_pos(MouseMode::Clamp).unwrap();
        if first_mouse {
            last_x = x_pos;
            last_y = y_pos;
            first_mouse = false;
        }

        let mut x_offset = x_pos - last_x;
        let mut y_offset = last_y - y_pos;
        last_x = x_pos;
        last_y = y_pos;

        let sensitivity = 0.3;
        x_offset *= sensitivity;
        y_offset *= sensitivity;

        yaw   += x_offset;
        pitch += y_offset;

        if pitch > 89. {
            pitch = 89.;
        }
        if pitch < -89. {
            pitch = -89.;
        }

        let direction = [
        f32::cos(yaw * std::f32::consts::PI / 180.) * f32::cos(pitch * std::f32::consts::PI / 180.),
        f32::sin(pitch * std::f32::consts::PI / 180.),
        f32::sin(yaw * std::f32::consts::PI / 180.) * f32::cos(pitch * std::f32::consts::PI / 180.)];

        camera_front = Vec3::normalize(&camera_front, direction);

        if window.is_key_down(Key::S) {
            cam_x -= speed * camera_front[0];
            cam_y -= speed * camera_front[1];
            cam_z -= speed * camera_front[2];
        }

        if window.is_key_down(Key::W) {
            cam_x += speed * camera_front[0];
            cam_y += speed * camera_front[1];
            cam_z += speed * camera_front[2];
        }

        if window.is_key_down(Key::A) {
            let normalized = Vec3::normalize(&camera_up, camera_pos.cross(camera_front, camera_up));
            cam_x -= speed * normalized[0];
            cam_y -= speed * normalized[1];
            cam_z -= speed * normalized[2];
        }

        if window.is_key_down(Key::D) {
            let normalized = Vec3::normalize(&camera_up, camera_pos.cross(camera_front, camera_up));
            cam_x += speed * normalized[0];
            cam_y += speed * normalized[1];
            cam_z += speed * normalized[2];
        }

        let world: Mat4 = Mat4::set_lookat(
            camera_pos,
            [
                camera_front[0] - camera_pos[0],
                camera_front[1] - camera_pos[1],
                camera_front[2] - camera_pos[2],
            ],
            camera_up,
        );
        let mut buf: buffer::Buffer = buffer::Buffer::new(WIDTH, HEIGHT, proj, world);
        buf.clear_color(Color { r: 0, g: 0, b: 0 });
        buf.clear_depth(1000.);

        // Cube
        //buf.translate([-2., 0., 0.]);
        // let mut cube: Mesh = Mesh::construct();
        // <Mesh as Cube>::new(&mut cube);
        // cube.render(&mut buf);

        // Sphere
        //buf.scale([0.6, 0.6, 0.6]);
        let mut sphere: Mesh = Mesh::construct();
        <Mesh as Sphere>::new(&mut sphere, 18, 13);
        sphere.render(&mut buf);

        // // Cone
        // buf.rotate(-180., [1., 0., 0.]);
        // buf.translate([2.1, 0.5, 0.]);
        // let mut cone: Mesh = Mesh::construct();
        // <Mesh as Cone>::new(&mut cone, 12, 0.7, 1.);
        // cone.render(&mut buf);

        // // Torus
        // //buf.rotate(90., [1., 0., 0.]);
        // buf.scale([0.5, 0.5, 0.5]);
        // let mut torus: Mesh = Mesh::construct();
        // <Mesh as Torus>::new(&mut torus, 4, 10, 0.5, 2.);
        // torus.render(&mut buf);

        window
            .update_with_buffer(&buf.data_as_u32_vec(), WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}
