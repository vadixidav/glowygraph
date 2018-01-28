#version 150

in vec3 position;
in vec4 inner_color;
in vec4 falloff_color;
in float falloff;
in float falloff_radius;
in float inner_radius;
out vec4 ginner_color;
out vec4 gfalloff_color;
out float gfalloff;
out float gfalloff_radius;
out float ginner_radius;
uniform mat4 modelview;

void main() {
    ginner_color = inner_color;
    gfalloff_color = falloff_color;
    gfalloff = falloff;
    gfalloff_radius = falloff_radius;
    ginner_radius = inner_radius;
    gl_Position = modelview * vec4(position, 1.0);
}