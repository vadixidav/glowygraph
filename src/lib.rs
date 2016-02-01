#[macro_use]
extern crate glium;
extern crate num;
extern crate nalgebra as na;

use glium::Surface;

static VSHADER_SOURCE: &'static str = r#"
    #version 150
    in vec3 position;
    in vec4 color;
    in float falloff;
    out vec4 gcolor;
    out float gfalloff;
    uniform mat4 modelview;
    void main() {
        gcolor = color;
        gfalloff = falloff;
        gl_Position = modelview * vec4(position, 1.0);
    }
"#;

static GSHADER_SOURCE: &'static str = r#"
    #version 150

    uniform mat4 projection;

    layout(points) in;
    layout(triangle_strip, max_vertices = 3) out;

    in vec4 gcolor[1];
    in float gfalloff[1];
    out vec2 delta;
    out vec4 fcolor;
    out float ffalloff;

    void main()
    {
        fcolor = gcolor[0];
        ffalloff = gfalloff[0];
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
    in vec4 fcolor;
    in float ffalloff;
    out vec4 color;
    void main() {
        color = vec4(fcolor.xyz, fcolor.a * max(0.0, 1.0 - pow(length(delta), ffalloff)));
    }
"#;

///Node is used to pass nodes into the renderer.
#[derive(Copy, Clone)]
pub struct Node {
    pub position: [f32; 3],
    pub color: [f32; 4],
    ///Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff: f32,
}

implement_vertex!(Node, position, color, falloff);

pub struct Renderer<'a> {
    display: &'a glium::Display,
    program: glium::Program,
}

///A Renderer is tied to the lifetime of the glium Display and making one builds a GLSL program internally.
impl<'a> Renderer<'a> {
    ///Make a new Renderer from a glium::Display.
    pub fn new(display: &'a glium::Display) -> Self {
        Renderer {
            display: display,
            program: glium::Program::from_source(display,
                VSHADER_SOURCE, FSHADER_SOURCE, Some(GSHADER_SOURCE)).unwrap(),
        }
    }

    ///Take a modelview matrix, projection matrix, and a series of nodes and draw them in parallel on the GPU.
    pub fn render_nodes(&self, modelview: &[[f32; 4]; 4], projection: &[[f32; 4]; 4], nodes: &[Node]) {
        let vertex_buffer = glium::VertexBuffer::new(self.display, nodes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: *modelview,
            projection: *projection,
        };

        let params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &self.program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }
}
