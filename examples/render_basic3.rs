extern crate glowygraph as gg;
extern crate glium;
extern crate nalgebra as na;
extern crate num;

use na::{ToHomogeneous, Translation, Rotation};
use num::traits::One;
use gg::render3::*;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    let window = display.get_window().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);

    let nodes = vec![[-0.2, -0.3, 2.0],
                     [0.4, 0.5, 5.0],
                     [0.6, -0.7, 4.0],
                     [-0.8, -0.9, 2.5],
                     [0.1, 0.2, 3.0],
                     [-0.3, 0.4, 3.0],
                     [0.5, -0.6, 4.0]];

    let edges = vec![([0.198476, -0.19746, 6.9781234], [-0.198476, -0.19746, 2.9781234]),
                     ([0.715482, 0.1784692, 5.7615471], [-0.1824612, -0.7813652, 3.825643])];

    // Set mouse cursor to middle
    {
        let (dimx, dimy) = display.get_framebuffer_dimensions();
        let (hdimx, hdimy) = (dimx / 2, dimy / 2);
        window.set_cursor_position(hdimx as i32, hdimy as i32).ok().unwrap();
    }

    let perspective = *na::Persp3::new(1.5, 1.0, 0.0, 500.0).to_mat().as_ref();
    let mut movement = na::Iso3::<f32>::one();

    let mut upstate = glium::glutin::ElementState::Released;
    let mut dnstate = glium::glutin::ElementState::Released;
    let mut ltstate = glium::glutin::ElementState::Released;
    let mut rtstate = glium::glutin::ElementState::Released;
    let mut fdstate = glium::glutin::ElementState::Released;
    let mut bkstate = glium::glutin::ElementState::Released;

    loop {
        use glium::Surface;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let matr = movement.to_homogeneous() * 3.0;

        // Render nodes
        glowy.render_nodes(&mut target,
                           matr.as_ref(),
                           &perspective,
                           &nodes.iter()
                               .map(|n| {
                Node {
                    position: n.clone(),
                    inner_color: [1.0, 0.0, 0.0, 1.0],
                    falloff_color: [0.0, 0.0, 1.0, 1.0],
                    falloff: 0.25,
                    inner_radius: 1.0,
                    falloff_radius: 1.0,
                }
            })
                               .collect::<Vec<_>>()[..]);

        // Render edges
        glowy.render_edges(&mut target,
                           matr.as_ref(),
                           &perspective,
                           &edges.iter()
                               .flat_map(|indices| {
                std::iter::once(Node {
                        position: indices.0,
                        inner_color: [0.0, 1.0, 0.0, 1.0],
                        falloff_color: [1.0, 0.0, 0.0, 1.0],
                        falloff: 0.25,
                        inner_radius: 0.5,
                        falloff_radius: 0.5,
                    })
                    .chain(std::iter::once(Node {
                        position: indices.1,
                        inner_color: [0.0, 0.0, 1.0, 1.0],
                        falloff_color: [0.0, 1.0, 0.0, 1.0],
                        falloff: 0.10,
                        inner_radius: 1.5,
                        falloff_radius: 1.5,
                    }))
            })
                               .collect::<Vec<_>>()[..]);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::W)) => {
                    fdstate = state;
                }
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::S)) => {
                    bkstate = state;
                }
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::A)) => {
                    ltstate = state;
                }
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::D)) => {
                    rtstate = state;
                }
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Q)) => {
                    dnstate = state;
                }
                glium::glutin::Event::KeyboardInput(state,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::E)) => {
                    upstate = state;
                }
                glium::glutin::Event::MouseMoved(x, y) => {
                    let (dimx, dimy) = display.get_framebuffer_dimensions();
                    let (hdimx, hdimy) = (dimx / 2, dimy / 2);
                    movement.append_rotation_mut(&na::Vec3::new(-(y - hdimy as i32) as f32 /
                                                                128.0,
                                                                (x - hdimx as i32) as f32 / 128.0,
                                                                0.0));
                    window.set_cursor_position(hdimx as i32, hdimy as i32).ok().unwrap();
                }
                _ => (),
            }
        }

        if upstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(0.0, -0.1, 0.0));
        }
        if dnstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(0.0, 0.1, 0.0));
        }
        if ltstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(-0.1, 0.0, 0.0));
        }
        if rtstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(0.1, 0.0, 0.0));
        }
        if fdstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(0.0, 0.0, -0.1));
        }
        if bkstate == glium::glutin::ElementState::Pressed {
            movement.append_translation_mut(&na::Vec3::new(0.0, 0.0, 0.1));
        }
    }
}
