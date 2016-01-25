#[macro_use]
extern crate glium;
extern crate petgraph;
extern crate zoom;
extern crate num;

use glium::Surface;

pub fn render_nodes<I>(display: &mut glium::Display, nodes: I)
    where I: Iterator<Item=zoom::Cartesian2<f32>>
{
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        center: [f32; 2],
    }

    implement_vertex!(Vertex, position, center);

    let out_triangle = [
        zoom::Cartesian2{x: 0.0, y: 2.0}/32.0,
        zoom::Cartesian2{x: -1.7320508075689, y: -1.0}/32.0,
        zoom::Cartesian2{x: 1.7320508075689, y: -1.0}/32.0,
    ];

    let vertices: Vec<_> = nodes.flat_map(|n| {
        out_triangle
            .iter()
            .map(move |&v| (v + n))
            .map(move |v| Vertex{position: [v.x, v.y], center: [n.x, n.y]})
    }).collect();

    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    //let center_buffer = glium::VertexBuffer::new(display, &centers).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 center;
        out vec2 frag_pos;
        out vec2 frag_cen;
        uniform mat4 matrix;
        void main() {
            frag_pos = position;
            frag_cen = center;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 frag_pos;
        in vec2 frag_cen;
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, max(0.0, 1.0 - 32.0*length(frag_pos-frag_cen)));
        }
    "#;

    let program = glium::Program::from_source(display, vertex_shader_src,
        fragment_shader_src, None).unwrap();

    let mut t = -0.5;

    loop {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ]
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
