extern crate glowygraph as gg;
extern crate petgraph;
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use gg::render2::*;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);

    let mut deps = petgraph::Graph::<[f32; 2], bool>::new();
    let nodes = [deps.add_node([-0.2, -0.3]),
                 deps.add_node([0.4, 0.5]),
                 deps.add_node([0.6, -0.7]),
                 deps.add_node([-0.8, -0.9]),
                 deps.add_node([0.1, 0.2]),
                 deps.add_node([-0.3, 0.4]),
                 deps.add_node([0.5, -0.6])];

    deps.extend_with_edges(&[(nodes[0], nodes[1]),
                             (nodes[1], nodes[2]),
                             (nodes[2], nodes[3]),
                             (nodes[3], nodes[4]),
                             (nodes[4], nodes[5]),
                             (nodes[5], nodes[6]),
                             (nodes[6], nodes[0])]);

    let qbeziers = vec![QBezier {
                            position0: [-0.9, -0.9],
                            position1: [0.9, 0.0],
                            position2: [0.9, 0.0],
                            inner_color0: [1.0, 0.0, 0.0, 1.0],
                            inner_color1: [0.0, 0.0, 1.0, 1.0],
                            falloff_color0: [0.0, 1.0, 0.0, 1.0],
                            falloff_color1: [1.0, 0.0, 0.0, 1.0],
                            falloff0: 0.25,
                            falloff1: 0.8,
                            falloff_radius0: 0.2,
                            falloff_radius1: 0.05,
                            inner_radius0: 0.05,
                            inner_radius1: 0.3,
                            accuracy: 8,
                        }];

    loop {
        use glium::Surface;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Render nodes
        /*glowy.render_nodes(&mut target,
                           &deps.node_weights_mut()
                               .map(|n| {
                Node {
                    position: n.clone(),
                    inner_color: [1.0, 0.0, 0.0, 1.0],
                    falloff_color: [0.0, 0.0, 1.0, 1.0],
                    falloff: 0.25,
                    inner_radius: 0.05,
                    falloff_radius: 0.1,
                }
            })
                               .collect::<Vec<_>>()[..]);

        // Render edges
        glowy.render_edges(&mut target,
                           &deps.edge_indices()
                               .map(|e| deps.edge_endpoints(e))
                               .flat_map(|n| {
                let indices = n.unwrap().clone();
                std::iter::once(Node {
                        position: deps.node_weight(indices.0).unwrap().clone(),
                        inner_color: [0.0, 1.0, 0.0, 1.0],
                        falloff_color: [1.0, 0.0, 0.0, 1.0],
                        falloff: 0.25,
                        inner_radius: 0.05,
                        falloff_radius: 0.1,
                    })
                    .chain(std::iter::once(Node {
                        position: deps.node_weight(indices.1).unwrap().clone(),
                        inner_color: [0.0, 0.0, 1.0, 1.0],
                        falloff_color: [0.0, 1.0, 0.0, 1.0],
                        falloff: 0.10,
                        inner_radius: 0.1,
                        falloff_radius: 0.05,
                    }))
            })
                               .collect::<Vec<_>>()[..]);*/

        // Render nodes
        glowy.render_qbeziers(&mut target, &qbeziers);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
