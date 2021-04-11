#version 330 core

uniform sampler2D texture1;
uniform sampler2D texture2;

in vec2 texCoord;

out vec4 FragColor;
void main() {
    vec2 face_tex_coord = vec2(1.0 - texCoord.x, texCoord.y);
    FragColor = mix(texture(texture1, texCoord),
    texture(texture2, face_tex_coord), 0.2);
}