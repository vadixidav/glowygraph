#version 150

uniform mat4 projection;

layout(points) in;
layout(triangle_strip, max_vertices = 3) out;

in vec4 ginner_color[1];
in vec4 gfalloff_color[1];
in float gfalloff[1];
in float gfalloff_radius[1];
in float ginner_radius[1];
out vec2 delta;
out vec4 finner_color;
out vec4 ffalloff_color;
out float finner_radius;
out float ffalloff_radius;
out float ffalloff;

void main() {
    finner_color = ginner_color[0];
    ffalloff_color = ginner_color[0];
    finner_radius = ginner_radius[0];
    ffalloff = gfalloff[0];
    ffalloff_radius = gfalloff_radius[0];
    vec4 center = gl_in[0].gl_Position;
    float full_radius = finner_radius + ffalloff_radius;

    delta = full_radius * vec2(0, 2);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();

    delta = full_radius * vec2(-1.7320508075689, -1);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();

    delta = full_radius * vec2(1.7320508075689, -1);
    gl_Position = projection * (center + vec4(delta, 0, 0));
    EmitVertex();
}