#version 150

flat in vec2 fposition0;
flat in vec2 fposition1;
flat in vec2 fposition2;
flat in vec4 finner_color0;
flat in vec4 finner_color1;
flat in float ffalloff0;
flat in float ffalloff1;
flat in vec4 ffalloff_color0;
flat in vec4 ffalloff_color1;
flat in float ffalloff_radius0;
flat in float ffalloff_radius1;
flat in float finner_radius0;
flat in float finner_radius1;
in vec2 realpos;

out vec4 color;

float det(vec2 a, vec2 b) { return a.x * b.y - b.x * a.y; }

#define EPSILON 0.000000001
#define MAX 9999999.
#define PI 3.14159265358979

int findRoots(float a, float b, float c, float d, out float r[3])
{
    vec3 vS = vec3(-1.0, -1.0, -1.0);
    if (abs(a) > EPSILON) {
        float z = 1.0 / a;
        float d3 = 1.0 / 3.0;
        float d27 = 1.0 / 27.0;
        a = b * z;
        b = c * z;
        c = d * z;
        float p = b - a * a * d3;
        float q = a * (2.0 * a * a - 9.0 * b) * d27 + c;
        float ppp = p * p * p;
        float D = q * q + 4.0 * ppp * d27;
        float delta = -a * d3;
        if (D > EPSILON) {
            z = sqrt(D);
            float u = (-q + z) * 0.5;
            float v = (-q - z) * 0.5;
            u = sign(u) * pow(abs(u), d3);
            v = sign(v) * pow(abs(v), d3);
            r[0] = u + v + delta;
            return 1;
        } else if (D < -EPSILON) {
            float u = sqrt(-p * d3) * 2.0;
            float s = -sqrt(-27.0 / ppp) * q * 0.5;
            if (abs(s) > 0.) {}
            float v = acos(s) * d3;
            r[0] = u * cos(v) + delta;
            r[1] = u * cos(v + 2.0 * PI * d3) + delta;
            r[2] = u * cos(v + 4.0 * PI * d3) + delta;
            return 3;
        } else {
            q = sign(q) * pow(abs(q) * 0.5, d3);
            r[0] = 2.0 * -q + delta;
            r[1] = q + delta;
            return 2;
        }
    } else {
        if (abs(b) <= EPSILON && abs(c) > EPSILON) {
            r[0] = -d / c;
            return 1;
        } else {
            float D = c * c - 4.0 * b * d;
            float z = 1.0 / (2.0 * b);
            if (D > EPSILON) {
                D = sqrt(D);
                r[0] = (-c - D) * z;
                r[1] = (-c + D) * z;
                return 2;
            } else {
                r[0] = -c * z;
                return 1;
            }
        }
    }
    return 0;
}

void clampRoots(inout float r[3])
{
    r[0] = clamp(r[0], 0.0, 1.0);
    r[1] = clamp(r[1], 0.0, 1.0);
    r[2] = clamp(r[2], 0.0, 1.0);
}

vec2 getPositionOnBezierCurve(float t, vec2 p0, vec2 p1, vec2 p2)
{
    float fOMT = 1.0 - t;
    vec2 pos = fOMT * fOMT * p0 + 2.0 * t * fOMT * p1 + t * t * p2;
    return pos;
}

float calculateDistanceToQuadraticBezier(vec2 p, vec2 p0, vec2 p1, vec2 p2, out float t)
{
    vec2 dP0P = p0 - p;
    vec2 dP1P0 = p1 - p0;
    vec2 sP0P2 = p0 + p2 - p1 * 2.0;
    float a = dot(sP0P2, sP0P2);
    float b = dot(dP1P0, sP0P2) * 3.0;
    float c = dot(dP1P0, dP1P0) * 2.0 + dot(dP0P, sP0P2);
    float d = dot(dP0P, dP1P0);
    float r[3];
    int roots = findRoots(a, b, c, d, r);
    clampRoots(r);
    float dist = distance(p, getPositionOnBezierCurve(r[0], p0, p1, p2));
    t = r[0];
    if (roots > 1) {
        float bestdist = min(dist, distance(p, getPositionOnBezierCurve(r[1], p0, p1, p2)));
        if (bestdist < dist) {
            dist = bestdist;
            t = r[1];
        }
    }
    if (roots > 2) {
        float bestdist = min(dist, distance(p, getPositionOnBezierCurve(r[2], p0, p1, p2)));
        if (bestdist < dist) {
            dist = bestdist;
            t = r[2];
        }
    }
    return dist;
}

void main() {
    float best_t;
    float best_distance = calculateDistanceToQuadraticBezier(realpos, fposition0, fposition1, fposition2, best_t);

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