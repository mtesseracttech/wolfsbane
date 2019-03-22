#version 150

in vec3 v_normal;
out vec4 color;

void main() {
    vec3 normal_color = vec3(v_normal.x, v_normal.y, v_normal.z);
    color = vec4(normal_color, 1.0);
}
