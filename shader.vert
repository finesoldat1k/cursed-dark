#version 450
layout(location=0) in vec3 in_pos;
layout(location=1) in vec3 in_normal;
layout(location=2) in vec2 in_uv;

layout(set=0, binding=0) uniform Camera {
    mat4 view_proj;
    vec2 render_res;  // например (320.0, 240.0)
} u;

layout(location=0) out vec3 v_normal;
layout(location=1) noperspective out vec2 v_uv;  // <-- affine-маппинг!

void main() {
    vec4 clip = u.view_proj * vec4(in_pos, 1.0);

    // PS1 snapping: квантуем позицию на пиксельную сетку в screen-space
    vec2 grid = u.render_res * 0.5;
    clip.xy = floor((clip.xy / clip.w) * grid) / grid * clip.w;

    gl_Position = clip;
    v_normal = in_normal;
    v_uv = in_uv;
}
