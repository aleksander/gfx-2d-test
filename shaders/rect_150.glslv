#version 150 core

in vec2 a_Pos;
in vec3 a_Color;
out vec4 v_Color;

uniform Locals {
    mat2 u_view;
};

void main() {
    v_Color = vec4(u_view[0][0], u_view[0][1], u_view[1][1], 1.0);
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
