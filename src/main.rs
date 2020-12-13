use minifb::{Key, Window, WindowOptions};

mod buffer;

use crate::buffer::color::Color;
use crate::buffer::math::mat4::Mat4;
use crate::buffer::math::mat4::ProjectionMatrix;
use crate::buffer::math::mat4::WorldMatrix;
use crate::buffer::mesh::*;
use crate::buffer::Savable;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    let proj: Mat4 = Mat4::create_perspective(90., 3. / 3., 0.1, 1000.);
    
    let mut cam_y = -5.;
    let world: Mat4 = Mat4::set_lookat([0., cam_y, 15.], [0., 0., 0.], [0., 1., 0.]);
    let mut buf: buffer::Buffer = buffer::Buffer::new(WIDTH, HEIGHT, proj, world);
    buf.clear_color(Color { r: 0, g: 0, b: 0 });
    buf.clear_depth(1000.);

    // Cube
    buf.translate([-2., 0., 0.]);
    let mut cube: Mesh = Mesh::construct();
    <Mesh as Cube>::new(&mut cube);
    cube.render(&mut buf);

    // Sphere
    buf.scale([0.6, 0.6, 0.6]);
    let mut sphere: Mesh = Mesh::construct();
    <Mesh as Sphere>::new(&mut sphere, 18, 13);
    sphere.render(&mut buf);

    // Cone
    buf.rotate(-180., [1., 0., 0.]);
    buf.translate([2.1, 0.5, 0.]);
    let mut cone: Mesh = Mesh::construct();
    <Mesh as Cone>::new(&mut cone, 12, 0.7, 1.);
    cone.render(&mut buf);

    // Torus
    //buf.rotate(90., [1., 0., 0.]);
    buf.scale([0.5, 0.5, 0.5]);
    let mut torus: Mesh = Mesh::construct();
    <Mesh as Torus>::new(&mut torus, 4, 10, 0.5, 2.);
    torus.render(&mut buf);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        if window.is_key_down(Key::Q) {
           cam_y -= 0.1; 
        }
        window
            .update_with_buffer(&buf.data_as_u32_vec(), WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }

    buf.save_to_png("image.png");
}
