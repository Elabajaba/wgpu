@group(0) @binding(0)
var<storage, read_write> v_out: u32;
@group(0) @binding(1)
var<storage, read_write> v_indices: vec4<f32>; // this is used as both input and output for convenience

// Unpacking
@compute
@workgroup_size(1)
fn unp4x8u(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices = unpack4x8unorm(v_out);
}

@compute
@workgroup_size(1)
fn unp4x8s(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices = unpack4x8snorm(v_out);
}

@compute
@workgroup_size(1)
fn unp2x16f(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices = vec4<f32>(unpack2x16float(v_out), 0.0, 0.0);
}

@compute
@workgroup_size(1)
fn unp2x16s(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices = vec4<f32>(unpack2x16snorm(v_out), 0.0, 0.0);
}

@compute
@workgroup_size(1)
fn unp2x16u(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices = vec4<f32>(unpack2x16unorm(v_out), 0.0, 0.0);
}
