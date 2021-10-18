pub const VERTEX_SHADER_SRC_GLSL: &'static str = r#"
# version 140

in vec4 rgba;
in vec2 coords;

out vec4 color;

uniform float angle;

vec2 rotate(vec2 v, float a) {
    float s = sin(a);
    float c = cos(a);
    mat2 m = mat2(
        c, -s,
        s, c
    );

    return m * v;
}

void main() {
    gl_Position = vec4(rotate(coords, angle), 0.0, 1.0);
    color = rgba;
}
"#;

pub const FRAGMENT_SHADER_SRC_GLSL: &'static str = r#"
# version 140

in vec4 color;
out vec4 FragColor;

void main() {
    FragColor = color;
}
"#;

