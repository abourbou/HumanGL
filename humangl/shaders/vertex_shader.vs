#version 440 core

// Input vertex data, different for all executions of this shader.
layout (location = 0) in vec3 pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 color;

out vec3 fragmentColor;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    fragmentColor = color;

} 