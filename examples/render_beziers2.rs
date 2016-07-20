extern crate glowygraph as gg;
extern crate glium;
extern crate rand;
use rand::{Rng, SeedableRng, Isaac64Rng};

use gg::render2::*;

const TOTAL_BEZIERS: usize = 8192;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().with_vsync().build_glium().unwrap();
    // window.set_cursor_state(glium::glutin::CursorState::Hide).ok().unwrap();
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
                    position0: [(basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                                basepos[1] + rng.gen_range(-0.1, 0.1)],
                    position1: [(basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                                basepos[1] + rng.gen_range(-0.1, 0.1)],
                    position2: [(basepos[0] + rng.gen_range(-0.1, 0.1)) / hscale,
                                basepos[1] + rng.gen_range(-0.1, 0.1)],
                    inner_color0: [rng.next_f32(), rng.next_f32(), rng.next_f32(), rng.next_f32()],
                    inner_color1: [rng.next_f32(), rng.next_f32(), rng.next_f32(), rng.next_f32()],
                    falloff_color0: [rng.next_f32(),
                                     rng.next_f32(),
                                     rng.next_f32(),
                                     rng.next_f32()],
                    falloff_color1: [rng.next_f32(),
                                     rng.next_f32(),
                                     rng.next_f32(),
                                     rng.next_f32()],
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
        glowy.render_flat_qbeziers_hscale(&mut target, hscale, &qbeziers);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
