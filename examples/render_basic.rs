extern crate glowygraph;
extern crate petgraph;
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use num::traits::Float;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let glowy = glowygraph::Renderer::new(&display);

    let mut deps = petgraph::Graph::<na::Vec3<f32>, bool>::new();
    deps.add_node(na::Vec3{x: -0.2, y: -0.3, z: 2.0} / 2.0);
    deps.add_node(na::Vec3{x: 0.4, y: 0.5, z: 5.0} / 2.0);
    deps.add_node(na::Vec3{x: 0.6, y: -0.7, z: 4.0} / 2.0);
    deps.add_node(na::Vec3{x: -0.8, y: -0.9, z: 2.5} / 2.0);
    deps.add_node(na::Vec3{x: 0.1, y: 0.2, z: 3.0} / 2.0);
    deps.add_node(na::Vec3{x: -0.3, y: 0.4, z: 3.0} / 2.0);
    deps.add_node(na::Vec3{x: 0.5, y: -0.6, z: 4.0} / 2.0);

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
        glowy.render_nodes(transform, perspective, deps.node_weights_mut().map(|n| n.clone()));

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
