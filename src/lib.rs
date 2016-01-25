#[macro_use]
extern crate glium;
extern crate petgraph;
extern crate zoom;
extern crate num;

pub fn render_nodes<I>(nodes: I)
    where I: Iterator<Item=zoom::Cartesian2<f32>>
{
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let out_triangle = [
        zoom::Cartesian2{x: 0.0, y: 2.0}/32.0,
        zoom::Cartesian2{x: -1.7320508075689, y: -1.0}/32.0,
        zoom::Cartesian2{x: 1.7320508075689, y: -1.0}/32.0,
    ];

    let shape: Vec<_> = nodes.flat_map(|n| {
        out_triangle.iter().map(move |&v| (v + n)).map(|v| Vertex{position: [v.x, v.y]})
    }).collect();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        out vec2 my_attr;
        uniform mat4 matrix;
        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 my_attr;
        out vec4 color;
        void main() {
            color = vec4(my_attr, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src,
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

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
