pub use packing::*;
pub use unpacking::*;

pub mod packing {
    // pack2x16float
    pub const PACK2X16FLOAT: [([f32; 2], u32); 9] = [
        ([0., 0.], 0x00000000),
        ([1., 0.], 0x00003c00),
        ([1., 1.], 0x3c003c00),
        ([-1., -1.], 0xbc00bc00),
        ([10., 1.], 0x3c004900),
        ([-10., 1.], 0x3c00c900),
        // f32 normal, but not f16 precise
        ([1.00000011920928955078125, 1.], 0x3c003c00),
        // f32 subnormals
        ([f32::MIN_POSITIVE * 0.1, 1.], 0x3c000000), // [0x3c000000, 0x3c008000, 0x3c000001]
        ([f32::MIN_POSITIVE * -0.1, 1.], 0x3c008000), // [0x3c008001, 0x3c000000, 0x3c008000]
    ];
    // Unimplemented tests
    // ([(f16::MIN_POSITIVE * 0.1) as f32, 1.], 0x00000000),   // [0x3c0003ff, 0x3c000000, 0x3c008000]
    // ([(f16::MIN_POSITIVE * -0.1) as f32, 1.], 0x00000000),  // [0x03c0083ff, 0x3c000000, 0x3c008000]
    // ([f32::MAX, 1.], 0x00000000),   // TODO: Undefined
    // ([f32::MIN, 1.], 0x00000000),   // TODO: Undefined
    // ([1., f32::MAX], 0x00000000),   // TODO: Undefined
    // ([1., f32::MIN], 0x00000000),   // TODO: Undefined

    pub const PACK2X16SNORM: [([f32; 2], u32); 15] = [
        ([0., 0.], 0x00000000),
        ([1., 0.], 0x00007fff),
        ([0., 1.], 0x7fff0000),
        ([1., 1.], 0x7fff7fff),
        ([-1., -1.], 0x80018001),
        ([10., 10.], 0x7fff7fff),
        ([-10., -10.], 0x80018001),
        ([0.1, 0.1], 0x0ccd0ccd),
        ([-0.1, -0.1], 0xf333f333),
        ([0.5, 0.5], 0x40004000),
        ([-0.5, -0.5], 0xc001c001),
        ([0.1, 0.5], 0x40000ccd),
        ([-0.1, -0.5], 0xc001f333),
        // subnormals
        ([f32::MIN_POSITIVE * 0.1, 1.], 0x7fff0000),
        ([f32::MIN_POSITIVE * -0.1, 1.], 0x7fff0000),
    ];

    pub const PACK2X16UNORM: [([f32; 2], u32); 10] = [
        ([0., 0.], 0x00000000),
        ([1., 0.], 0x0000ffff),
        ([0., 1.], 0xffff0000),
        ([1., 1.], 0xffffffff),
        ([-1., -1.], 0x00000000),
        ([0.1, 0.1], 0x199a199a),
        ([0.5, 0.5], 0x80008000),
        ([0.1, 0.5], 0x8000199a),
        ([10., 10.], 0xffffffff),
        // subnormals
        ([f32::MIN_POSITIVE * 0.1, 1.], 0xffff0000),
    ];

    pub const PACK4X8SNORM: [([f32; 4], u32); 23] = [
        // Normals
        ([0., 0., 0., 0.], 0x00000000),
        ([1., 0., 0., 0.], 0x0000007f),
        ([0., 1., 0., 0.], 0x00007f00),
        ([0., 0., 1., 0.], 0x007f0000),
        ([0., 0., 0., 1.], 0x7f000000),
        ([1., 1., 1., 1.], 0x7f7f7f7f),
        ([10., 10., 10., 10.], 0x7f7f7f7f),
        ([-1., 0., 0., 0.], 0x00000081),
        ([0., -1., 0., 0.], 0x00008100),
        ([0., 0., -1., 0.], 0x00810000),
        ([0., 0., 0., -1.], 0x81000000),
        ([-1., -1., -1., -1.], 0x81818181),
        ([-10., -10., -10., -10.], 0x81818181),
        ([0.1, 0.1, 0.1, 0.1], 0x0d0d0d0d),
        ([-0.1, -0.1, -0.1, -0.1], 0xf3f3f3f3),
        ([0.1, -0.1, 0.1, -0.1], 0xf30df30d),
        ([0.5, 0.5, 0.5, 0.5], 0x40404040),
        ([-0.5, -0.5, -0.5, -0.5], 0xc1c1c1c1),
        ([-0.5, 0.5, -0.5, 0.5], 0x40c140c1),
        ([0.1, 0.5, 0.1, 0.5], 0x400d400d),
        ([-0.1, -0.5, -0.1, -0.5], 0xc1f3c1f3),
        // Subnormals
        ([f32::MIN_POSITIVE * 0.1, 1., 1., 1.], 0x7f7f7f00),
        ([f32::MIN_POSITIVE * -0.1, 1., 1., 1.], 0x7f7f7f00),
    ];

    pub const PACK4X8UNORM: [([f32; 4], u32); 13] = [
        ([0., 0., 0., 0.], 0x00000000),
        ([1., 0., 0., 0.], 0x000000ff),
        ([0., 1., 0., 0.], 0x0000ff00),
        ([0., 0., 1., 0.], 0x00ff0000),
        ([0., 0., 0., 1.], 0xff000000),
        ([1., 1., 1., 1.], 0xffffffff),
        ([10., 10., 10., 10.], 0xffffffff),
        ([-1., -1., -1., -1.], 0x00000000),
        ([-10., -10., -10., -10.], 0x00000000),
        ([0.1, 0.1, 0.1, 0.1], 0x1a1a1a1a),
        ([0.5, 0.5, 0.5, 0.5], 0x80808080),
        ([0.1, 0.5, 0.1, 0.5], 0x801a801a),
        // subnormals
        ([f32::MIN_POSITIVE * 0.1, 1., 1., 1.], 0xffffff00),
    ];
}

pub mod unpacking {
    pub struct IntervalBounds {
        pub min: f32,
        pub max: f32,
    }

    // Magic numbers from the spec
    // https://github.com/gpuweb/cts/blob/main/src/unittests/floating_point.spec.ts

    pub const ZERO_BOUNDS: IntervalBounds = IntervalBounds {
        min: f32::MIN_POSITIVE * -1.,
        max: f32::MIN_POSITIVE,
    };

    pub const ONE_BOUNDS_SNORM: IntervalBounds = IntervalBounds {
        min: 0.999999821186065673828125,
        max: 1.0000002384185791015625,
    };

    pub const ONE_BOUNDS_UNORM: IntervalBounds = IntervalBounds {
        min: 0.9999998509883880615234375,
        max: 1.0000001490116119384765625,
    };

    pub const NEG_ONE_BOUNDS_SNORM: IntervalBounds = IntervalBounds {
        min: -1.0 - f32::EPSILON,
        max: -0.999999821186065673828125,
    };

    pub const HALF_BOUNDS_2X16_SNORM: IntervalBounds = IntervalBounds {
        min: 0.500015079975128173828125,
        max: 0.5000154972076416015625,
    };

    pub const NEG_HALF_BOUNDS_2X16_SNORM: IntervalBounds = IntervalBounds {
        min: -0.4999848306179046630859375,
        max: -0.49998462200164794921875,
    };

    pub const HALF_BOUNDS_2X16_UNORM: IntervalBounds = IntervalBounds {
        min: 0.5000074803829193115234375,
        max: 0.5000078380107879638671875,
    };

    pub const HALF_BOUNDS_4X8_SNORM: IntervalBounds = IntervalBounds {
        min: 0.503936827182769775390625,
        max: 0.503937244415283203125,
    };

    pub const NEG_HALF_BOUNDS_4X8_SNORM: IntervalBounds = IntervalBounds {
        min: -0.4960630834102630615234375,
        max: -0.49606287479400634765625,
    };

    pub const HALF_BOUNDS_4X8_UNORM: IntervalBounds = IntervalBounds {
        min: 0.5019606053829193115234375,
        max: 0.5019609630107879638671875,
    };

    pub const UNPACK_2X16_SNORM: [(u32, [IntervalBounds; 2]); 9] = [
        (0x00000000, [ZERO_BOUNDS, ZERO_BOUNDS]),
        (0x00007fff, [ONE_BOUNDS_SNORM, ZERO_BOUNDS]),
        (0x7fff0000, [ZERO_BOUNDS, ONE_BOUNDS_SNORM]),
        (0x7fff7fff, [ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM]),
        (0x80018001, [NEG_ONE_BOUNDS_SNORM, NEG_ONE_BOUNDS_SNORM]),
        (0x40004000, [HALF_BOUNDS_2X16_SNORM, HALF_BOUNDS_2X16_SNORM]),
        (
            0xc001c001,
            [NEG_HALF_BOUNDS_2X16_SNORM, NEG_HALF_BOUNDS_2X16_SNORM],
        ),
        (0x0000c001, [NEG_HALF_BOUNDS_2X16_SNORM, ZERO_BOUNDS]), // Error here
        (0xc0010000, [ZERO_BOUNDS, NEG_HALF_BOUNDS_2X16_SNORM]),
    ];

    pub const UNPACK_2X16_UNORM: [(u32, [IntervalBounds; 2]); 5] = [
        (0x00000000, [ZERO_BOUNDS, ZERO_BOUNDS]),
        (0x0000ffff, [ONE_BOUNDS_UNORM, ZERO_BOUNDS]),
        (0xffff0000, [ZERO_BOUNDS, ONE_BOUNDS_UNORM]),
        (0xffffffff, [ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM]),
        (0x80008000, [HALF_BOUNDS_2X16_UNORM, HALF_BOUNDS_2X16_UNORM]),
    ];

    #[rustfmt::skip]
    pub const UNPACK_4X8_SNORM: [(u32, [IntervalBounds; 4]); 13] = [
        (0x00000000, [ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS] ),
        (0x0000007f, [ONE_BOUNDS_SNORM, ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS] ),
        (0x00007f00, [ZERO_BOUNDS, ONE_BOUNDS_SNORM, ZERO_BOUNDS, ZERO_BOUNDS] ),
        (0x007f0000, [ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_SNORM, ZERO_BOUNDS] ),
        (0x7f000000, [ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_SNORM] ),
        (0x00007f7f, [ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM, ZERO_BOUNDS, ZERO_BOUNDS] ),
        (0x7f7f0000, [ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM] ),
        (0x7f007f00, [ZERO_BOUNDS, ONE_BOUNDS_SNORM, ZERO_BOUNDS, ONE_BOUNDS_SNORM] ),
        (0x007f007f, [ONE_BOUNDS_SNORM, ZERO_BOUNDS, ONE_BOUNDS_SNORM, ZERO_BOUNDS] ),
        (0x7f7f7f7f, [ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM, ONE_BOUNDS_SNORM] ),
        (0x81818181, [NEG_ONE_BOUNDS_SNORM, NEG_ONE_BOUNDS_SNORM, NEG_ONE_BOUNDS_SNORM, NEG_ONE_BOUNDS_SNORM] ),
        (0x40404040, [HALF_BOUNDS_4X8_SNORM, HALF_BOUNDS_4X8_SNORM, HALF_BOUNDS_4X8_SNORM, HALF_BOUNDS_4X8_SNORM] ),
        (0xc1c1c1c1, [NEG_HALF_BOUNDS_4X8_SNORM, NEG_HALF_BOUNDS_4X8_SNORM, NEG_HALF_BOUNDS_4X8_SNORM, NEG_HALF_BOUNDS_4X8_SNORM] ),
        ];

    #[rustfmt::skip]
    pub const UNPACK_4X8_UNORM: [(u32, [IntervalBounds; 4]); 11] = [
        (0x00000000, [ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS]),
        (0x000000ff, [ONE_BOUNDS_UNORM, ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS]),
        (0x0000ff00, [ZERO_BOUNDS, ONE_BOUNDS_UNORM, ZERO_BOUNDS, ZERO_BOUNDS]),
        (0x00ff0000, [ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_UNORM, ZERO_BOUNDS]),
        (0xff000000, [ZERO_BOUNDS, ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_UNORM]),
        (0x0000ffff, [ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM, ZERO_BOUNDS, ZERO_BOUNDS]),
        (0xffff0000, [ZERO_BOUNDS, ZERO_BOUNDS, ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM]),
        (0xff00ff00, [ZERO_BOUNDS, ONE_BOUNDS_UNORM, ZERO_BOUNDS, ONE_BOUNDS_UNORM]),
        (0x00ff00ff, [ONE_BOUNDS_UNORM, ZERO_BOUNDS, ONE_BOUNDS_UNORM, ZERO_BOUNDS]),
        (0xffffffff, [ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM, ONE_BOUNDS_UNORM]),
        (0x80808080, [HALF_BOUNDS_4X8_UNORM, HALF_BOUNDS_4X8_UNORM, HALF_BOUNDS_4X8_UNORM, HALF_BOUNDS_4X8_UNORM]),
    ];

    pub const UNPACK_2X16_FLOAT: [(u32, [f32; 2]); 10] = [
        // f16 normals
        (0x00000000, [0.0, 0.0]),
        (0x80000000, [0.0, 0.0]),
        (0x00008000, [0.0, 0.0]),
        (0x80008000, [0.0, 0.0]),
        (0x00003c00, [1.0, 0.0]),
        (0x3c000000, [0.0, 1.0]),
        (0x3c003c00, [1.0, 1.0]),
        (0xbc00bc00, [-1.0, -1.0]),
        (0x49004900, [10.0, 10.0]),
        (0xc900c900, [-10.0, -10.0]),
        // // f16 subnormals
        // (0x000003ff, [[0, kValue.f16.subnormal.positive.max], 0] },
        // (0x000083ff, [[kValue.f16.subnormal.negative.min, 0], 0] },
        // // f16 out of bounds
        // (0x7c000000, [kAnyBounds, kAnyBounds] },
        // (0xffff0000, [kAnyBounds, kAnyBounds] },
    ];
}

pub mod insert {
    // https://github.com/gpuweb/cts/blob/main/src/webgpu/shader/execution/expression/call/builtin/insertBits.spec.ts
    use super::helpers::*;

    const PATTERN: Things = Things::Pattern([
        0b10001001010100100010010100100010,
        0b11001110001100111000110011100011,
        0b10101010101010101010101010101010,
        0b01010101010101010101010101010101,
    ]);

    // TODO : Overflow tests
    pub const BASIC_CASES: [([Things; 4], Things); 16] = [
        ([ALL_0, ALL_0, Things::Single(0), Things::Single(32)], ALL_0),
        ([ALL_0, ALL_0, Things::Single(1), Things::Single(10)], ALL_0),
        ([ALL_0, ALL_0, Things::Single(2), Things::Single(5)], ALL_0),
        ([ALL_0, ALL_0, Things::Single(0), Things::Single(1)], ALL_0),
        ([ALL_0, ALL_0, Things::Single(31), Things::Single(1)], ALL_0),
        ([ALL_0, ALL_1, Things::Single(0), Things::Single(32)], ALL_1),
        ([ALL_1, ALL_0, Things::Single(0), Things::Single(32)], ALL_0),
        ([ALL_0, ALL_1, Things::Single(0), Things::Single(1)], LOW_1),
        ([ALL_1, ALL_0, Things::Single(0), Things::Single(1)], LOW_0),
        ([ALL_0, ALL_1, Things::Single(0), Things::Single(0)], ALL_0),
        (
            [ALL_0, ALL_1, Things::Single(31), Things::Single(1)],
            HIGH_1,
        ),
        (
            [ALL_1, ALL_0, Things::Single(31), Things::Single(1)],
            HIGH_0,
        ),
        (
            [ALL_0, ALL_1, Things::Single(1), Things::Single(10)],
            Things::Single(0b00000000000000000000011111111110),
        ),
        (
            [ALL_1, ALL_0, Things::Single(1), Things::Single(10)],
            Things::Single(0b11111111111111111111100000000001),
        ),
        (
            [ALL_0, ALL_1, Things::Single(2), Things::Single(5)],
            Things::Single(0b00000000000000000000000001111100),
        ),
        (
            [ALL_1, ALL_0, Things::Single(2), Things::Single(5)],
            Things::Single(0b11111111111111111111111110000011),
        ),
    ];

    // Patterns
    pub const PATTERN_CASES: [([Things; 4], Things); 34] = [
        (
            [ALL_0, PATTERN, Things::Single(0), Things::Single(32)],
            PATTERN,
        ),
        (
            [ALL_1, PATTERN, Things::Single(0), Things::Single(32)],
            PATTERN,
        ),
        (
            [ALL_0, PATTERN, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b00010010101001000100101001000100,
                0b10011100011001110001100111000110,
                0b01010101010101010101010101010100,
                0b10101010101010101010101010101010,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b00010010101001000100101001000101,
                0b10011100011001110001100111000111,
                0b01010101010101010101010101010101,
                0b10101010101010101010101010101011,
            ]),
        ),
        (
            [ALL_0, PATTERN, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b10001001010010001000000000000000,
                0b11100011001110001100000000000000,
                0b10101010101010101000000000000000,
                0b01010101010101010100000000000000,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b10001001010010001011111111111111,
                0b11100011001110001111111111111111,
                0b10101010101010101011111111111111,
                0b01010101010101010111111111111111,
            ]),
        ),
        (
            [ALL_0, PATTERN, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b00000000000010001000000000000000,
                0b00000000000110001100000000000000,
                0b00000000000010101000000000000000,
                0b00000000000101010100000000000000,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b11111111111010001011111111111111,
                0b11111111111110001111111111111111,
                0b11111111111010101011111111111111,
                0b11111111111101010111111111111111,
            ]),
        ),
        (
            [ALL_0, PATTERN, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b00000000000000001000000000000000,
                0b00000000000000001100000000000000,
                0b00000000000000101000000000000000,
                0b00000000000000010100000000000000,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b11111111111111001011111111111111,
                0b11111111111111001111111111111111,
                0b11111111111111101011111111111111,
                0b11111111111111010111111111111111,
            ]),
        ),
        (
            [ALL_0, PATTERN, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b00000000000000001000000000000000,
                0b00000000000000001100000000000000,
                0b00000000000000001000000000000000,
                0b00000000000000010100000000000000,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b11111111111111101011111111111111,
                0b11111111111111101111111111111111,
                0b11111111111111101011111111111111,
                0b11111111111111110111111111111111,
            ]),
        ),
        (
            [ALL_0, PATTERN, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b00000000000010000000000000000000,
                0b00000000000011000000000000000000,
                0b00000000000010000000000000000000,
                0b00000000000101000000000000000000,
            ]),
        ),
        (
            [ALL_1, PATTERN, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b11111111111010111111111111111111,
                0b11111111111011111111111111111111,
                0b11111111111010111111111111111111,
                0b11111111111101111111111111111111,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000001,
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000001,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b11111111111111111111111111111110,
                0b11111111111111111111111111111111,
                0b11111111111111111111111111111110,
                0b11111111111111111111111111111111,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b00000000000000000010010100100010,
                0b00000000000000000000110011100011,
                0b00000000000000000010101010101010,
                0b00000000000000000001010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b11111111111111111110010100100010,
                0b11111111111111111100110011100011,
                0b11111111111111111110101010101010,
                0b11111111111111111101010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b10001001010000000010010100100010,
                0b11001110001000000000110011100011,
                0b10101010101000000010101010101010,
                0b01010101010000000001010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b10001001010111111110010100100010,
                0b11001110001111111100110011100011,
                0b10101010101111111110101010101010,
                0b01010101010111111101010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b10001001010100000010010100100010,
                0b11001110001100000000110011100011,
                0b10101010101010000010101010101010,
                0b01010101010101000001010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b10001001010100111110010100100010,
                0b11001110001100111100110011100011,
                0b10101010101010111110101010101010,
                0b01010101010101111101010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b10001001010100100010010100100010,
                0b11001110001100100000110011100011,
                0b10101010101010100010101010101010,
                0b01010101010101000001010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b10001001010100111110010100100010,
                0b11001110001100111100110011100011,
                0b10101010101010111110101010101010,
                0b01010101010101011101010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_0, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b10001001010000100010010100100010,
                0b11001110001000111000110011100011,
                0b10101010101000101010101010101010,
                0b01010101010000010101010101010101,
            ]),
        ),
        (
            [PATTERN, ALL_1, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b10001001010111100010010100100010,
                0b11001110001111111000110011100011,
                0b10101010101111101010101010101010,
                0b01010101010111010101010101010101,
            ]),
        ),
        (
            [PATTERN, PATTERN, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b10001001010010100010010100100010,
                0b11001110001011111000110011100011,
                0b10101010101010101010101010101010,
                0b01010101010101010101010101010101,
            ]),
        ),
        (
            [PATTERN, PATTERN, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b10001001010010001010010100100010,
                0b11001110001110001100110011100011,
                0b10101010101010101010101010101010,
                0b01010101010101010101010101010101,
            ]),
        ),
        // Zero count
        (
            [PATTERN, ALL_1, Things::Single(0), Things::Single(0)],
            PATTERN,
        ),
        (
            [PATTERN, ALL_1, Things::Single(1), Things::Single(0)],
            PATTERN,
        ),
        (
            [PATTERN, ALL_1, Things::Single(2), Things::Single(0)],
            PATTERN,
        ),
        (
            [PATTERN, ALL_1, Things::Single(31), Things::Single(0)],
            PATTERN,
        ),
        (
            [PATTERN, ALL_1, Things::Single(32), Things::Single(0)],
            PATTERN,
        ),
        (
            [PATTERN, ALL_1, Things::Single(0), Things::Single(0)],
            PATTERN,
        ),
    ];
}

pub mod extract {
    use super::helpers::*;

    pub const PATTERN: Things = Things::Pattern([
        0b00000000000111011100000000000000,
        0b11111111111000000011111111111111,
        0b00000000010101010101000000000000,
        0b00000000001010101010100000000000,
    ]);

    pub const BASIC_CASES_UNSIGNED: [([Things; 3], Things); 14] = [
        ([ALL_0, Things::Single(0), Things::Single(32)], ALL_0),
        ([ALL_0, Things::Single(1), Things::Single(10)], ALL_0),
        ([ALL_0, Things::Single(2), Things::Single(5)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(1)], ALL_0),
        ([ALL_0, Things::Single(31), Things::Single(1)], ALL_0),
        ([ALL_1, Things::Single(0), Things::Single(32)], ALL_1),
        (
            [ALL_1, Things::Single(1), Things::Single(10)],
            Things::Single(0b00000000000000000000001111111111),
        ),
        (
            [ALL_1, Things::Single(2), Things::Single(5)],
            Things::Single(0b00000000000000000000000000011111),
        ),
        ([ALL_1, Things::Single(0), Things::Single(1)], LOW_1),
        ([ALL_1, Things::Single(31), Things::Single(1)], LOW_1),
        // Zero count
        ([ALL_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(0)], ALL_0),
        ([LOW_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([HIGH_1, Things::Single(31), Things::Single(0)], ALL_0),
    ];

    pub const PATTERN_CASES_UNSIGNED: [([Things; 3], Things); 14] = [
        // Patterns
        ([PATTERN, Things::Single(0), Things::Single(32)], PATTERN),
        (
            [PATTERN, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b00000000000011101110000000000000,
                0b01111111111100000001111111111111,
                0b00000000001010101010100000000000,
                0b00000000000101010101010000000000,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b00000000000000000000000001110111,
                0b00000000000000111111111110000000,
                0b00000000000000000000000101010101,
                0b00000000000000000000000010101010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b00000000000000000000000001110111,
                0b00000000000000000000000000000000,
                0b00000000000000000000000001010101,
                0b00000000000000000000000000101010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b00000000000000000000000000000111,
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000101,
                0b00000000000000000000000000001010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b00000000000000000000000000000111,
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000101,
                0b00000000000000000000000000000010,
            ]),
        ),
        (
            [PATTERN, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b00000000000000000000000000000111,
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000101,
                0b00000000000000000000000000000010,
            ]),
        ),
        ([LOW_1, Things::Single(0), Things::Single(1)], LOW_1),
        ([HIGH_1, Things::Single(31), Things::Single(1)], LOW_1),
        ([ALL_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(0)], ALL_0),
        ([LOW_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([HIGH_1, Things::Single(31), Things::Single(0)], ALL_0),
        ([PATTERN, Things::Single(0), Things::Single(0)], ALL_0),
    ];

    pub const BASIC_CASES_SIGNED: [([Things; 3], Things); 16] = [
        ([ALL_0, Things::Single(0), Things::Single(32)], ALL_0),
        ([ALL_0, Things::Single(1), Things::Single(10)], ALL_0),
        ([ALL_0, Things::Single(2), Things::Single(5)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(1)], ALL_0),
        ([ALL_0, Things::Single(31), Things::Single(1)], ALL_0),
        ([ALL_1, Things::Single(0), Things::Single(32)], ALL_1),
        ([ALL_1, Things::Single(1), Things::Single(10)], ALL_1),
        ([ALL_1, Things::Single(2), Things::Single(5)], ALL_1),
        ([ALL_1, Things::Single(0), Things::Single(1)], ALL_1),
        ([ALL_1, Things::Single(31), Things::Single(1)], ALL_1),
        ([LOW_1, Things::Single(0), Things::Single(1)], ALL_1),
        ([HIGH_1, Things::Single(31), Things::Single(1)], ALL_1),
        ([ALL_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(0)], ALL_0),
        ([LOW_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([HIGH_1, Things::Single(31), Things::Single(0)], ALL_0),
    ];

    pub const PATTERN_CASES_SIGNED: [([Things; 3], Things); 14] = [
        ([PATTERN, Things::Single(0), Things::Single(32)], PATTERN),
        (
            [PATTERN, Things::Single(1), Things::Single(31)],
            Things::Pattern([
                0b00000000000011101110000000000000,
                0b11111111111100000001111111111111,
                0b00000000001010101010100000000000,
                0b00000000000101010101010000000000,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(18)],
            Things::Pattern([
                0b00000000000000000000000001110111,
                0b11111111111111111111111110000000,
                0b00000000000000000000000101010101,
                0b00000000000000000000000010101010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(7)],
            Things::Pattern([
                0b11111111111111111111111111110111,
                0b00000000000000000000000000000000,
                0b11111111111111111111111111010101,
                0b00000000000000000000000000101010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(4)],
            Things::Pattern([
                0b00000000000000000000000000000111,
                0b00000000000000000000000000000000,
                0b00000000000000000000000000000101,
                0b11111111111111111111111111111010,
            ]),
        ),
        (
            [PATTERN, Things::Single(14), Things::Single(3)],
            Things::Pattern([
                0b11111111111111111111111111111111,
                0b00000000000000000000000000000000,
                0b11111111111111111111111111111101,
                0b00000000000000000000000000000010,
            ]),
        ),
        (
            [PATTERN, Things::Single(18), Things::Single(3)],
            Things::Pattern([
                0b11111111111111111111111111111111,
                0b00000000000000000000000000000000,
                0b11111111111111111111111111111101,
                0b00000000000000000000000000000010,
            ]),
        ),
        ([LOW_1, Things::Single(0), Things::Single(1)], ALL_1),
        ([HIGH_1, Things::Single(31), Things::Single(1)], ALL_1),
        // Zero count
        ([ALL_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([ALL_0, Things::Single(0), Things::Single(0)], ALL_0),
        ([LOW_1, Things::Single(0), Things::Single(0)], ALL_0),
        ([HIGH_1, Things::Single(31), Things::Single(0)], ALL_0),
        ([PATTERN, Things::Single(0), Things::Single(0)], ALL_0),
    ];
}

pub mod helpers {
    #[derive(Debug, Clone, Copy)]
    pub enum Things {
        Single(u32),
        Pattern([u32; 4]),
    }

    impl Things {
        pub fn into_pat(&self) -> Things {
            match self {
                Self::Single(v) => Things::Pattern([*v; 4]),
                Self::Pattern(_) => *self,
            }
        }
    }

    impl Into<u32> for Things {
        fn into(self: Things) -> u32 {
            match self {
                Things::Single(v) => v,
                Things::Pattern(_) => panic!("Cannot convert PATTERN to single value"),
            }
        }
    }

    impl PartialEq for Things {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Single(l0), Self::Single(r0)) => l0 == r0,
                (Self::Pattern(l0), Self::Pattern(r0)) => l0 == r0,
                (Self::Single(l0), Self::Pattern(r0)) => [*l0, *l0, *l0, *l0] == *r0,
                (Self::Pattern(l0), Self::Single(r0)) => *l0 == [*r0, *r0, *r0, *r0],
                _ => false,
            }
        }
    }

    impl Eq for Things {}

    impl PartialEq<[u32; 4]> for Things {
        fn eq(&self, other: &[u32; 4]) -> bool {
            match self {
                Self::Single(l0) => [*l0; 4] == *other,
                Self::Pattern(l0) => l0 == other,
            }
        }
    }

    pub const ALL_1: Things = Things::Single(0b11111111111111111111111111111111);
    pub const ALL_0: Things = Things::Single(0b00000000000000000000000000000000);
    pub const LOW_1: Things = Things::Single(0b00000000000000000000000000000001);
    pub const LOW_0: Things = Things::Single(0b11111111111111111111111111111110);
    pub const HIGH_1: Things = Things::Single(0b10000000000000000000000000000000);
    pub const HIGH_0: Things = Things::Single(0b01111111111111111111111111111111);
}
