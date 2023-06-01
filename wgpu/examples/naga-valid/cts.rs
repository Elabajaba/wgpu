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
        (0x0000c001, [NEG_HALF_BOUNDS_2X16_SNORM, ZERO_BOUNDS]),    // Error here
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
        // { input: 0x000003ff, expected: [[0, kValue.f16.subnormal.positive.max], 0] },
        // { input: 0x000083ff, expected: [[kValue.f16.subnormal.negative.min, 0], 0] },
        // // f16 out of bounds
        // { input: 0x7c000000, expected: [kAnyBounds, kAnyBounds] },
        // { input: 0xffff0000, expected: [kAnyBounds, kAnyBounds] },
    ];
}
