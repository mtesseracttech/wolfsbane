#[macro_use]
extern crate glium;
extern crate straal;
extern crate vertexify;

use std::time::{Duration, SystemTime};

use glium::{glutin, Surface};
use glutin::dpi::LogicalPosition;
use glutin::ElementState;
use glutin::MouseScrollDelta;
use glutin::VirtualKeyCode;
use straal::{Mat3n, Mat4n, Quatn, Vec2n, Vec3n, Vec4n};

mod renderer;
use renderer::SceneContainer;

#[allow(dead_code)]
fn main() {
    run_glium();
}

fn run_glium() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_multisampling(8);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,

            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
        ..Default::default()
    };

    let mut lucy_model = vertexify::ObjModel::load_from_file("res/meshes/lucy.obj").unwrap();
    let lucy = lucy_model.gen_glium_buffer(&display);

    let mut quad_model = vertexify::ObjModel::load_from_file("res/meshes/quad.obj").unwrap();
    let quad = quad_model.gen_glium_buffer(&display);

    let program = renderer::Shader::load(&display, renderer::Shader::NORMALS).unwrap();

    let timer = SystemTime::now();
    let mut time_current = 0.0;
    let mut time_previous = 0.0;
    let mut delta_time = 0.0;

    let mut transform = renderer::Transform::default();

    let view_matrix = get_view_matrix(
        &Vec3n::new(0.0, 0.0, 1.0),
        &Vec3n::new(0.0, 0.0, -1.0),
        &Vec3n::new(0.0, 1.0, 0.0),
    );
    let light_direction = Vec3n::new(0.5, -0.5, 1.0).normalized();

    let mut frames = 0;

    //let mut position_delta = Vec2::zero();
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;

    //Mouse related debugging things
    let mut mouse_delta = Vec2n::zero();
    let mut mouse_down = false;
    let mut mouse_zoom = 2.0;
    let mut mouse_zoom_changed = false;

    transform.set_local_scale(Vec3n::all(mouse_zoom));
    //transform.rotate_angle_axis(std::f32::consts::FRAC_PI_4, Vec3n::up());

    let mut rotation_matrix = Mat3n::identity();

    let mut world = SceneContainer::new();
    let mut world_access = (*world).get_mut();

    let mut closed = false;
    while !closed {
        frames += 1;
        time_previous = time_current;
        time_current = get_time(&timer);
        delta_time = time_current - time_previous;

        let dx = match (left_pressed, right_pressed) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };

        let dy = match (up_pressed, down_pressed) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };

        let position_delta = Vec2n::new(dx, dy);

        let delta_factor = 200.0;

        if position_delta != Vec2n::zero() {
            transform.translate(Vec3n::new(
                position_delta.x / delta_factor,
                position_delta.y / delta_factor,
                0.0,
            ));
        }

        let mut target = display.draw();

        let perspective_matrix = get_perspective_matrix(&Vec2n::from(target.get_dimensions()));

        if mouse_zoom_changed {
            transform.set_local_scale(Vec3n::all(mouse_zoom))
        }

        let mut rot = transform.get_local_rotation();
        rot *= Quatn::get_quat_from_angle_axis(delta_time, Vec3n::up());
        rot *= Quatn::get_quat_from_angle_axis(delta_time, Vec3n::right());
        transform.set_local_rotation(rot);

        let uniforms = uniform! {model : transform.get_local_to_world_matrix(), view: view_matrix, perspective : perspective_matrix, light_dir : light_direction};

        target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

        lucy.draw(&mut target, &program, &uniforms, &draw_parameters);
        //lucy.draw(&mut target, &program, &uniforms_2, &draw_parameters);
        //quad.draw(&mut target, &program, &uniforms, &draw_parameters);

        target.finish().unwrap();

        mouse_delta = Vec2n::zero();
        mouse_zoom_changed = false;
        //Processing the glutin events
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        if button == glutin::MouseButton::Left {
                            mouse_down = match state {
                                ElementState::Pressed => true,
                                ElementState::Released => false,
                            };
                        }
                    }
                    glutin::WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            mouse_zoom += y / 10.0;
                            mouse_zoom_changed = true;
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            mouse_zoom += pos.y as f32;
                            mouse_zoom_changed = true;
                        }
                    },
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(keycode) => match keycode {
                                VirtualKeyCode::W => {
                                    up_pressed = input.state == ElementState::Pressed
                                }
                                VirtualKeyCode::S => {
                                    down_pressed = input.state == ElementState::Pressed
                                }
                                VirtualKeyCode::A => {
                                    left_pressed = input.state == ElementState::Pressed
                                }
                                VirtualKeyCode::D => {
                                    right_pressed = input.state == ElementState::Pressed
                                }
                                _ => {}
                            },
                            None => {}
                        }
                    }
                    _ => (), //Don't do anything for other window events
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::MouseMotion { delta } => {
                        mouse_delta = Vec2n::from((delta.0 as f32, delta.1 as f32));
                    }
                    _ => (),
                },
                _ => (), //Don't do anything for other events
            }
        });
    }
}

pub fn get_perspective_matrix(target_dims: &Vec2n) -> Mat4n {
    let aspect_ratio = target_dims.y as f32 / target_dims.x as f32;
    let fov = std::f32::consts::PI / 3.0;
    let z_far = 1024.0;
    let z_near = 0.1;
    let f = 1.0 / (fov / 2.0).tan();

    Mat4n::new(
        f * aspect_ratio,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (z_far + z_near) / (z_far - z_near),
        -(2.0 * z_far * z_near) / (z_far - z_near),
        0.0,
        0.0,
        1.0,
        0.0,
    )
}

pub fn get_view_matrix(pos: &Vec3n, dir: &Vec3n, up: &Vec3n) -> Mat4n {
    let fwd = dir.normalized();
    let rht = up.cross(fwd).normalized();
    let up = fwd.cross(rht);
    let pos = Vec3n {
        x: -pos.dot(rht),
        y: -pos.dot(up),
        z: -pos.dot(fwd),
    };

    Mat4n::new_from_vec4s(
        Vec4n::from((rht, pos.x)),
        Vec4n::from((up, pos.y)),
        Vec4n::from((fwd, pos.z)),
        Vec4n::new(0.0, 0.0, 0.0, 1.0),
    )
}

pub fn get_model_matrix(pos: &Vec3n, scale: f32) -> Mat4n {
    Mat4n::new(
        scale, 0.0, 0.0, pos.x, 0.0, scale, 0.0, pos.y, 0.0, 0.0, scale, pos.z, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn get_time(timer: &SystemTime) -> f32 {
    match timer.elapsed() {
        Ok(elapsed) => {
            ((elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64) as f64
                / 1_000_000_000.0) as f32
        }
        Err(e) => {
            println!("Error: {:?}", e);
            0.0
        }
    }
}
