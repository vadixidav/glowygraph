extern crate glowygraph as gg;
extern crate glium;

use gg::render2::*;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
    let glowy = Renderer::new(&display);
    let nodes = vec![[-0.2, -0.3],
                     [0.4, 0.5],
                     [0.6, -0.7],
                     [-0.8, -0.9],
                     [0.1, 0.2],
                     [-0.3, 0.4],
                     [0.5, -0.6]];

    let edges = vec![([-0.24, 0.8462], [0.3272, -0.1236]),
                             ([-0.19375, -0.5146], [0.1497, 0.981274]),
                             ];

    let qbeziers = vec![QBezier {
                            position0: [-0.9, 0.0],
                            position1: [-0.8, -0.7],
                            position2: [0.9, 0.0],
                            inner_color0: [1.0, 0.0, 0.0, 1.0],
                            inner_color1: [0.0, 0.0, 1.0, 1.0],
                            falloff_color0: [0.0, 1.0, 0.0, 1.0],
                            falloff_color1: [1.0, 0.0, 0.0, 1.0],
                            falloff0: 0.05,
                            falloff1: 0.8,
                            falloff_radius0: 0.02,
                            falloff_radius1: 0.05,
                            inner_radius0: 0.05,
                            inner_radius1: 0.01,
                        }];

    loop {
        use glium::Surface;

        // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        let hscale = dims.1 as f32 / dims.0 as f32;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Render nodes
        glowy.render_nodes_hscale(&mut target,
                                  hscale,
                                  &nodes.iter()
                                      .map(|n| {
                Node {
                    position: [n[0] / hscale, n[1]],
                    inner_color: [1.0, 0.0, 0.0, 1.0],
                    falloff_color: [0.0, 0.0, 1.0, 1.0],
                    falloff: 0.25,
                    inner_radius: 0.05,
                    falloff_radius: 0.1,
                }
            })
                                      .collect::<Vec<_>>()[..]);


        // Render edges
        glowy.render_edges_hscale(&mut target,
                                  hscale,
                                  &edges.iter()
                                      .flat_map(|indices| {
                std::iter::once(Node {
                        position: [indices.0[0] / hscale, indices.0[1]],
                        inner_color: [0.0, 1.0, 0.0, 1.0],
                        falloff_color: [1.0, 0.0, 0.0, 1.0],
                        falloff: 0.25,
                        inner_radius: 0.05,
                        falloff_radius: 0.1,
                    })
                    .chain(std::iter::once(Node {
                        position: [indices.1[0] / hscale, indices.1[1]],
                        inner_color: [0.0, 0.0, 1.0, 1.0],
                        falloff_color: [0.0, 1.0, 0.0, 1.0],
                        falloff: 0.10,
                        inner_radius: 0.1,
                        falloff_radius: 0.05,
                    }))
            })
                                      .collect::<Vec<_>>()[..]);


        // Render nodes
        glowy.render_qbeziers_hscale(&mut target,
                                     hscale,
                                     &qbeziers.iter()
                                         .cloned()
                                         .map(|mut b| {
                                             b.position0[0] /= hscale;
                                             b.position1[0] /= hscale;
                                             b.position2[0] /= hscale;
                                             b
                                         })
                                         .collect::<Vec<_>>());

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
