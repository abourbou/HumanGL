#version 440 core

// Input vertex data, different for all executions of this shader.
layout (location = 0) in vec3 pos;

uniform mat4 MVP;
uniform vec3 color;

out vec3 fragmentColor;

void main() {
    gl_Position = MVP * vec4(pos, 1.0);
    fragmentColor = color;

} 