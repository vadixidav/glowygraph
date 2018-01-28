extern crate glium;
extern crate glowygraph as gg;
extern crate rand;
use rand::{Isaac64Rng, Rng, SeedableRng};
use glium::glutin;

use gg::render2::*;

const TOTAL_BEZIERS: usize = 8192;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let window_builder = glutin::WindowBuilder::new();
    let display = glium::Display::new(window_builder, context, &events_loop).unwrap();
    let glowy = Renderer::new(&display);

    let mut rng = Isaac64Rng::from_seed(&[5, 1, 2, 6]);

    loop {
        use glium::Surface;

        // Get dimensions
        let dims = display.get_framebuffer_dimensions();
        let hscale = dims.1 as f32 / dims.0 as f32;

        let qbeziers = (0..TOTAL_BEZIERS)
            .map(|_| {
                let basepos = [rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)];
                QBezier {
                    position0: [
                        (basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                        basepos[1] + rng.gen_range(-0.1, 0.1),
                    ],
                    position1: [
                        (basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                        basepos[1] + rng.gen_range(-0.1, 0.1),
                    ],
                    position2: [
                        (basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                        basepos[1] + rng.gen_range(-0.1, 0.1),
                    ],
                    inner_color0: [
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                    ],
                    inner_color1: [
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                    ],
                    falloff_color0: [
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                    ],
                    falloff_color1: [
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                        rng.next_f32(),
                    ],
                    falloff0: rng.gen_range(0.0, 1.0),
                    falloff1: rng.gen_range(0.0, 1.0),
                    falloff_radius0: rng.gen_range(0.001, 0.002),
                    falloff_radius1: rng.gen_range(0.001, 0.002),
                    inner_radius0: rng.gen_range(0.001, 0.002),
                    inner_radius1: rng.gen_range(0.001, 0.002),
                }
            })
            .collect::<Vec<_>>();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Render nodes
        glowy.render_qbeziers_flat(
            &mut target,
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            [[hscale, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            &qbeziers,
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
