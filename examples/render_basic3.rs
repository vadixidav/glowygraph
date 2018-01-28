extern crate cgmath;
extern crate glium;
extern crate glowygraph as gg;

use gg::render3::*;
use glium::glutin;
use cgmath::{Matrix4, PerspectiveFov, Rad, Transform, Vector3};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let window_builder = glutin::WindowBuilder::new().with_title("Use wasd, mouse, and qe to move");
    let display = glium::Display::new(window_builder, context, &events_loop).unwrap();
    let glowy = Renderer::new(&display);

    let nodes = vec![
        [-0.2, -0.3, 2.0],
        [0.4, 0.5, 5.0],
        [0.6, -0.7, 4.0],
        [-0.8, -0.9, 2.5],
        [0.1, 0.2, 3.0],
        [-0.3, 0.4, 3.0],
        [0.5, -0.6, 4.0],
    ];

    let edges = vec![
        (
            [0.198476, -0.19746, 6.9781234],
            [-0.198476, -0.19746, 2.9781234],
        ),
        (
            [0.715482, 0.1784692, 5.7615471],
            [-0.1824612, -0.7813652, 3.825643],
        ),
    ];

    // Set mouse cursor to middle
    {
        let (dimx, dimy) = display.get_framebuffer_dimensions();
        let (hdimx, hdimy) = (dimx / 2, dimy / 2);
        display
            .gl_window()
            .window()
            .set_cursor_position(hdimx as i32, hdimy as i32)
            .ok()
            .unwrap();
    }

    let mut movement = Matrix4::from_angle_y(Rad(3.14));

    let mut upstate = glutin::ElementState::Released;
    let mut dnstate = glutin::ElementState::Released;
    let mut ltstate = glutin::ElementState::Released;
    let mut rtstate = glutin::ElementState::Released;
    let mut fdstate = glutin::ElementState::Released;
    let mut bkstate = glutin::ElementState::Released;

    loop {
        use glium::Surface;

        // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        let hscale = dims.1 as f32 / dims.0 as f32;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let perspective: Matrix4<f32> = PerspectiveFov {
            fovy: Rad(3.14 / 2.0),
            aspect: hscale,
            near: 1.0,
            far: 500.0,
        }.to_perspective()
            .into();

        let matr: Matrix4<f32> = movement * 5.0;

        let perspective: &[[f32; 4]; 4] = perspective.as_ref();
        let matr: &[[f32; 4]; 4] = matr.as_ref();

        // Render nodes
        glowy.render_nodes(
            &mut target,
            matr.clone(),
            perspective.clone(),
            &nodes
                .iter()
                .map(|n| Node {
                    position: n.clone(),
                    inner_color: [1.0, 0.0, 0.0, 1.0],
                    falloff_color: [0.0, 0.0, 1.0, 1.0],
                    falloff: 0.25,
                    inner_radius: 0.0,
                    falloff_radius: 2.0,
                })
                .collect::<Vec<_>>()[..],
        );

        // Render edges
        glowy.render_edges_flat(
            &mut target,
            matr.clone(),
            perspective.clone(),
            &edges
                .iter()
                .flat_map(|indices| {
                    std::iter::once(Node {
                        position: indices.0,
                        inner_color: [0.0, 1.0, 0.0, 1.0],
                        falloff_color: [1.0, 0.0, 0.0, 1.0],
                        falloff: 0.25,
                        inner_radius: 0.0,
                        falloff_radius: 1.0,
                    }).chain(std::iter::once(Node {
                        position: indices.1,
                        inner_color: [0.0, 0.0, 1.0, 1.0],
                        falloff_color: [0.0, 1.0, 0.0, 1.0],
                        falloff: 0.10,
                        inner_radius: 0.0,
                        falloff_radius: 3.0,
                    }))
                })
                .collect::<Vec<_>>()[..],
        );

        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent {
                event:
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                state,
                                virtual_keycode: Some(vkc),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                use glutin::VirtualKeyCode as VKC;
                match vkc {
                    VKC::W => {
                        fdstate = state;
                    }
                    VKC::S => {
                        bkstate = state;
                    }
                    VKC::A => {
                        ltstate = state;
                    }
                    VKC::D => {
                        rtstate = state;
                    }
                    VKC::Q => {
                        dnstate = state;
                    }
                    VKC::E => {
                        upstate = state;
                    }
                    _ => (),
                }
            }
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => ::std::process::exit(0),
                glutin::WindowEvent::CursorMoved {
                    position: (x, y), ..
                } => {
                    let (dimx, dimy) = display.get_framebuffer_dimensions();
                    let (hdimx, hdimy) = (dimx / 2, dimy / 2);
                    movement = Matrix4::from_angle_x(Rad((y as i32 - hdimy as i32) as f32 / 512.0))
                        .concat(&movement);
                    movement = Matrix4::from_angle_y(Rad((x as i32 - hdimx as i32) as f32 / 512.0))
                        .concat(&movement);
                    display
                        .gl_window()
                        .window()
                        .set_cursor_position(hdimx as i32, hdimy as i32)
                        .ok()
                        .unwrap();
                }
                _ => (),
            },
            _ => (),
        });

        if upstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(-Vector3::unit_y() * 0.1).concat(&movement);
        }
        if dnstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(Vector3::unit_y() * 0.1).concat(&movement);
        }
        if ltstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(Vector3::unit_x() * 0.1).concat(&movement);
        }
        if rtstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(-Vector3::unit_x() * 0.1).concat(&movement);
        }
        if fdstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(Vector3::unit_z() * 0.1).concat(&movement);
        }
        if bkstate == glutin::ElementState::Pressed {
            movement = Matrix4::from_translation(-Vector3::unit_z() * 0.1).concat(&movement);
        }
    }
}
