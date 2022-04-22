
layout (location = 0) in vec3 vertex; // the position variable has attribute position 0

out vec2 TexCoords;

void main() {
    gl_Position = vec4(vertex.xy, 0.0, 1.0); // see how we directly give a vec3 to vec4's constructor
    TexCoords = vertex.zw;
}
