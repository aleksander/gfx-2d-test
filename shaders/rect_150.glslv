#version 150 core

in vec2 pos;
in vec3 color;
out vec4 v_color;

uniform Globals {
    mat2 view;
};

void main() {
    v_color = vec4(view[0][0], view[1][0], view[1][1], 1.0);
    gl_Position = vec4(pos, 0.0, 1.0);
}
