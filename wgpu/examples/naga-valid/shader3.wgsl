struct InsBitsU {
    value: vec4<u32>,
    newbits: vec4<u32>,
    offset: u32,
    count: u32,
    test_width: u32,
    empty: u32,
}

struct InsBitsI {
    value: vec4<i32>,
    newbits: vec4<i32>,
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
var<storage, read_write> ins_bits_u: InsBitsU;
@group(0) @binding(3)
var<storage, read_write> ins_bits_i: InsBitsI;

// Unpacking unsigned
@compute
@workgroup_size(1)
fn ins_test(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // v_out_u = vec4<u32>(insertBits(ins_bits_u.value.x, ins_bits_u.newbits.x, ins_bits_u.offset, ins_bits_u.count), 0u, 0u, 0u);
    if ins_bits_u.test_width == 1u {
        v_out_u = vec4<u32>(insertBits(ins_bits_u.value.x, ins_bits_u.newbits.x, ins_bits_u.offset, ins_bits_u.count), 0u, 0u, 0u);
        v_out_i = vec4<i32>(insertBits(ins_bits_i.value.x, ins_bits_i.newbits.x, ins_bits_i.offset, ins_bits_i.count), 0, 0, 0);
    } else if ins_bits_u.test_width == 4u {
        v_out_u = insertBits(ins_bits_u.value, ins_bits_u.newbits, ins_bits_u.offset, ins_bits_u.count);
        v_out_i = insertBits(ins_bits_i.value, ins_bits_i.newbits, ins_bits_i.offset, ins_bits_i.count);
    }
}

// fn myinsertBits(e: u32, newbits: u32, offset: u32, count: u32) -> u32 {
//     // let mask = (0xFFFFFFFFu >> (32u - count)) << offset;
//     // let w = 32u;
//     // let o = min(offset, w);
//     // let c = min(count, w - o);
//     // if c == 0u {
//     //     return e;
//     // }
//     //     (('ior',
//     //  ('iand', 'base', ('inot', ('ishl', ('isub', ('ishl', 1, 'bits'), 1), 'offset'))),
//     //  ('iand', ('ishl', 'insert', 'offset'), ('ishl', ('isub', ('ishl', 1, 'bits'), 1), 'offset'))))),

//     let temp = ((u32(e & (~(((1u << count) - 1u) << offset))) | ((newbits << offset) & u32(((1u << count) - 1u) << offset))));
//     return temp;

//     // let mask = (0xFFFFFFFFu >> (32u - count)) << offset;
//     // return ((newbits << offset) & mask) | (e & ~mask);
// }