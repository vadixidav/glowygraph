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
"#;

pub static GSHADER_SOURCE_ROUND: &'static str = r#"
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
        vec2 l2 = normalize(gposition0[0] - gposition2[0]);

        vec2 b0 = normalize(l0 - l2);
        vec2 b1 = normalize(l0 - l1);
        vec2 b2 = normalize(l2 - l1);

        float radius0 = finner_radius0 + ffalloff_radius0;
        float radius2 = finner_radius1 + ffalloff_radius1;
        float radius1 = (radius0 + radius2) * 0.5;

        vec2 e0 = gposition0[0] + radius0 * vec2(b0.y, -b0.x) - radius0 * b0;
        vec2 e1 = gposition0[0] + radius0 * vec2(-b0.y, b0.x) - radius0 * b0;
        vec2 e2 = gposition1[0] + radius1 * b1;
        vec2 e3 = gposition2[0] + radius2 * vec2(b2.y, -b2.x) - radius2 * b2;
        vec2 e4 = gposition2[0] + radius2 * vec2(-b2.y, b2.x) - radius2 * b2;

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
"#;

pub static GSHADER_SOURCE_FLAT: &'static str = r#"
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
"#;

pub static FSHADER_SOURCE: &'static str = r#"
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

    float det(vec2 a, vec2 b) { return a.x*b.y-b.x*a.y; }

    #define EPSILON 0.000000001
    #define MAX 9999999.
    #define PI 3.14159265358979

    int findRoots(float a, float b, float c, float d, out float r[3])
    {
    	vec3 vS = vec3(-1.0,-1.0,-1.0);
    	if (abs(a) > EPSILON)
    	{
    		float z = 1.0/a;
    		float d3 = 1.0/3.0;
    		float d27 = 1.0/27.0;
    		a = b*z;
    		b = c*z;
    		c = d*z;
    		float p = b-a*a*d3;
    		float q = a*(2.0*a*a-9.0*b)*d27+c;
    		float ppp = p*p*p;
    		float D = q*q+4.0*ppp*d27;
    		float delta = -a*d3;
    		if (D > EPSILON)
    		{
    			z = sqrt(D);
    			float u = (-q+z)*0.5;
    			float v = (-q-z)*0.5;
    			u = sign(u)*pow(abs(u),d3);
    			v = sign(v)*pow(abs(v),d3);
    			r[0] = u+v+delta;
    			return 1;
    		}
    		else if (D < -EPSILON)
    		{
    			float u = sqrt(-p*d3)*2.0;
                float s = -sqrt(-27.0/ppp)*q*0.5;
                if (abs(s) > 0.) {}
    			float v = acos(s)*d3;
    			r[0] = u*cos(v)+delta;
    			r[1] = u*cos(v+2.0*PI*d3)+delta;
    			r[2] = u*cos(v+4.0*PI*d3)+delta;
    			return 3;
    		}
    		else
    		{
    			q = sign(q)*pow(abs(q)*0.5,d3);
    			r[0] = 2.0*-q+delta;
    			r[1] = q+delta;
    			return 2;
    		}
    	}
    	else
    	{
    		if (abs(b) <= EPSILON && abs(c) > EPSILON)
    		{
    			r[0] = -d/c;
    			return 1;
    		}
    		else
    		{
    			float D = c*c-4.0*b*d;
    			float z = 1.0/(2.0*b);
    			if (D > EPSILON)
    			{
    				D = sqrt(D);
    				r[0] = (-c-D)*z;
    				r[1] = (-c+D)*z;
    				return 2;
    			}
    			else
    			{
    				r[0] = -c*z;
    				return 1;
    			}
    		}
    	}
    	return 0;
    }

    void clampRoots(inout float r[3]) {
        r[0] = clamp(r[0], 0.0, 1.0);
        r[1] = clamp(r[1], 0.0, 1.0);
        r[2] = clamp(r[2], 0.0, 1.0);
    }

    vec2 getPositionOnBezierCurve(float t, vec2 p0, vec2 p1, vec2 p2)
    {
    	float fOneMinusT = 1.0-t;
    	vec2 pos = fOneMinusT*fOneMinusT*p0+2.0*t*fOneMinusT*p1+t*t*p2;
    	return pos;
    }

    float calculateDistanceToQuadraticBezier(vec2 p, vec2 p0, vec2 p1, vec2 p2, out float t)
    {
    	vec2 dP0P = p0-p;
    	vec2 dP1P0 = p1-p0;
    	vec2 sP0P2 = p0+p2-p1*2.0;
    	float a = dot(sP0P2,sP0P2);
    	float b = dot(dP1P0,sP0P2)*3.0;
    	float c = dot(dP1P0,dP1P0)*2.0+dot(dP0P, sP0P2);
    	float d = dot(dP0P,dP1P0);
    	float r[3];
    	int roots = findRoots(a,b,c,d,r);
        clampRoots(r);
    	float dist = distance(p,getPositionOnBezierCurve(r[0],p0,p1,p2));
        t = r[0];
        if (roots > 1) {
            float bestdist = min(dist, distance(p,getPositionOnBezierCurve(r[1],p0,p1,p2)));
            if (bestdist < dist) {
                dist = bestdist;
                t = r[1];
            }
        }
        if (roots > 2) {
            float bestdist = min(dist, distance(p,getPositionOnBezierCurve(r[2],p0,p1,p2)));
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
"#;
