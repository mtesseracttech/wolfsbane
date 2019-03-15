#version 140

in vec2 position;
in vec3 color;
out vec3 vertColor;

uniform mat4 model;

void main() {
    vertColor= color;
    gl_Position = model * vec4(position, 0.0, 1.0);
}