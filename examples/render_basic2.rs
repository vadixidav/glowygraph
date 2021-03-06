extern crate glium;
extern crate glowygraph as gg;

use gg::render2::*;
use glium::glutin;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let window_builder = glutin::WindowBuilder::new();
    let display = glium::Display::new(window_builder, context, &events_loop).unwrap();
    let glowy = Renderer::new(&display);
    let nodes = vec![
        [-0.2, -0.3],
        [0.4, 0.5],
        [0.6, -0.7],
        [-0.8, -0.9],
        [0.1, 0.2],
        [-0.3, 0.4],
        [0.5, -0.6],
    ];

    let edges = vec![
        ([-0.24, 0.8462], [0.3272, -0.1236]),
        ([-0.19375, -0.5146], [0.1497, 0.981274]),
    ];

    let qbeziers = vec![
        QBezier {
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
            falloff_radius1: 0.01,
            inner_radius0: 0.05,
            inner_radius1: 0.01,
        },
    ];

    loop {
        use glium::Surface;

        // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        let hscale = dims.1 as f32 / dims.0 as f32;

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Render nodes
        glowy.render_nodes(
            &mut target,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [[hscale, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &nodes
                .iter()
                .map(|n| Node {
                    position: [n[0] / hscale, n[1]],
                    inner_color: [1.0, 0.0, 0.0, 1.0],
                    falloff_color: [0.0, 0.0, 1.0, 1.0],
                    falloff: 0.25,
                    inner_radius: 0.05,
                    falloff_radius: 0.1,
                })
                .collect::<Vec<_>>()[..],
        );

        // Render edges
        glowy.render_edges_round(
            &mut target,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [[hscale, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &edges
                .iter()
                .flat_map(|indices| {
                    std::iter::once(Node {
                        position: [indices.0[0] / hscale, indices.0[1]],
                        inner_color: [0.0, 1.0, 0.0, 1.0],
                        falloff_color: [1.0, 0.0, 0.0, 1.0],
                        falloff: 0.25,
                        inner_radius: 0.05,
                        falloff_radius: 0.1,
                    }).chain(std::iter::once(Node {
                        position: [indices.1[0] / hscale, indices.1[1]],
                        inner_color: [0.0, 0.0, 1.0, 1.0],
                        falloff_color: [0.0, 1.0, 0.0, 1.0],
                        falloff: 0.10,
                        inner_radius: 0.1,
                        falloff_radius: 0.05,
                    }))
                })
                .collect::<Vec<_>>()[..],
        );

        // Render nodes
        glowy.render_qbeziers_round(
            &mut target,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [[hscale, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &qbeziers
                .iter()
                .cloned()
                .map(|mut b| {
                    b.position0[0] /= hscale;
                    b.position1[0] /= hscale;
                    b.position2[0] /= hscale;
                    b
                })
                .collect::<Vec<_>>(),
        );

        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => ::std::process::exit(0),
                _ => (),
            },
            _ => (),
        });
    }
}
