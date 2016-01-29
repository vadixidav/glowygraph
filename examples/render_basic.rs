extern crate glowygraph as gg;
extern crate petgraph;
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use num::traits::Float;
use glium::Surface;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let glowy = gg::Renderer::new(&display);

    let mut deps = petgraph::Graph::<[f32; 3], bool>::new();
    deps.add_node([-0.2, -0.3, 2.0]);
    deps.add_node([0.4, 0.5, 5.0]);
    deps.add_node([0.6, -0.7, 4.0]);
    deps.add_node([-0.8, -0.9, 2.5]);
    deps.add_node([0.1, 0.2, 3.0]);
    deps.add_node([-0.3, 0.4, 3.0]);
    deps.add_node([0.5, -0.6, 4.0]);

    let mut t = 0.0;
    loop {
        t += 0.01;
        let transform = [
            [t.cos(), -t.sin(), 0.0, 0.0],
            [t.sin(), t.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let perspective = *na::Persp3::new(1.5, 1.0, 1.0, 500.0).to_mat().as_ref();
        glowy.render_nodes(&transform, &perspective,
            &deps.node_weights_mut().map(|n| gg::Node{position: n.clone(), color: [1.0, 1.0, 1.0, 1.0]}).collect::<Vec<_>>()[..]);

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
