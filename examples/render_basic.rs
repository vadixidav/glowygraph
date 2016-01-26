extern crate glowygraph;
extern crate zoom;
extern crate petgraph;
extern crate glium;
extern crate num;

use num::traits::Float;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let glowy = glowygraph::Renderer::new(&display);

    let mut deps = petgraph::Graph::<zoom::Cartesian2<f32>, bool>::new();
    deps.add_node(zoom::Cartesian2{x: -0.2, y: -0.3} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.4, y: 0.5} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.6, y: -0.7} * 32.0);
    deps.add_node(zoom::Cartesian2{x: -0.8, y: -0.9} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.1, y: 0.2} * 32.0);
    deps.add_node(zoom::Cartesian2{x: -0.3, y: 0.4} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.5, y: -0.6} * 32.0);

    let mut t = 0.0;
    loop {
        t += 0.01;
        let transform = [
            [t.cos()/32.0, -t.sin()/32.0, 0.0, 0.0],
            [t.sin()/32.0, t.cos()/32.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        glowy.render_nodes(transform, deps.node_weights_mut().map(|n| n.clone()));

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
