use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use wgpu::util::DeviceExt;

use crate::cts::helpers::Things;

mod cts;

async fn run() {
    // Instantiates instance of WebGPU
    let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let dx12_shader_compiler = wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler,
    });

    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    println!("{:?}", adapter.get_info());

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();
    println!("PACK4X8UNORM---------------------------------------------");
    for (nums, expected) in cts::PACK4X8UNORM {
        let res = execute_gpu_pack(&nums, &device, &queue, "p4x8u")
            .await
            .unwrap();
        if res != expected {
            println!("ERROR: nums: {:?}", nums);
            println!("res: {:#x}, expected: {:#x}", res, expected);
        }
    }
    println!("PACK4X8SNORM---------------------------------------------");
    for (nums, expected) in cts::PACK4X8SNORM {
        let res = execute_gpu_pack(&nums, &device, &queue, "p4x8s")
            .await
            .unwrap();
        if res != expected {
            println!("ERROR: nums: {:?}", nums);
            println!("res: {:#x}, expected: {:#x}", res, expected);
        }
    }
    println!("PACK2X16UNORM---------------------------------------------");
    for (nums, expected) in cts::PACK2X16UNORM {
        let res = execute_gpu_pack(&[nums[0], nums[1], 0., 0.], &device, &queue, "p2x16u")
            .await
            .unwrap();
        if res != expected {
            println!("ERROR: nums: {:?}", nums);
            println!("res: {:#x}, expected: {:#x}", res, expected);
        }
    }
    println!("PACK2X16SNORM---------------------------------------------");
    for (nums, expected) in cts::PACK2X16SNORM {
        let res = execute_gpu_pack(&[nums[0], nums[1], 0., 0.], &device, &queue, "p2x16s")
            .await
            .unwrap();
        if res != expected {
            println!("ERROR: nums: {:?}", nums);
            println!("res: {:#x}, expected: {:#x}", res, expected);
        }
    }
    println!("PACK2X16FLOAT---------------------------------------------");
    for (nums, expected) in cts::PACK2X16FLOAT {
        let res = execute_gpu_pack(&[nums[0], nums[1], 0., 0.], &device, &queue, "p2x16f")
            .await
            .unwrap();
        if res != expected {
            println!("ERROR: nums: {:?}", nums);
            println!("res: {:#x}, expected: {:#x}", res, expected);
        }
    }
    println!(">>>>>>>>>>>>>>>>>>>UNPACKING<<<<<<<<<<<<<<<<<<<<<<");
    println!("UNPACK4X8UNORM---------------------------------------------");
    for (nums, expected) in cts::UNPACK_4X8_UNORM {
        let res = execute_gpu_unpack(&[nums], &device, &queue, "unp4x8u")
            .await
            .unwrap();
        for (r, bound) in res.iter().zip(expected.iter()) {
            if *r <= bound.min || *r > bound.max {
                println!("ERROR: nums: {:?}", nums);
                println!("res: {}, expected: {}", r, bound.min);
            }
        }
    }
    println!("UNPACK4X8SNORM---------------------------------------------");
    for (nums, expected) in cts::UNPACK_4X8_SNORM {
        let res = execute_gpu_unpack(&[nums], &device, &queue, "unp4x8s")
            .await
            .unwrap();
        for (r, bound) in res.iter().zip(expected.iter()) {
            if *r <= bound.min || *r > bound.max {
                println!("ERROR: nums: {:?}", nums);
                println!("res: {}, expected: {}", r, bound.min);
            }
        }
    }
    println!("UNPACK2X16UNORM---------------------------------------------");
    for (nums, expected) in cts::UNPACK_2X16_UNORM {
        let res = execute_gpu_unpack(&[nums], &device, &queue, "unp2x16u")
            .await
            .unwrap();
        for (r, bound) in res.iter().zip(expected.iter()) {
            if *r <= bound.min || *r > bound.max {
                println!("ERROR: nums: {:?}", nums);
                println!("res: {}, expected: {}", r, bound.min);
            }
        }
    }
    println!("UNPACK2X16SNORM---------------------------------------------");
    for (nums, expected) in cts::UNPACK_2X16_SNORM {
        let res = execute_gpu_unpack(&[nums], &device, &queue, "unp2x16s")
            .await
            .unwrap();
        for (r, bound) in res.iter().zip(expected.iter()) {
            if *r <= bound.min || *r > bound.max {
                println!("ERROR: nums: {:?}", nums);
                println!("res: {}, expected: {}", r, bound.min);
            }
        }
    }
    println!("UNPACK2X16FLOAT--------------------------------------------");
    for (nums, expected) in cts::UNPACK_2X16_FLOAT {
        let res = execute_gpu_unpack(&[nums], &device, &queue, "unp2x16f")
            .await
            .unwrap();
        for (r, bound) in res.iter().zip(expected.iter()) {
            if r != bound {
                println!("ERROR: nums: {:?}", nums);
                println!("res: {}, expected: {}", r, bound);
            }
        }
    }
    println!(">>>>>>>>>>>>>>>>>>>INSERT_BITS<<<<<<<<<<<<<<<<<<<<<<");
    println!("INSERTBITS_SIMPLE--------------------------------------------");
    for (a, expected) in cts::insert::BASIC_CASES {
        let insbits = InsBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v, 0, 0, 0],
                Things::Pattern(_) => panic!("Patterns shouldn't be part of INSERTBITS_SIMPLE"),
            },
            newbits: match a[1] {
                Things::Single(v) => [v, 0, 0, 0],
                Things::Pattern(_) => panic!("Patterns shouldn't be part of INSERTBITS_SIMPLE"),
            },
            offset: a[2].into(),
            count: a[3].into(),
            width: 1,
            empty: 0,
        };
        let (res_u, res_i) = execute_gpu_insertbits(insbits, &device, &queue, "ins_test")
            .await
            .unwrap();
        let unsigned_expected = match expected {
            Things::Single(v) => v,
            Things::Pattern(_) => panic!("Patterns shouldn't be part of INSERTBITS_SIMPLE"),
        };
        let signed_expected: i32 = i32::from_ne_bytes(unsigned_expected.to_ne_bytes());
        if unsigned_expected != res_u[0] {
            println!("UNSIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res_u[0], unsigned_expected);
        }
        if signed_expected != res_i[0] {
            println!("SIGNED ERROR: a: {:?}", a);
            println!("res: {}, exp: {signed_expected}", res_i[0]);
            println!("res: {:#b},\nexp: {:#b}", res_i[0], signed_expected);
        }
    }

    println!("INSERTBITS_PATTERNS--------------------------------------------");
    for (a, expected) in cts::insert::PATTERN_CASES {
        let insbits = InsBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v; 4],
                Things::Pattern(p) => p,
            },
            newbits: match a[1] {
                Things::Single(v) => [v; 4],
                Things::Pattern(p) => p,
            },
            offset: a[2].into(),
            count: a[3].into(),
            width: 4,
            empty: 0,
        };
        let res = execute_gpu_insertbits(insbits, &device, &queue, "ins_test")
            .await
            .unwrap();
        let expec = match expected {
            Things::Single(v) => [v; 4],
            Things::Pattern(p) => p,
        };
        if expec != res.0 {
            println!("UNSIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[0], expec[0]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[1], expec[1]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[2], expec[2]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[3], expec[3]);
        }
        let expec_signed: [i32; 4] = expec
            .iter()
            .map(|&x| i32::from_ne_bytes(x.to_ne_bytes()))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        if expec_signed != res.1 {
            println!("SIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[0], expec_signed[0]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[1], expec_signed[1]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[2], expec_signed[2]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[3], expec_signed[3]);
        }
    }
    println!(">>>>>>>>>>>>>>>>>>>EXTRACT_BITS<<<<<<<<<<<<<<<<<<<<<<");
    println!("EXTRACTBITS_SIMPLE_UNSIGNED--------------------------------------------");
    for (a, expected) in cts::extract::BASIC_CASES_UNSIGNED {
        let extbits = ExtBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v, 0, 0, 0],
                Things::Pattern(_) => panic!("Patterns shouldn't be part of INSERTBITS_SIMPLE"),
            },
            offset: a[1].into(),
            count: a[2].into(),
            width: 1,
            empty: 0,
        };
        let (res_u, _) = execute_gpu_extractbits(extbits, &device, &queue, "ext_test")
            .await
            .unwrap();
        let unsigned_expected = match expected {
            Things::Single(v) => v,
            Things::Pattern(_) => panic!("Patterns shouldn't be part of EXTRACTBITS_SIMPLE_UNSIGNED"),
        };
        if unsigned_expected != res_u[0] {
            println!("UNSIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res_u[0], unsigned_expected);
        }
    }
    println!("EXTRACTBITS_SIMPLE_SIGNED--------------------------------------------");
    for (a, expected) in cts::extract::BASIC_CASES_SIGNED {
        let extbits = ExtBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v, 0, 0, 0],
                Things::Pattern(_) => panic!("Patterns shouldn't be part of INSERTBITS_SIMPLE"),
            },
            offset: a[1].into(),
            count: a[2].into(),
            width: 1,
            empty: 0,
        };
        let (_, res_i) = execute_gpu_extractbits(extbits, &device, &queue, "ext_test")
            .await
            .unwrap();
        let unsigned_expected = match expected {
            Things::Single(v) => v,
            Things::Pattern(_) => panic!("Patterns shouldn't be part of EXTRACTBITS_SIMPLE_SIGNED"),
        };
        let signed_expected: i32 = i32::from_ne_bytes(unsigned_expected.to_ne_bytes());
        if signed_expected != res_i[0] {
            println!("SIGNED ERROR: a: {:?}", a);
            println!("res: {}, exp: {signed_expected}", res_i[0]);
            println!("res: {:#b},\nexp: {:#b}", res_i[0], signed_expected);
        }
    }
    println!("EXTRACTBITS_PATTERNS_UNSIGNED--------------------------------------------");
    for (a, expected) in cts::extract::PATTERN_CASES_UNSIGNED {
        let extbits = ExtBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v; 4],
                Things::Pattern(p) => p,
            },
            offset: a[1].into(),
            count: a[2].into(),
            width: 4,
            empty: 0,
        };
        let res = execute_gpu_extractbits(extbits, &device, &queue, "ext_test")
            .await
            .unwrap();
        let expec = match expected {
            Things::Single(v) => [v; 4],
            Things::Pattern(p) => p,
        };
        if expec != res.0 {
            println!("UNSIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[0], expec[0]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[1], expec[1]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[2], expec[2]);
            println!("res: {:#034b},\nexp: {:#034b}", res.0[3], expec[3]);
        }
    }
    println!("EXTRACTBITS_PATTERNS_SIGNED--------------------------------------------");
    for (a, expected) in cts::extract::PATTERN_CASES_SIGNED {
        let extbits = ExtBitsUnsigned {
            e: match a[0] {
                Things::Single(v) => [v; 4],
                Things::Pattern(p) => p,
            },
            offset: a[1].into(),
            count: a[2].into(),
            width: 4,
            empty: 0,
        };
        let res = execute_gpu_extractbits(extbits, &device, &queue, "ext_test")
            .await
            .unwrap();
        let expec = match expected {
            Things::Single(v) => [v; 4],
            Things::Pattern(p) => p,
        };
        let expec_signed: [i32; 4] = expec
            .iter()
            .map(|&x| i32::from_ne_bytes(x.to_ne_bytes()))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        if expec_signed != res.1 {
            println!("SIGNED ERROR: a: {:?}", a);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[0], expec_signed[0]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[1], expec_signed[1]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[2], expec_signed[2]);
            println!("res: {:#034b},\nexp: {:#034b}", res.1[3], expec_signed[3]);
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct InsBitsUnsigned {
    e: [u32; 4],
    newbits: [u32; 4],
    offset: u32,
    count: u32,
    width: u32,
    empty: u32,
}

impl InsBitsUnsigned {
    fn cast_i32(&self) -> InsBitsSigned {
        let e: [i32; 4] = self
            .e
            .iter()
            .map(|&x| i32::from_ne_bytes(x.to_ne_bytes()))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        let newbits: [i32; 4] = self
            .newbits
            .iter()
            .map(|&x| i32::from_ne_bytes(x.to_ne_bytes()))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        unsafe {
            InsBitsSigned {
                e,
                newbits,
                offset: self.offset,
                count: self.count,
                width: self.width,
                empty: 0,
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct InsBitsSigned {
    e: [i32; 4],
    newbits: [i32; 4],
    offset: u32,
    count: u32,
    width: u32,
    empty: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ExtBitsUnsigned {
    e: [u32; 4],
    offset: u32,
    count: u32,
    width: u32,
    empty: u32,
}

impl ExtBitsUnsigned {
    fn cast_i32(&self) -> ExtBitsSigned {
        let e: [i32; 4] = self
            .e
            .iter()
            .map(|&x| i32::from_ne_bytes(x.to_ne_bytes()))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();
        unsafe {
            ExtBitsSigned {
                e,
                offset: self.offset,
                count: self.count,
                width: self.width,
                empty: 0,
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ExtBitsSigned {
    e: [i32; 4],
    offset: u32,
    count: u32,
    width: u32,
    empty: u32,
}

async fn execute_gpu_pack<T: Pod>(
    numbers: &[T],
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    entry_point: &str,
) -> Option<u32> {
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    // Gets the size in bytes of the buffer.
    let ret_buf = [0u32];
    let slice_size = ret_buf.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Instantiates buffer with data (`numbers`).
    // Usage allowing the buffer to be:
    //   A storage buffer (can be bound within a bind group and thus available to a shader).
    //   The destination of a copy.
    //   The source of a copy.
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(numbers),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let return_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&ret_buf),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader

    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (4 * std::mem::size_of::<u32>()) as u64,
                        ),
                    },
                    count: None,
                },
            ],
            label: None,
        });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point,
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: return_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage_buffer.as_entire_binding(),
            },
        ],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker(entry_point);
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(&return_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice = staging_buffer.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver.receive().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        staging_buffer.unmap(); // Unmaps buffer from memory
                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                //   delete myPointer;
                                //   myPointer = NULL;
                                // It effectively frees the memory

        // Returns data from buffer
        Some(result[0])
    } else {
        panic!("failed to run compute on gpu!")
    }
}

async fn execute_gpu_unpack(
    input: &[u32],
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    entry_point: &str,
) -> Option<[f32; 4]> {
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader2.wgsl"))),
    });

    // Gets the size in bytes of the buffer.
    let ret_buf = [0.0f32; 4];
    let slice_size = ret_buf.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Instantiates buffer with data (`numbers`).
    // Usage allowing the buffer to be:
    //   A storage buffer (can be bound within a bind group and thus available to a shader).
    //   The destination of a copy.
    //   The source of a copy.
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(input),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let return_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&ret_buf),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader
    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (input.len() * std::mem::size_of::<u32>()) as u64,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
            ],
            label: None,
        });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point,
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: return_buffer.as_entire_binding(),
            },
        ],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker(entry_point);
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(&return_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice = staging_buffer.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver.receive().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        staging_buffer.unmap(); // Unmaps buffer from memory
                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                //   delete myPointer;
                                //   myPointer = NULL;
                                // It effectively frees the memory

        // Returns data from buffer
        Some(result[0])
    } else {
        panic!("failed to run compute on gpu!")
    }
}

async fn execute_gpu_insertbits(
    input: InsBitsUnsigned,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    entry_point: &str,
) -> Option<([u32; 4], [i32; 4])> {
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader3.wgsl"))),
    });

    // Gets the size in bytes of the buffer.
    let ret_buf1 = [0u32; 4];
    let ret_buf2 = [0i32; 4];
    let slice_size = ret_buf1.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer_unsigned = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let staging_buffer_signed = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let ins_bits_buffer_unsigned = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("insertbits buffer unsigned"),
        contents: bytemuck::bytes_of(&input),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let ins_bits_buffer_signed = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("insertbits buffer signed"),
        contents: bytemuck::bytes_of(&input.cast_i32()),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let return_buffer_unsigned = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("return Buffer unsigned"),
        contents: bytemuck::cast_slice(&ret_buf1),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let return_buffer_signed = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("return Buffer signed"),
        contents: bytemuck::cast_slice(&ret_buf2),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader
    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (std::mem::size_of::<InsBitsUnsigned>()) as u64,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (std::mem::size_of::<InsBitsSigned>()) as u64,
                        ),
                    },
                    count: None,
                },
            ],
            label: None,
        });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point,
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: return_buffer_unsigned.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: return_buffer_signed.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: ins_bits_buffer_unsigned.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: ins_bits_buffer_signed.as_entire_binding(),
            },
        ],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker(entry_point);
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(
        &return_buffer_unsigned,
        0,
        &staging_buffer_unsigned,
        0,
        size,
    );
    encoder.copy_buffer_to_buffer(&return_buffer_signed, 0, &staging_buffer_signed, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice_unsigned = staging_buffer_unsigned.slice(..);
    let buffer_slice_signed = staging_buffer_signed.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender1, receiver1) = futures_intrusive::channel::shared::oneshot_channel();
    let (sender2, receiver2) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice_unsigned.map_async(wgpu::MapMode::Read, move |v| sender1.send(v).unwrap());
    buffer_slice_signed.map_async(wgpu::MapMode::Read, move |v| sender2.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver1.receive().await {
        if let Some(Ok(())) = receiver2.receive().await {
            // Gets contents of buffer
            let data1 = buffer_slice_unsigned.get_mapped_range();
            let data2 = buffer_slice_signed.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let a: Vec<[u32; 4]> = bytemuck::cast_slice(&data1).to_vec();
            let b: Vec<[i32; 4]> = bytemuck::cast_slice(&data2).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data1);
            staging_buffer_unsigned.unmap(); // Unmaps buffer from memory
                                             // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                             //   delete myPointer;
                                             //   myPointer = NULL;
                                             // It effectively frees the memory

            // Returns data from buffer
            // Some(result[0])
            return Some((a[0], b[0]));
        }
        todo!("failed to receive from receiver2")
    } else {
        panic!("failed to run compute on gpu!")
    }
}

async fn execute_gpu_extractbits(
    input: ExtBitsUnsigned,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    entry_point: &str,
) -> Option<([u32; 4], [i32; 4])> {
    // Loads the shader from WGSL
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader4.wgsl"))),
    });

    // Gets the size in bytes of the buffer.
    let ret_buf1 = [0u32; 4];
    let ret_buf2 = [0i32; 4];
    let slice_size = ret_buf1.len() * std::mem::size_of::<u32>();
    let size = slice_size as wgpu::BufferAddress;

    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer_unsigned = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let staging_buffer_signed = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let ext_bits_buffer_unsigned = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("extractbits buffer unsigned"),
        contents: bytemuck::bytes_of(&input),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let ext_bits_buffer_signed = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("extractbits buffer signed"),
        contents: bytemuck::bytes_of(&input.cast_i32()),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let return_buffer_unsigned = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("return Buffer unsigned"),
        contents: bytemuck::cast_slice(&ret_buf1),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let return_buffer_signed = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("return Buffer signed"),
        contents: bytemuck::cast_slice(&ret_buf2),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader
    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(slice_size as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (std::mem::size_of::<ExtBitsUnsigned>()) as u64,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            (std::mem::size_of::<ExtBitsSigned>()) as u64,
                        ),
                    },
                    count: None,
                },
            ],
            label: None,
        });
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point,
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: return_buffer_unsigned.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: return_buffer_signed.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: ext_bits_buffer_unsigned.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: ext_bits_buffer_signed.as_entire_binding(),
            },
        ],
    });

    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker(entry_point);
        cpass.dispatch_workgroups(1, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
    encoder.copy_buffer_to_buffer(
        &return_buffer_unsigned,
        0,
        &staging_buffer_unsigned,
        0,
        size,
    );
    encoder.copy_buffer_to_buffer(&return_buffer_signed, 0, &staging_buffer_signed, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    // Note that we're not calling `.await` here.
    let buffer_slice_unsigned = staging_buffer_unsigned.slice(..);
    let buffer_slice_signed = staging_buffer_signed.slice(..);
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender1, receiver1) = futures_intrusive::channel::shared::oneshot_channel();
    let (sender2, receiver2) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice_unsigned.map_async(wgpu::MapMode::Read, move |v| sender1.send(v).unwrap());
    buffer_slice_signed.map_async(wgpu::MapMode::Read, move |v| sender2.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    // Awaits until `buffer_future` can be read from
    if let Some(Ok(())) = receiver1.receive().await {
        if let Some(Ok(())) = receiver2.receive().await {
            // Gets contents of buffer
            let data1 = buffer_slice_unsigned.get_mapped_range();
            let data2 = buffer_slice_signed.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let a: Vec<[u32; 4]> = bytemuck::cast_slice(&data1).to_vec();
            let b: Vec<[i32; 4]> = bytemuck::cast_slice(&data2).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data1);
            staging_buffer_unsigned.unmap(); // Unmaps buffer from memory
                                             // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                             //   delete myPointer;
                                             //   myPointer = NULL;
                                             // It effectively frees the memory

            // Returns data from buffer
            // Some(result[0])
            return Some((a[0], b[0]));
        }
        todo!("failed to receive from receiver2")
    } else {
        panic!("failed to run compute on gpu!")
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
