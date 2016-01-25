extern crate glowygraph;
extern crate zoom;
extern crate petgraph;
extern crate glium;

fn main() {
    use glium::DisplayBuild;
    let mut display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let mut deps = petgraph::Graph::<zoom::Cartesian2<f32>, bool>::new();
    deps.add_node(zoom::Cartesian2{x: -0.5, y: -0.5});
    deps.add_node(zoom::Cartesian2{x: 0.0, y: 0.5});
    deps.add_node(zoom::Cartesian2{x: 0.5, y: -0.25});
    deps.add_node(zoom::Cartesian2{x: 0.25, y: -0.35});
    glowygraph::render_nodes(&mut display, deps.node_weights_mut().map(|n| n.clone()));
}
