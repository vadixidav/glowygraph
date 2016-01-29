#[macro_use]
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use glium::Surface;

static VSHADER_SOURCE: &'static str = r#"
    #version 150
    in vec3 position;
    uniform mat4 modelview;
    void main() {
        gl_Position = modelview * vec4(position, 1.0);
    }
"#;

static GSHADER_SOURCE: &'static str = r#"
    #version 150

    uniform mat4 projection;

    layout(points) in;
    layout(triangle_strip, max_vertices = 3) out;

    out vec2 delta;

    void main()
    {
    vec4 center = gl_in[0].gl_Position;

    delta = vec2(0, 2.0);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();

    delta = vec2(-1.7320508075689, -1.0);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();

    delta = vec2(1.7320508075689, -1.0);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();
    }
"#;

static FSHADER_SOURCE: &'static str = r#"
    #version 150
    in vec2 delta;
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, max(0.0, 0.5 - pow(length(delta), 0.25)));
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
            program: glium::Program::from_source(display,
                VSHADER_SOURCE, FSHADER_SOURCE, Some(GSHADER_SOURCE)).unwrap(),
        }
    }

    pub fn render_nodes<I>(&self, modelview: [[f32; 4]; 4], projection: [[f32; 4]; 4], nodes: I)
        where I: Iterator<Item=na::Vec3<f32>>
    {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 3],
        }

        implement_vertex!(Vertex, position);

        let vertices: Vec<_> = nodes.map(|n| Vertex{position: [n.x, n.y, n.z]}).collect();

        let vertex_buffer = glium::VertexBuffer::new(self.display, &vertices).unwrap();
        //let center_buffer = glium::VertexBuffer::new(display, &centers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        target.draw(&vertex_buffer, &indices, &self.program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }
}
