use glium::{self, Surface};
use cgmath;

/// Node is used to pass nodes into the renderer.
#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub position: [f32; 2],
    pub inner_color: [f32; 4],
    /// Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff: f32,
    pub falloff_color: [f32; 4],
    pub falloff_radius: f32,
    pub inner_radius: f32,
}

impl From<cgmath::Point2<f32>> for Node {
    fn from(point: cgmath::Point2<f32>) -> Node {
        Node {
            position: point.into(),
            inner_color: [1.0, 1.0, 1.0, 1.0],
            falloff: 0.5,
            falloff_color: [1.0, 1.0, 1.0, 1.0],
            // These radii will only work in certian scenarios, but can be modified.
            falloff_radius: 0.01,
            inner_radius: 0.002,
        }
    }
}

implement_vertex!(
    Node,
    position,
    inner_color,
    falloff,
    falloff_color,
    falloff_radius,
    inner_radius
);

/// QBezier is used to pass a quadratic bezier curve into the shader with interpolating values.
#[derive(Copy, Clone, Debug)]
pub struct QBezier {
    pub position0: [f32; 2],
    pub position1: [f32; 2],
    pub position2: [f32; 2],
    pub inner_color0: [f32; 4],
    pub inner_color1: [f32; 4],
    pub falloff_color0: [f32; 4],
    pub falloff_color1: [f32; 4],
    /// Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff0: f32,
    pub falloff1: f32,
    pub falloff_radius0: f32,
    pub falloff_radius1: f32,
    pub inner_radius0: f32,
    pub inner_radius1: f32,
}

implement_vertex!(
    QBezier,
    position0,
    position1,
    position2,
    inner_color0,
    inner_color1,
    falloff0,
    falloff1,
    falloff_color0,
    falloff_color1,
    falloff_radius0,
    falloff_radius1,
    inner_radius0,
    inner_radius1
);

/// A Renderer is tied to the lifetime of the glium Display and making one builds a GLSL program internally.
pub struct Renderer<'a, D>
where
    D: 'a,
{
    display: &'a D,
    node_program: glium::Program,
    round_edge_program: glium::Program,
    flat_edge_program: glium::Program,
    round_qbezier_program: glium::Program,
    flat_qbezier_program: glium::Program,
    params: glium::DrawParameters<'a>,
}

impl<'a, D> Renderer<'a, D>
where
    D: glium::backend::Facade,
{
    /// Make a new Renderer from a Facade.
    pub fn new(display: &'a D) -> Self {
        Renderer {
            display: display,
            node_program: glium::Program::from_source(
                display,
                include_str!("node.vert"),
                include_str!("node.frag"),
                Some(include_str!("node.geom")),
            ).unwrap(),
            round_edge_program: glium::Program::from_source(
                display,
                include_str!("node.vert"),
                include_str!("node.frag"),
                Some(include_str!("round_edge.geom")),
            ).unwrap(),
            flat_edge_program: glium::Program::from_source(
                display,
                include_str!("node.vert"),
                include_str!("node.frag"),
                Some(include_str!("flat_edge.geom")),
            ).unwrap(),
            round_qbezier_program: glium::Program::from_source(
                display,
                include_str!("qbezier.vert"),
                include_str!("qbezier.frag"),
                Some(include_str!("round_qbezier.geom")),
            ).unwrap(),
            flat_qbezier_program: glium::Program::from_source(
                display,
                include_str!("qbezier.vert"),
                include_str!("qbezier.frag"),
                Some(include_str!("flat_qbezier.geom")),
            ).unwrap(),
            params: glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        }
    }

    /// Take a series of nodes and draw them in parallel on the GPU.
    pub fn render_nodes<S>(
        &self,
        target: &mut S,
        modelview: [[f32; 3]; 3],
        projection: [[f32; 3]; 3],
        nodes: &[Node],
    ) where
        S: Surface,
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, nodes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.node_program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }

    /// Take a series of lines (edges) and draw them in parallel on the GPU.
    ///
    /// These will have round ends.
    pub fn render_edges_round<S>(
        &self,
        target: &mut S,
        modelview: [[f32; 3]; 3],
        projection: [[f32; 3]; 3],
        edges: &[Node],
    ) where
        S: Surface,
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, edges).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.round_edge_program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }

    /// Take a series of lines (edges) and draw them in parallel on the GPU.
    ///
    /// These will have flat ends.
    pub fn render_edges_flat<S>(
        &self,
        target: &mut S,
        modelview: [[f32; 3]; 3],
        projection: [[f32; 3]; 3],
        edges: &[Node],
    ) where
        S: Surface,
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, edges).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.flat_edge_program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }

    /// Take a series of triangles (quadratic bezier curves) and draw them in parallel on the GPU.
    ///
    /// These will have round ends.
    pub fn render_qbeziers_round<S>(
        &self,
        target: &mut S,
        modelview: [[f32; 3]; 3],
        projection: [[f32; 3]; 3],
        qbeziers: &[QBezier],
    ) where
        S: Surface,
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, qbeziers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.round_qbezier_program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }

    /// Take a series of triangles (quadratic bezier curves) and draw them in parallel on the GPU.
    ///
    /// These will have flat ends.
    pub fn render_qbeziers_flat<S>(
        &self,
        target: &mut S,
        modelview: [[f32; 3]; 3],
        projection: [[f32; 3]; 3],
        qbeziers: &[QBezier],
    ) where
        S: Surface,
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, qbeziers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &self.flat_qbezier_program,
                &uniforms,
                &self.params,
            )
            .unwrap();
    }
}
