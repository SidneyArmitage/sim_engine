#version 330 core
layout (location = 0) in vec3 vertex; // the position variable has attribute position 0

uniform float u_aspectRatio;

void main()
{
    gl_Position = vec4(vertex.x * u_aspectRatio, vertex.y, 0.0, 1.0); // see how we directly give a vec3 to vec4's constructor
}
