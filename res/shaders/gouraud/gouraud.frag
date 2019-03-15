#version 150

in vec3 v_normal;
out vec4 color;
uniform vec3 light_dir;

void main() {
    float brightness = dot(normalize(v_normal), normalize(light_dir));
    vec3 dark_color = vec3(0.6, 0.6, 0.6);
    vec3 regular_color = vec3(0.9, 0.9, 0.9);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}