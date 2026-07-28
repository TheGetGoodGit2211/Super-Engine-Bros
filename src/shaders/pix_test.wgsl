struct ScreenUniform {
    size: vec2<f32>,
};

@group(0) @binding(0) 
var<uniform> u_screen: ScreenUniform;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Convert pixel position (X: 0..width, Y: 0..height) to NDC space (-1.0..1.0)
    let ndc_x = (2.0 * model.position.x / u_screen.size.x) - 1.0;
    let ndc_y = 1.0 - (2.0 * model.position.y / u_screen.size.y);

    out.clip_position = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color; // Pass through the vertex color
}
