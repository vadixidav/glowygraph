#version 150

uniform mat4 projection;

layout(lines) in;
layout(triangle_strip, max_vertices = 12) out;

in vec4 ginner_color[2];
in vec4 gfalloff_color[2];
in float gfalloff[2];
in float gfalloff_radius[2];
in float ginner_radius[2];
out vec2 delta;
out vec4 finner_color;
out vec4 ffalloff_color;
out float finner_radius;
out float ffalloff_radius;
out float ffalloff;

void main() {
    vec4 first = gl_in[0].gl_Position;
    vec4 second = gl_in[1].gl_Position;

    vec3 full_delta = 2 * normalize(second.xyz - first.xyz);
    vec2 net_delta = 2 * normalize(second.xy - first.xy);

    float radius;

    //Face 0

    //Vertex 0
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(net_delta.y, -net_delta.x);
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 1
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * net_delta;
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 2
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(-net_delta.y, net_delta.x);
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    EndPrimitive();

    //Face 1

    //Vertex 0
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(net_delta.y, -net_delta.x);
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 2
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(-net_delta.y, net_delta.x);
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 3
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(net_delta.y, -net_delta.x);
    gl_Position = projection * (second - vec4(delta, 0, 0));
    EmitVertex();

    EndPrimitive();

    //Face 2

    //Vertex 2
    finner_color = ginner_color[0];
    ffalloff_color = gfalloff_color[0];
    finner_radius = ginner_radius[0];
    ffalloff_radius = gfalloff_radius[0];
    ffalloff = gfalloff[0];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(-net_delta.y, net_delta.x);
    gl_Position = projection * (first - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 4
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(-net_delta.y, net_delta.x);
    gl_Position = projection * (second - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 3
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(net_delta.y, -net_delta.x);
    gl_Position = projection * (second - vec4(delta, 0, 0));
    EmitVertex();

    EndPrimitive();

    //Face 3

    //Vertex 5
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * net_delta;
    gl_Position = projection * (second + vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 3
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(net_delta.y, -net_delta.x);
    gl_Position = projection * (second - vec4(delta, 0, 0));
    EmitVertex();

    //Vertex 4
    finner_color = ginner_color[1];
    ffalloff_color = gfalloff_color[1];
    finner_radius = ginner_radius[1];
    ffalloff_radius = gfalloff_radius[1];
    ffalloff = gfalloff[1];
    radius = finner_radius + ffalloff_radius;
    delta = radius * vec2(-net_delta.y, net_delta.x);
    gl_Position = projection * (second - vec4(delta, 0, 0));
    EmitVertex();

    EndPrimitive();
}