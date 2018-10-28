use hal::buffer::Usage;
use hal::Primitive;
use nalgebra::Vector2;

pub type Vertex = Vector2<f32>;

#[derive(Debug)]
pub struct RenderableDefinition {
    pub id: String,
    pub fragment_shader: String,
    pub vertex_shader: String,
    pub inputs: Vec<InputDescriptor>,
    pub draw_mode: Primitive,
    pub vertices_to_render: usize,
}

#[derive(Debug)]
pub struct InputDescriptor {
    pub location: usize,
    pub buffer_type: Usage,
    //    pub buffer_data_type: u32,
    //    pub num_components: i32,
    pub vertices: Vec<Vertex>,
}
