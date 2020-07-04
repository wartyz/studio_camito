#version 430

attribute vec3 a_position;

uniform mat4 u_projection;
uniform mat4 u_model;// matriz de transformación

void main() {
    gl_Position = u_projection * u_model * vec4(a_position, 1.0);
}
