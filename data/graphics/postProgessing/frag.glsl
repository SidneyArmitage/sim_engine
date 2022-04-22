
in vec2 TexCoords;
out vec4 color;

uniform sampler2D scene;

void main() {
  color =  texture(scene, TexCoords);
}