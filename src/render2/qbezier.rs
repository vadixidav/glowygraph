pub static VSHADER_SOURCE: &'static str = r#"
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
    in int accuracy;

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
    out int gaccuracy;

    void main() {
        gposition0 = position0;
        gposition1 = position1;
        gposition2 = position2;
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
        gaccuracy = accuracy;
        gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
    }
"#;

pub static GSHADER_SOURCE: &'static str = r#"
    #version 150

    layout(points) in;
    layout(triangle_strip, max_vertices = 9) out;

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
    in int gaccuracy[1];

    out vec2 fposition0;
    out vec2 fposition1;
    out vec2 fposition2;
    out vec4 finner_color0;
    out vec4 finner_color1;
    out float ffalloff0;
    out float ffalloff1;
    out vec4 ffalloff_color0;
    out vec4 ffalloff_color1;
    out float ffalloff_radius0;
    out float ffalloff_radius1;
    out float finner_radius0;
    out float finner_radius1;
    out vec2 realpos;
    flat out int faccuracy;

    void main() {
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
        faccuracy = gaccuracy[0];

        vec2 l0 = normalize(gposition1[0] - gposition0[0]);
        vec2 l1 = normalize(gposition2[0] - gposition1[0]);
        vec2 l2 = normalize(gposition0[0] - gposition2[0]);

        vec2 b0 = normalize(l0 - l2);
        vec2 b1 = normalize(l0 - l1);
        vec2 b2 = normalize(l2 - l1);

        float radius0 = finner_radius0 + ffalloff_radius0;
        float radius2 = finner_radius1 + ffalloff_radius1;
        float radius1 = (radius0 + radius2) * 0.5;

        vec4 e0 = vec4(gposition0[0] + radius0 * vec2(b0.y, -b0.x), 0.0, 1.0);
        vec4 e1 = vec4(gposition0[0] + radius0 * vec2(-b0.y, b0.x), 0.0, 1.0);
        vec4 e2 = vec4(gposition1[0] + radius1 * b1, 0.0, 1.0);
        vec4 e3 = vec4(gposition2[0] + radius2 * vec2(b2.y, -b2.x), 0.0, 1.0);
        vec4 e4 = vec4(gposition2[0] + radius2 * vec2(-b2.y, b2.x), 0.0, 1.0);

        gl_Position = e2;
        realpos = e2.xy;
        EmitVertex();

        gl_Position = e1;
        realpos = e1.xy;
        EmitVertex();

        gl_Position = e0;
        realpos = e0.xy;
        EmitVertex();

        EndPrimitive();

        gl_Position = e2;
        realpos = e2.xy;
        EmitVertex();

        gl_Position = e0;
        realpos = e0.xy;
        EmitVertex();

        gl_Position = e4;
        realpos = e4.xy;
        EmitVertex();

        EndPrimitive();

        gl_Position = e2;
        realpos = e2.xy;
        EmitVertex();

        gl_Position = e4;
        realpos = e4.xy;
        EmitVertex();

        gl_Position = e3;
        realpos = e3.xy;
        EmitVertex();

        EndPrimitive();
    }
"#;

pub static FSHADER_SOURCE: &'static str = r#"
    #version 150

    in vec2 fposition0;
    in vec2 fposition1;
    in vec2 fposition2;
    in vec4 finner_color0;
    in vec4 finner_color1;
    in float ffalloff0;
    in float ffalloff1;
    in vec4 ffalloff_color0;
    in vec4 ffalloff_color1;
    in float ffalloff_radius0;
    in float ffalloff_radius1;
    in float finner_radius0;
    in float finner_radius1;
    in vec2 realpos;
    flat in int faccuracy;

    out vec4 color;

    void main() {
        bool best_begin;
        float best_distance;
        float best_begin_t = 0.0;
        float best_end_t = 1.0;

        float beginning_distance = length(fposition0 - realpos);
        float ending_distance = length(fposition2 - realpos);
        if (beginning_distance < ending_distance) {
            best_begin = true;
            best_distance = beginning_distance;
        } else {
            best_begin = false;
            best_distance = ending_distance;
        }

        for (int i = 0; i < faccuracy; i++) {
            // Solve the bezier curve distance between beginning and end.
            float middle_t = (best_begin_t + best_end_t) * 0.5;
            float distance = length(
                pow(1.0 - middle_t, 2) * fposition0 +
                2.0 * middle_t * (1.0 - middle_t) * fposition1 +
                pow(middle_t, 2) * fposition2 -
                realpos);

            if (distance < best_distance) {
                if (best_begin) {
                    best_begin = false;
                    best_distance = distance;
                    best_end_t = middle_t;
                } else {
                    best_begin = true;
                    best_distance = distance;
                    best_begin_t = middle_t;
                }
            } else {
                if (best_begin) {
                    best_end_t = middle_t;
                } else {
                    best_begin_t = middle_t;
                }
            }
        }

        float best_t;
        if (best_begin) {
            best_t = best_begin_t;
        } else {
            best_t = best_end_t;
        }

        vec4 inner_color = finner_color0 + best_t * (finner_color1 - finner_color0);
        float falloff = ffalloff0 + best_t * (ffalloff1 - ffalloff0);
        vec4 falloff_color = ffalloff_color0 + best_t * (ffalloff_color1 - ffalloff_color0);
        float falloff_radius = ffalloff_radius0 + best_t * (ffalloff_radius1 - ffalloff_radius0);
        float inner_radius = finner_radius0 + best_t * (finner_radius1 - finner_radius0);

        if (best_distance <= inner_radius) {
            float travel = best_distance / inner_radius;
            // Manually interpolate the inner color into the falloff color.
            color = inner_color * (1.0 - travel) + falloff_color * travel;
        } else {
            color = vec4(falloff_color.xyz,
                falloff_color.a * max(0.0, 1.0 - pow((best_distance - inner_radius) / falloff_radius, falloff)));
        }
    }
"#;
