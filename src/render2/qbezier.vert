#version 150

in vec2 position0;
in vec2 position1;
in vec2 position2;
in vec4 inner_color0;
in vec4 inner_color1;
in float falloff0;
in float falloff1;
in vec4 falloff_color0;
in vec4 falloff_color1;
in float falloff_radius0;
in float falloff_radius1;
in float inner_radius0;
in float inner_radius1;

out vec2 gposition0;
out vec2 gposition1;
out vec2 gposition2;
out vec4 ginner_color0;
out vec4 ginner_color1;
out float gfalloff0;
out float gfalloff1;
out vec4 gfalloff_color0;
out vec4 gfalloff_color1;
out float gfalloff_radius0;
out float gfalloff_radius1;
out float ginner_radius0;
out float ginner_radius1;

uniform mat3 modelview;

void main() {
    vec2 wigglepos1;
    // Determine if we need to wiggle
    vec2 norm02 = normalize(position2 - position0);
    vec2 norm12 = normalize(position2 - position1);
    float flatness = dot(norm02, norm12);
    // We must wiggle (angle incredibly small or flat)
    if (flatness > 0.995) {
        // If its perfectly flat, we cant know the direction to wiggle, so we must go perpendicular to the norm02
        vec2 wiggle_vector = vec2(-norm02.y, norm02.x);
        float scale = length(position2 - position1);
        // Wiggle by 2 percent of the scale
        wigglepos1 = position1 + 0.005 * scale * wiggle_vector;
    } else {
        wigglepos1 = position1;
    }
    // Find clockwise vs counter-clockwise
    float cc =
        (wigglepos1.x - position0.x) * (wigglepos1.y + position0.y) +
        (position2.x - wigglepos1.x) * (position2.y + wigglepos1.y) +
        (position0.x - position2.x) * (position0.y + position2.y);
    gposition1 = (modelview * vec3(wigglepos1, 1.0)).xy;
    if (cc > 0.0) {
        gposition0 = (modelview * vec3(position0, 1.0)).xy;
        gposition2 = (modelview * vec3(position2, 1.0)).xy;
        ginner_color0 = inner_color0;
        ginner_color1 = inner_color1;
        gfalloff0 = falloff0;
        gfalloff1 = falloff1;
        gfalloff_color0 = falloff_color0;
        gfalloff_color1 = falloff_color1;
        gfalloff_radius0 = falloff_radius0;
        gfalloff_radius1 = falloff_radius1;
        ginner_radius0 = inner_radius0;
        ginner_radius1 = inner_radius1;
    } else {
        gposition0 = (modelview * vec3(position2, 1.0)).xy;
        gposition2 = (modelview * vec3(position0, 1.0)).xy;
        ginner_color0 = inner_color1;
        ginner_color1 = inner_color0;
        gfalloff0 = falloff1;
        gfalloff1 = falloff0;
        gfalloff_color0 = falloff_color1;
        gfalloff_color1 = falloff_color0;
        gfalloff_radius0 = falloff_radius1;
        gfalloff_radius1 = falloff_radius0;
        ginner_radius0 = inner_radius1;
        ginner_radius1 = inner_radius0;
    }
}