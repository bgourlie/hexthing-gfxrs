#[derive(Debug)]
pub struct RenderableDefinition {
    id: String,
    fragment_shader: String,
    vertex_shader: String,
    inputs: Vec<InputDescriptor>,
    draw_mode: u32,
    vertices_to_render: i32,
}

#[derive(Debug)]
pub struct InputDescriptor {
    location: u32,
    buffer_type: u32,
    buffer_data_type: u32,
    num_components: i32,
    vertices: Float32Array,
}