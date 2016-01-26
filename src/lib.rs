#[macro_use]
extern crate glium;
extern crate zoom;
extern crate num;

use glium::Surface;

static VSHADER_SOURCE: &'static str = r#"
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

static FSHADER_SOURCE: &'static str = r#"
    #version 140
    in vec2 frag_pos;
    in vec2 frag_cen;
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, max(0.0, 1.0 - length(frag_pos-frag_cen)));
    }
"#;

pub struct Renderer<'a> {
    display: &'a glium::Display,
    program: glium::Program,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &'a glium::Display) -> Self {
        Renderer {
            display: display,
            program: glium::Program::from_source(display, VSHADER_SOURCE, FSHADER_SOURCE, None).unwrap(),
        }
    }

    pub fn render_nodes<I>(&self, transform: [[f32; 4]; 4], nodes: I)
        where I: Iterator<Item=zoom::Cartesian2<f32>>
    {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            center: [f32; 2],
        }

        implement_vertex!(Vertex, position, center);

        //Create smallest equilateral triangle possible containing unit circle
        let out_triangle = [
            zoom::Cartesian2{x: 0.0, y: 2.0},
            zoom::Cartesian2{x: -1.7320508075689, y: -1.0},
            zoom::Cartesian2{x: 1.7320508075689, y: -1.0},
        ];

        let vertices: Vec<_> = nodes.flat_map(|n| {
            out_triangle
                .iter()
                .map(move |&v| (v + n))
                .map(move |v| Vertex{position: [v.x, v.y], center: [n.x, n.y]})
        }).collect();

        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        //let center_buffer = glium::VertexBuffer::new(display, &centers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: transform,
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target.draw(&vertex_buffer, &indices, &self.program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }
}
