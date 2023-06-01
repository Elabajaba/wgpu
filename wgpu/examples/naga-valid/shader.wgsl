@group(0) @binding(0)
var<storage, read_write> v_out: u32;
@group(0) @binding(1)
var<storage, read_write> v_indices: vec4<f32>; // this is used as both input and output for convenience

@compute
@workgroup_size(1)
fn p4x8u(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_out = pack4x8unorm(v_indices);
}

@compute
@workgroup_size(1)
fn p4x8s(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_out = pack4x8snorm(v_indices);
}

@compute
@workgroup_size(1)
fn p2x16f(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_out = pack2x16float(v_indices.xy);
}

@compute
@workgroup_size(1)
fn p2x16s(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_out = pack2x16snorm(v_indices.xy);
}

@compute
@workgroup_size(1)
fn p2x16u(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_out = pack2x16unorm(v_indices.xy);
}
