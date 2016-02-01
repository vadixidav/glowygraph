extern crate glowygraph as gg;
extern crate petgraph;
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use na::{ToHomogeneous, Translation, Rotation};
use num::traits::One;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let window = display.get_window().unwrap();
    //window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = gg::Renderer::new(&display);

    let mut deps = petgraph::Graph::<[f32; 3], bool>::new();
    deps.add_node([-0.2, -0.3, 2.0]);
    deps.add_node([0.4, 0.5, 5.0]);
    deps.add_node([0.6, -0.7, 4.0]);
    deps.add_node([-0.8, -0.9, 2.5]);
    deps.add_node([0.1, 0.2, 3.0]);
    deps.add_node([-0.3, 0.4, 3.0]);
    deps.add_node([0.5, -0.6, 4.0]);

    //Set mouse cursor to middle
    {
        let (dimx, dimy) = display.get_framebuffer_dimensions();
        let (hdimx, hdimy) = (dimx/2, dimy/2);
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
        glowy.render_nodes(movement.to_homogeneous().as_ref(), &perspective,
            &deps.node_weights_mut().map(|n| gg::Node{position: n.clone(), color: [1.0, 1.0, 1.0, 1.0]}).collect::<Vec<_>>()[..]);

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::W)) => {
                    fdstate = state;
                },
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::S)) => {
                    bkstate = state;
                },
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::A)) => {
                    ltstate = state;
                },
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::D)) => {
                    rtstate = state;
                },
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::Q)) => {
                    dnstate = state;
                },
                glium::glutin::Event::KeyboardInput(state, _, Some(glium::glutin::VirtualKeyCode::E)) => {
                    upstate = state;
                },
                glium::glutin::Event::MouseMoved((x, y)) => {
                    let (dimx, dimy) = display.get_framebuffer_dimensions();
                    let (hdimx, hdimy) = (dimx/2, dimy/2);
                    movement.append_rotation_mut(&na::Vec3::new(-(y - hdimy as i32) as f32 / 128.0,
                        (x - hdimx as i32) as f32 / 128.0, 0.0));
                    window.set_cursor_position(hdimx as i32, hdimy as i32).ok().unwrap();
                },
                _ => ()
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
