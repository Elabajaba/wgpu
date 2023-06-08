struct ExtBitsU {
    value: vec4<u32>,
    offset: u32,
    count: u32,
    test_width: u32,
    empty: u32,
}

struct ExtBitsI {
    value: vec4<i32>,
    offset: u32,
    count: u32,
    test_width: u32,
    empty: u32,
}

@group(0) @binding(0)
var<storage, read_write> v_out_u: vec4<u32>;
@group(0) @binding(1)
var<storage, read_write> v_out_i: vec4<i32>;
@group(0) @binding(2)
var<storage, read_write> ext_bits_u: ExtBitsU;
@group(0) @binding(3)
var<storage, read_write> ext_bits_i: ExtBitsI;

// Unpacking unsigned
@compute
@workgroup_size(1)
fn ext_test(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // v_out_u = vec4<u32>(insertBits(ext_bits_u.value.x, ext_bits_u.newbits.x, ext_bits_u.offset, ext_bits_u.count), 0u, 0u, 0u);
    if ext_bits_u.test_width == 1u {
        v_out_u = vec4<u32>(extractBits(ext_bits_u.value.x, ext_bits_u.offset, ext_bits_u.count), 0u, 0u, 0u);
        v_out_i = vec4<i32>(extractBits(ext_bits_i.value.x, ext_bits_i.offset, ext_bits_i.count), 0, 0, 0);
    } else if ext_bits_u.test_width == 4u {
        v_out_u = extractBits(ext_bits_u.value, ext_bits_u.offset, ext_bits_u.count);
        v_out_i = extractBits(ext_bits_i.value, ext_bits_i.offset, ext_bits_i.count);
    }
}

// fn myextractBits(e: u32, offset: u32, count: u32) -> u32 {
// //     // gap = max(0, w - (offset + count))
// //     // ( value << gap ) >> offset
// //     let gap = max(0u, 32u - (offset + (32u - (count + offset))));
// //     return (e << gap) >> offset;

//     return ((e >> offset) & (count == 32u ? 0xffffffffu : ((1 << count) - 1)))
// }