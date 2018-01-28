#version 150

layout(points) in;
layout(triangle_strip, max_vertices = 5) out;

in vec2 gposition0[1];
in vec2 gposition1[1];
in vec2 gposition2[1];
in vec4 ginner_color0[1];
in vec4 ginner_color1[1];
in float gfalloff0[1];
in float gfalloff1[1];
in vec4 gfalloff_color0[1];
in vec4 gfalloff_color1[1];
in float gfalloff_radius0[1];
in float gfalloff_radius1[1];
in float ginner_radius0[1];
in float ginner_radius1[1];

flat out vec2 fposition0;
flat out vec2 fposition1;
flat out vec2 fposition2;
flat out vec4 finner_color0;
flat out vec4 finner_color1;
flat out float ffalloff0;
flat out float ffalloff1;
flat out vec4 ffalloff_color0;
flat out vec4 ffalloff_color1;
flat out float ffalloff_radius0;
flat out float ffalloff_radius1;
flat out float finner_radius0;
flat out float finner_radius1;
out vec2 realpos;

uniform mat3 projection;

void main() {
    fposition0 = gposition0[0];
    fposition1 = gposition1[0];
    fposition2 = gposition2[0];
    finner_color0 = ginner_color0[0];
    finner_color1 = ginner_color1[0];
    ffalloff0 = gfalloff0[0];
    ffalloff1 = gfalloff1[0];
    ffalloff_color0 = gfalloff_color0[0];
    ffalloff_color1 = gfalloff_color1[0];
    ffalloff_radius0 = gfalloff_radius0[0];
    ffalloff_radius1 = gfalloff_radius1[0];
    finner_radius0 = ginner_radius0[0];
    finner_radius1 = ginner_radius1[0];

    vec2 l0 = normalize(gposition1[0] - gposition0[0]);
    vec2 l1 = normalize(gposition2[0] - gposition1[0]);

    vec2 b1 = normalize(l0 - l1);

    float radius0 = finner_radius0 + ffalloff_radius0;
    float radius2 = finner_radius1 + ffalloff_radius1;
    float radius1 = (radius0 + radius2) * 0.5;

    vec2 e0 = gposition0[0] + radius0 * vec2(l0.y, -l0.x);
    vec2 e1 = gposition0[0] + radius0 * vec2(-l0.y, l0.x);
    vec2 e2 = gposition1[0] + radius1 * b1;
    vec2 e3 = gposition2[0] - radius2 * vec2(l1.y, -l1.x);
    vec2 e4 = gposition2[0] - radius2 * vec2(-l1.y, l1.x);

    gl_Position = vec4((projection * vec3(e1, 1.0)).xy, 0.0, 1.0);
    realpos = e1;
    EmitVertex();

    gl_Position = vec4((projection * vec3(e0, 1.0)).xy, 0.0, 1.0);
    realpos = e0;
    EmitVertex();

    gl_Position = vec4((projection * vec3(e2, 1.0)).xy, 0.0, 1.0);
    realpos = e2;
    EmitVertex();

    gl_Position = vec4((projection * vec3(e4, 1.0)).xy, 0.0, 1.0);
    realpos = e4;
    EmitVertex();

    gl_Position = vec4((projection * vec3(e3, 1.0)).xy, 0.0, 1.0);
    realpos = e3;
    EmitVertex();
}