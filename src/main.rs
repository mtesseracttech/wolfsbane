#[macro_use]
extern crate glium;
extern crate straal;
extern crate vertexify;

use std::time::{Duration, SystemTime};

use glium::{glutin, Surface};
use straal::{Mat4, Vec2, Vec3, Vec4};

mod renderer;

fn main() {
    matrix_experiments();

    run_glium();
}


fn run_glium() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut quad_model = vertexify::ObjModel::load_from_file("res/meshes/quad.obj").unwrap();
    let quad = quad_model.gen_glium_buffer(&display);

    let program = renderer::Shader::load(&display, renderer::Shader::GOURAUD).unwrap();

    let timer = SystemTime::now();
    let mut time_current = 0.0;
    let mut time_previous = 0.0;
    let mut delta_time = 0.0;


    let mut model_matrix = get_model_matrix(&straal::Vec3::new(0.0, 0.0, 0.0), 0.1);
    let view_matrix = get_view_matrix(&Vec3::new(0.0, 0.0, 1.0), &Vec3::new(0.0, 0.0, -1.0), &Vec3::new(0.0, 1.0, 1.0));
    let mut frames = 0;

    let mut closed = false;
    while !closed {
        frames += 1;
        time_previous = time_current;
        time_current = get_time(&timer);
        delta_time = time_current - time_previous;

        if frames == 100 {
            frames = 0;
            let fps = 1.0 / delta_time;
            println!("fps: {}", fps);
        }

        let mut target = display.draw();

        let perspective_matrix = get_perspective_matrix(&Vec2::from(target.get_dimensions()));

        model_matrix.rotate_around(Vec3::right(), delta_time * 2.3);
        model_matrix.rotate_around(Vec3::up(), delta_time * 2.9);
        model_matrix.rotate_around(-Vec3::forward(), delta_time * 3.1);

        let uniforms = uniform! {model : model_matrix, view: view_matrix, perspective : perspective_matrix};

        target.clear_color(0.01, 0.01, 0.01, 1.0);

        quad.draw(&mut target, &program, &uniforms, &Default::default());

        target.finish().unwrap();

        //Processing the glutin events
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (), //Don't do anything for other window events
                }
                _ => (), //Don't do anything for other events
            }
        });
    }
}


fn matrix_experiments() {
    let mut trans = renderer::Transform::default();
    trans.set_position(straal::Vec3::new(10.0, 10.0, 10.0));
    trans.set_scale(straal::Vec3::new(2.0, 3.0, 4.0));
    trans.set_forward(straal::Vec3::new(1.0, 1.0, 1.0).normalized(), straal::Vec3::up());
    let matrix = trans.get_matrix();

    println!("{}", trans);
    println!("{}", matrix);
}

pub fn get_perspective_matrix(target_dims: &Vec2) -> Mat4 {
    let aspect_ratio = target_dims.y as f32 / target_dims.x as f32;
    let fov = std::f32::consts::PI / 3.0;
    let z_far = 1024.0;
    let z_near = 0.1;
    let f = 1.0 / (fov / 2.0).tan();

    Mat4::new(f * aspect_ratio, 0.0, 0.0, 0.0,
              0.0, f, 0.0, 0.0,
              0.0, 0.0, (z_far + z_near) / (z_far - z_near), -(2.0 * z_far * z_near) / (z_far - z_near),
              0.0, 0.0, 1.0, 0.0)
}


pub fn get_view_matrix(pos: &Vec3, dir: &Vec3, up: &Vec3) -> Mat4 {
    let fwd = dir.normalized();
    let rht = Vec3::cross(up, &fwd).normalized();
    let up = Vec3::cross(&fwd, &rht);
    let pos = Vec3::new(-Vec3::dot(pos, &rht), -Vec3::dot(pos, &up), -Vec3::dot(pos, &fwd));

    Mat4::new_from_vec4s(Vec4::from((rht, pos.x)),
                         Vec4::from((up, pos.y)),
                         Vec4::from((fwd, pos.z)),
                         Vec4::new(0.0, 0.0, 0.0, 1.0))
}

pub fn get_model_matrix(pos: &Vec3, scale: straal::Real) -> Mat4 {
    Mat4::new(scale, 0.0, 0.0, pos.x,
              0.0, scale, 0.0, pos.y,
              0.0, 0.0, scale, pos.z,
              0.0, 0.0, 0.0, 1.0)
}

pub fn get_time(timer: &SystemTime) -> f32 {
    match timer.elapsed() {
        Ok(elapsed) => ((elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64) as f64 / 1_000_000_000.0) as f32,
        Err(e) => {
            println!("Error: {:?}", e);
            0.0
        }
    }
}