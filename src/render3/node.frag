#version 150

in vec2 delta;
in vec4 finner_color;
in vec4 ffalloff_color;
in float finner_radius;
in float ffalloff_radius;
in float ffalloff;
out vec4 color;

void main() {
    float length = length(delta);
    if (length <= finner_radius) {
        float travel = length / finner_radius;
        // Manually interpolate the inner color into the falloff color.
        color = finner_color * (1 - travel) + ffalloff_color * travel;
    } else {
        color = vec4(ffalloff_color.xyz,
            ffalloff_color.a * max(0.0, 1.0 - pow((length - finner_radius) / ffalloff_radius, ffalloff)));
    }
}