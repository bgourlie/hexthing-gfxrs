#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform UBOCol {
    vec4 color;
} color_dat;

void main() {
    target0 = color_dat.color;
}