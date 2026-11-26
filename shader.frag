#version 450
layout(location=0) in vec3 v_normal;
layout(location=1) noperspective in vec2 v_uv;

layout(set=0, binding=1) uniform sampler2D tex;

layout(location=0) out vec4 out_color;

void main() {
    vec3 light = normalize(vec3(0.3, 1.0, 0.4));  // один источник, как в PS1
    float ndl = max(dot(normalize(v_normal), light), 0.0);
    ndl = floor(ndl * 4.0) / 4.0;  // 4 ступени яркости

    vec4 tex_col = texture(tex, v_uv);
    // квантуем в 16-битную палитру (5-5-5 или 4-4-4)
    vec3 col = floor(tex_col.rgb * ndl * 15.0) / 15.0;

    out_color = vec4(col, 1.0);
}
