//! 🏗️ Pipeline resources construction: the single `PipelineResources::new` that builds every pipeline, atlas, target and bind group. See [`super`].

use super::*;

impl PipelineResources {
    pub fn new(device: &gpu_device::Device, queue: &gpu_device::Queue, format: gpu_device::TextureFormat) -> Self {
        let world_encoded_format = format.remove_srgb_suffix();
        let globals_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("ui_globals_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
                gpu_device::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 4, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let glyph_bind_group_layout = globals_bind_group_layout.clone();
        let _ = glyph_bind_group_layout;

        let ui_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("ui_shader"), source: gpu_device::ShaderSource::Wgsl(UI_SHADER.into()) });
        let vector_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("vector_shader"), source: gpu_device::ShaderSource::Wgsl(VECTOR_SHADER.into()) });
        let world_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_SHADER.into()) });
        let world_lines_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_lines_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_LINES_SHADER.into()) });

        let depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(true),
            depth_compare: Some(gpu_device::CompareFunction::Less),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });
        let overlay_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::Always),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });

        let quad_vertices: &[f32] = &[0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let quad_vertex_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("ui_quad_vertices"), contents: bytemuck::cast_slice(quad_vertices), usage: gpu_device::BufferUsages::VERTEX });

        let globals_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor {
            label: Some("ui_globals"),
            contents: bytemuck::bytes_of(&UiGlobals { screen_size: [1.0, 1.0], _pad: [0.0, 0.0] }),
            usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
        });

        let glyph_texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("glyph_atlas"),
            size: gpu_device::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: gpu_device::TextureFormat::R8Unorm,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let glyph_view = glyph_texture.create_view(&gpu_device::TextureViewDescriptor::default());
        let glyph_sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("glyph_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let icon_texture = device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("icon_atlas"),
            size: gpu_device::Extent3d { width: ICON_ATLAS_TEXTURE_SIZE, height: ICON_ATLAS_TEXTURE_SIZE, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: gpu_device::TextureFormat::Rgba8UnormSrgb,
            usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let icon_view = icon_texture.create_view(&gpu_device::TextureViewDescriptor::default());
        let icon_sampler = device.create_sampler(&gpu_device::SamplerDescriptor { label: Some("icon_sampler"), mag_filter: gpu_device::FilterMode::Linear, min_filter: gpu_device::FilterMode::Linear, ..Default::default() });
        let glyph_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("ui_bind_group"),
            layout: &globals_bind_group_layout,
            entries: &[
                gpu_device::BindGroupEntry { binding: 0, resource: globals_buffer.as_entire_binding() },
                gpu_device::BindGroupEntry { binding: 1, resource: gpu_device::BindingResource::TextureView(&glyph_view) },
                gpu_device::BindGroupEntry { binding: 2, resource: gpu_device::BindingResource::Sampler(&glyph_sampler) },
                gpu_device::BindGroupEntry { binding: 3, resource: gpu_device::BindingResource::TextureView(&icon_view) },
                gpu_device::BindGroupEntry { binding: 4, resource: gpu_device::BindingResource::Sampler(&icon_sampler) },
            ],
        });
        let ui_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout)], immediate_size: 0 });
        #[cfg(any(test,feature="testkit"))]
        let mask_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("silhouette_mask_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState { module: &ui_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format, blend: None, write_mask: gpu_device::ColorWrites::empty() })], compilation_options: Default::default() }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::Always),
                stencil: mask_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let ui_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("ui_pipeline"),
            layout: Some(&ui_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &ui_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<UiInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &ui_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: overlay_depth_state.clone(),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vector_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("vector_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout)], immediate_size: 0 });
        let vector_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("vector_pipeline"),
            layout: Some(&vector_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &vector_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout {
                    array_stride: size_of::<VectorVertex>() as gpu_device::BufferAddress,
                    step_mode: gpu_device::VertexStepMode::Vertex,
                    attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }, gpu_device::VertexAttribute { offset: 8, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &vector_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: overlay_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let world_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_globals_layout"),
            entries: &[gpu_device::BindGroupLayoutEntry {
                binding: 0,
                visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: true, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGlobals>() as u64) },
                count: None,
            }],
        });

        let world_globals_ring = WorldGlobalsRing::new(device, &world_bind_group_layout, 8);

        let world_shadow_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_shadow_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Depth, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Comparison), count: None },
            ],
        });
        let world_shadow_target = WorldShadowTarget::new(device, &world_shadow_bind_group_layout, 1);

        let world_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout)], immediate_size: 0 });
        let world_mesh_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_mesh_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout)], immediate_size: 0 });
        let world_vertex_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dVertex>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Vertex,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 },
                gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x3 },
                gpu_device::VertexAttribute { offset: 24, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 40, shader_location: 9, format: gpu_device::VertexFormat::Float32x2 },
            ],
        };
        let world_instance_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dGpuInstance>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Instance,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 96, shader_location: 10, format: gpu_device::VertexFormat::Float32x4 },
            ],
        };
        let world_surface_vertex_layout=||gpu_device::VertexBufferLayout {array_stride:size_of::<World3dVertex>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Vertex,attributes:&[
            gpu_device::VertexAttribute{offset:0,shader_location:0,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:12,shader_location:1,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:24,shader_location:2,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:40,shader_location:9,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:56,shader_location:11,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:72,shader_location:12,format:gpu_device::VertexFormat::Float32x2},gpu_device::VertexAttribute{offset:80,shader_location:13,format:gpu_device::VertexFormat::Float32x4},]};
        let world_surface_instance_layout=||gpu_device::VertexBufferLayout{array_stride:size_of::<World3dGpuInstance>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Instance,attributes:&[
            gpu_device::VertexAttribute{offset:0,shader_location:3,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:16,shader_location:4,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:32,shader_location:5,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:48,shader_location:6,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:64,shader_location:7,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:80,shader_location:8,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:96,shader_location:10,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:112,shader_location:15,format:gpu_device::VertexFormat::Float32x4},]};
        let world_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let translucent_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 },
        });
        let world_line_depth_state = Some(gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(false),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: gpu_device::DepthBiasState::default(),
        });
        let world_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_pipeline_translucent"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: translucent_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let section_toggle_face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Equal, fail_op: gpu_device::StencilOperation::Keep, depth_fail_op: gpu_device::StencilOperation::Invert, pass_op: gpu_device::StencilOperation::Invert };
        let world_section_stencil_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_section_stencil_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_section_stencil"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: None, write_mask: gpu_device::ColorWrites::empty() })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::Always),
                stencil: gpu_device::StencilState { front: section_toggle_face, back: section_toggle_face, read_mask: !SECTION_STENCIL_BIT & 0xff, write_mask: SECTION_STENCIL_BIT },
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let section_cap_face = gpu_device::StencilFaceState { compare: gpu_device::CompareFunction::Equal, fail_op: gpu_device::StencilOperation::Keep, depth_fail_op: gpu_device::StencilOperation::Keep, pass_op: gpu_device::StencilOperation::Keep };
        let world_section_cap_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_section_cap_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_section_cap"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(true),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: gpu_device::StencilState { front: section_cap_face, back: section_cap_face, read_mask: 0xff, write_mask: 0x00 },
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_shadow_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_shadow_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_shadow"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: None,
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: gpu_device::StencilState::default(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_line_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_line_pipeline"),
            layout: Some(&world_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_lines_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout {
                    array_stride: size_of::<WorldLineGpuVertex>() as gpu_device::BufferAddress,
                    step_mode: gpu_device::VertexStepMode::Vertex,
                    attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }, gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_lines_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { topology: gpu_device::PrimitiveTopology::LineList, ..Default::default() },
            depth_stencil: world_line_depth_state,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("blur_downsample_shader"), source: gpu_device::ShaderSource::Wgsl(BLUR_DOWNSAMPLE_SHADER.into()) });
        let scene_blit_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("scene_blit_shader"), source: gpu_device::ShaderSource::Wgsl(SCENE_BLIT_SHADER.into()) });
        let glass_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("glass_shader"), source: gpu_device::ShaderSource::Wgsl(GLASS_SHADER.into()) });
        let world_postprocess_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world_postprocess_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_POSTPROCESS_SHADER.into()) });

        let blur_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("blur_downsample_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<BlurGlobals>() as u64) },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let postprocess_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world_postprocess_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry { binding: 0, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<WorldPostprocessGlobals>() as u64) }, count: None },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false }, count: None },
                gpu_device::BindGroupLayoutEntry { binding: 2, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
                gpu_device::BindGroupLayoutEntry { binding: 3, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false }, count: None },
            ],
        });

        let scene_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("scene_sample_layout"),
            entries: &[
                gpu_device::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: gpu_device::ShaderStages::FRAGMENT,
                    ty: gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false },
                    count: None,
                },
                gpu_device::BindGroupLayoutEntry { binding: 1, visibility: gpu_device::ShaderStages::FRAGMENT, ty: gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering), count: None },
            ],
        });

        let authored_entries: Vec<_> = (0..10).map(|binding| gpu_device::BindGroupLayoutEntry { binding, visibility: gpu_device::ShaderStages::FRAGMENT, ty: if binding % 2 == 0 { gpu_device::BindingType::Texture { sample_type: gpu_device::TextureSampleType::Float { filterable: true }, view_dimension: gpu_device::TextureViewDimension::D2, multisampled: false } } else { gpu_device::BindingType::Sampler(gpu_device::SamplerBindingType::Filtering) }, count: None }).collect();
        let world_authored_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor { label: Some("world3d_authored_surface_layout"), entries: &authored_entries });
        let authored_white = device.create_texture(&gpu_device::TextureDescriptor { label: Some("world3d_authored_white"), size: gpu_device::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: gpu_device::TextureDimension::D2, format: gpu_device::TextureFormat::Rgba8Unorm, usage: gpu_device::TextureUsages::TEXTURE_BINDING | gpu_device::TextureUsages::COPY_DST, view_formats: &[] });
        queue.write_texture(gpu_device::TexelCopyTextureInfo { texture: &authored_white, mip_level: 0, origin: gpu_device::Origin3d::ZERO, aspect: gpu_device::TextureAspect::All }, &[255;4], gpu_device::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(4), rows_per_image: Some(1) }, gpu_device::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 });
        let world_authored_white = authored_white.create_view(&Default::default());
        let world_authored_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_authored_shader"), source: gpu_device::ShaderSource::Wgsl(world3d_authored_shader().into()) });
        let world_authored_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_authored_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout), Some(&world_authored_bind_group_layout)], immediate_size: 0 });
        let world_painted_source = world3d_painted_shader();
        let world_painted_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_painted_shader"), source: gpu_device::ShaderSource::Wgsl(world_painted_source.into()) });
        let world_celebration_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_celebration_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_CELEBRATION_SHADER.into()) });
        let world_painted_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor {
            label: Some("world3d_painted_pipeline_layout"),
            bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_shadow_bind_group_layout), Some(&scene_bind_group_layout)],
            immediate_size: 0,
        });
        let world_celebration_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_celebration_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout)], immediate_size: 0 });
        let celebration_instance_layout = || gpu_device::VertexBufferLayout {
            array_stride: size_of::<World3dCelebrationGpuInstance>() as gpu_device::BufferAddress,
            step_mode: gpu_device::VertexStepMode::Instance,
            attributes: &[
                gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 96, shader_location: 10, format: gpu_device::VertexFormat::Float32x4 },
                gpu_device::VertexAttribute { offset: 112, shader_location: 11, format: gpu_device::VertexFormat::Float32x4 },
            ],
        };
        let material_depth = |write| gpu_device::DepthStencilState {
            format: gpu_device::TextureFormat::Depth24PlusStencil8,
            depth_write_enabled: Some(write),
            depth_compare: Some(gpu_device::CompareFunction::LessEqual),
            stencil: content_stencil_state(),
            bias: if write { gpu_device::DepthBiasState::default() } else { gpu_device::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 } },
        };
        let world_standard_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_standard_translucent_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_front_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_front_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_double_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_double_translucent_pipeline"),
            layout: Some(&world_mesh_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_painted_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_painted_pipeline"),
            layout: Some(&world_painted_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_painted_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_double_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_double_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_double_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_double_translucent_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_front_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_front_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_authored_painted_front_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_authored_painted_front_translucent_pipeline"),
            layout: Some(&world_authored_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_authored_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: Some(gpu_device::Face::Back), ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_painted_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_painted_pipeline_translucent"),
            layout: Some(&world_painted_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_painted_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_celebration_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_celebration_pipeline"),
            layout: Some(&world_celebration_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_celebration_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(true)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_celebration_pipeline_translucent = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_celebration_pipeline_translucent"),
            layout: Some(&world_celebration_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &world_celebration_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(material_depth(false)),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        let world_textured_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_textured_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_TEXTURED_SHADER.into()) });
        let world_grid_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_grid_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_GRID_SHADER.into()) });
        let world_plane_vertex_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("world3d_plane_vertices"), contents: bytemuck::cast_slice(WORLD_PLANE_VERTICES), usage: gpu_device::BufferUsages::VERTEX });
        let world_plane_sampler = device.create_sampler(&gpu_device::SamplerDescriptor {
            label: Some("world3d_plane_sampler"),
            address_mode_u: gpu_device::AddressMode::ClampToEdge,
            address_mode_v: gpu_device::AddressMode::ClampToEdge,
            address_mode_w: gpu_device::AddressMode::ClampToEdge,
            mag_filter: gpu_device::FilterMode::Linear,
            min_filter: gpu_device::FilterMode::Linear,
            mipmap_filter: gpu_device::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let mut world_material_samplers = Vec::with_capacity(108);
        for wrap_u in [
            crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge,
            crate::kernel_3d_scene::SceneTextureWrap3d::Repeat,
            crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat,
        ] {
            for wrap_v in [
                crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge,
                crate::kernel_3d_scene::SceneTextureWrap3d::Repeat,
                crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat,
            ] {
                for mag_filter in [crate::kernel_3d_scene::SceneTextureFilter3d::Nearest, crate::kernel_3d_scene::SceneTextureFilter3d::Linear] {
                    for min_filter in [crate::kernel_3d_scene::SceneTextureFilter3d::Nearest, crate::kernel_3d_scene::SceneTextureFilter3d::Linear,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapNearest,crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapNearest,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear,crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapLinear] {
                        let descriptor = crate::kernel_3d_scene::SceneTextureSampler3d { wrap_u, wrap_v, mag_filter, min_filter };
                        let address = |mode| match mode {
                            crate::kernel_3d_scene::SceneTextureWrap3d::ClampToEdge => gpu_device::AddressMode::ClampToEdge,
                            crate::kernel_3d_scene::SceneTextureWrap3d::Repeat => gpu_device::AddressMode::Repeat,
                            crate::kernel_3d_scene::SceneTextureWrap3d::MirrorRepeat => gpu_device::AddressMode::MirrorRepeat,
                        };
                        let filter = |mode| match mode {
                            crate::kernel_3d_scene::SceneTextureFilter3d::Nearest | crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapNearest | crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear => gpu_device::FilterMode::Nearest,
                            _ => gpu_device::FilterMode::Linear,
                        };
                        let sampler = device.create_sampler(&gpu_device::SamplerDescriptor {
                            label: Some("world3d_material_sampler"),
                            address_mode_u: address(wrap_u),
                            address_mode_v: address(wrap_v),
                            address_mode_w: gpu_device::AddressMode::Repeat,
                            mag_filter: filter(mag_filter),
                            min_filter: filter(min_filter),
                            mipmap_filter:if matches!(min_filter,crate::kernel_3d_scene::SceneTextureFilter3d::NearestMipmapLinear|crate::kernel_3d_scene::SceneTextureFilter3d::LinearMipmapLinear){gpu_device::MipmapFilterMode::Linear}else{gpu_device::MipmapFilterMode::Nearest},
                            lod_max_clamp:if matches!(min_filter,crate::kernel_3d_scene::SceneTextureFilter3d::Nearest|crate::kernel_3d_scene::SceneTextureFilter3d::Linear){0.0}else{32.0},
                            ..Default::default()
                        });
                        world_material_samplers.push((descriptor, sampler));
                    }
                }
            }
        }
        let world_textured_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_textured_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&scene_bind_group_layout)], immediate_size: 0 });
        let world_textured_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_textured_pipeline"),
            layout: Some(&world_textured_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_textured_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout {
                        array_stride: 20,
                        step_mode: gpu_device::VertexStepMode::Vertex,
                        attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }, gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x2 }],
                    },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<World3dTexturedGpuInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 4, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 5, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 48, shader_location: 6, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 64, shader_location: 7, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 80, shader_location: 8, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_textured_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: content_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        let world_grid_bind_group_layout = device.create_bind_group_layout(&gpu_device::BindGroupLayoutDescriptor {
            label: Some("world3d_grid_layout"),
            entries: &[gpu_device::BindGroupLayoutEntry {
                binding: 0,
                visibility: gpu_device::ShaderStages::VERTEX | gpu_device::ShaderStages::FRAGMENT,
                ty: gpu_device::BindingType::Buffer { ty: gpu_device::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: std::num::NonZeroU64::new(size_of::<World3dGridUniforms>() as u64) },
                count: None,
            }],
        });
        let world_grid_uniform_buffer =
            device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("world3d_grid_uniforms"), contents: bytemuck::bytes_of(&World3dGridUniforms::zeroed()), usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST });
        let world_grid_bind_group = device.create_bind_group(&gpu_device::BindGroupDescriptor {
            label: Some("world3d_grid_bind_group"),
            layout: &world_grid_bind_group_layout,
            entries: &[gpu_device::BindGroupEntry { binding: 0, resource: world_grid_uniform_buffer.as_entire_binding() }],
        });
        let world_grid_pipeline_layout =
            device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_grid_pipeline_layout"), bind_group_layouts: &[Some(&world_bind_group_layout), Some(&world_grid_bind_group_layout)], immediate_size: 0 });
        let world_grid_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world3d_grid_pipeline"),
            layout: Some(&world_grid_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &world_grid_shader,
                entry_point: Some("vs_main"),
                buffers: &[gpu_device::VertexBufferLayout { array_stride: 20, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }] }],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &world_grid_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState { cull_mode: None, ..Default::default() },
            depth_stencil: Some(gpu_device::DepthStencilState {
                format: gpu_device::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: Some(false),
                depth_compare: Some(gpu_device::CompareFunction::LessEqual),
                stencil: content_stencil_state(),
                bias: gpu_device::DepthBiasState::default(),
            }),
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_globals_buffer =
            device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("blur_globals"), contents: bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }), usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST });
        let postprocess_globals_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor {
            label: Some("world_postprocess_globals"),
            contents: bytemuck::bytes_of(&WorldPostprocessGlobals::zeroed()),
            usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
        });

        let world_postprocess_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world_postprocess_pipeline_layout"), bind_group_layouts: &[Some(&postprocess_bind_group_layout)], immediate_size: 0 });
        let world_postprocess_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("world_postprocess_pipeline"),
            layout: Some(&world_postprocess_pipeline_layout),
            vertex: gpu_device::VertexState { module: &world_postprocess_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState { module: &world_postprocess_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let blur_downsample_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[Some(&blur_bind_group_layout)], immediate_size: 0 });
        let blur_downsample_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("blur_downsample_pipeline"),
            layout: Some(&blur_downsample_pipeline_layout),
            vertex: gpu_device::VertexState { module: &blur_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &blur_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let scene_blit_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[Some(&scene_bind_group_layout)], immediate_size: 0 });
        let scene_blit_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("scene_blit_pipeline"),
            layout: Some(&scene_blit_pipeline_layout),
            vertex: gpu_device::VertexState { module: &scene_blit_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
            fragment: Some(gpu_device::FragmentState {
                module: &scene_blit_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let glass_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[Some(&globals_bind_group_layout), Some(&scene_bind_group_layout)], immediate_size: 0 });
        let glass_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
            label: Some("glass_pipeline"),
            layout: Some(&glass_pipeline_layout),
            vertex: gpu_device::VertexState {
                module: &glass_shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    gpu_device::VertexBufferLayout { array_stride: 8, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }] },
                    gpu_device::VertexBufferLayout {
                        array_stride: size_of::<GlassInstance>() as gpu_device::BufferAddress,
                        step_mode: gpu_device::VertexStepMode::Instance,
                        attributes: &[
                            gpu_device::VertexAttribute { offset: 0, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 16, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
                            gpu_device::VertexAttribute { offset: 32, shader_location: 3, format: gpu_device::VertexFormat::Float32x4 },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(gpu_device::FragmentState {
                module: &glass_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(gpu_device::ColorTargetState { format, blend: Some(gpu_device::BlendState::ALPHA_BLENDING), write_mask: gpu_device::ColorWrites::ALL })],
                compilation_options: Default::default(),
            }),
            primitive: gpu_device::PrimitiveState::default(),
            depth_stencil: None,
            multisample: gpu_device::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let _ = queue;
        Self {
            #[cfg(any(test,feature="testkit"))]
            mask_pipeline,
            ui_pipeline,
            vector_pipeline,
            world_pipeline,
            world_pipeline_translucent,
            world_section_stencil_pipeline,
            world_section_cap_pipeline,
            world_authored_front_pipeline,
            world_authored_double_translucent_pipeline,
            world_standard_translucent_pipeline,
            world_painted_pipeline,
            world_painted_pipeline_translucent,
            world_authored_painted_double_pipeline,
            world_authored_painted_double_translucent_pipeline,
            world_authored_painted_front_pipeline,
            world_authored_painted_front_translucent_pipeline,
            world_celebration_pipeline,
            world_celebration_pipeline_translucent,
            world_shadow_pipeline,
            world_line_pipeline,
            world_grid_pipeline,
            world_textured_pipeline,
            blur_downsample_pipeline,
            scene_blit_pipeline,
            glass_pipeline,
            world_postprocess_pipeline,
            quad_vertex_buffer,
            world_plane_vertex_buffer,
            world_plane_sampler,
            world_material_samplers,
            world_grid_uniform_buffer,
            world_grid_bind_group,
            globals_buffer,
            blur_globals_buffer,
            postprocess_globals_buffer,
            world_globals_ring,
            world_bind_group_layout,
            world_shadow_bind_group_layout,
            world_shadow_target,
            blur_bind_group_layout,
            postprocess_bind_group_layout,
            scene_bind_group_layout,
            world_authored_bind_group_layout,
            world_authored_white,
            glyph_texture,
            glyph_sampler,
            icon_texture,
            icon_sampler,
            glyph_bind_group,
            bind_group_layout: globals_bind_group_layout,
            surface_scale: 1.0,
        }
    }

}
