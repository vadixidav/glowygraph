extern crate glowygraph;
extern crate zoom;
extern crate petgraph;
extern crate glium;
extern crate num;

use num::traits::Float;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let glowy = glowygraph::GraphRenderer::new(&display);

    let mut deps = petgraph::Graph::<zoom::Cartesian2<f32>, bool>::new();
    deps.add_node(zoom::Cartesian2{x: -0.5, y: -0.5} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.0, y: 0.5} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.5, y: -0.25} * 32.0);
    deps.add_node(zoom::Cartesian2{x: 0.25, y: -0.35} * 32.0);

    let mut t = 0.0;
    loop {
        t += 0.01;
        let transform = [
            [t.cos()/32.0, -t.sin()/32.0, 0.0, 0.0],
            [t.sin()/32.0, t.cos()/32.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        glowy.render(transform, deps.node_weights_mut().map(|n| n.clone()));

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
