//! 🔰️ Base resources and ui pipelines: the globals layout, shaders, atlases, glyph bind group and the ui, vector and mask pipelines. See [`super`].

use super::*;

/// 📦 The globals layout, shaders, depth states, buffers, atlases and glyph bind group that every later pipeline reads.
pub(super) struct BaseResources {
    pub(super) world_encoded_format: gpu_device::TextureFormat,
    pub(super) globals_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) ui_shader: gpu_device::ShaderModule,
    pub(super) vector_shader: gpu_device::ShaderModule,
    pub(super) world_shader: gpu_device::ShaderModule,
    pub(super) world_lines_shader: gpu_device::ShaderModule,
    pub(super) depth_state: Option<gpu_device::DepthStencilState>,
    pub(super) overlay_depth_state: Option<gpu_device::DepthStencilState>,
    pub(super) quad_vertex_buffer: gpu_device::Buffer,
    pub(super) globals_buffer: gpu_device::Buffer,
    pub(super) glyph_texture: gpu_device::Texture,
    pub(super) glyph_sampler: gpu_device::Sampler,
    pub(super) icon_texture: gpu_device::Texture,
    pub(super) icon_sampler: gpu_device::Sampler,
    pub(super) glyph_bind_group: gpu_device::BindGroup,
}

/// 🧱 Builds the globals layout, the ui, vector and world shaders, the depth states, the quad and globals buffers, the glyph and icon atlases and the glyph bind group.
pub(super) fn build_base_resources(device: &gpu_device::Device, format: gpu_device::TextureFormat) -> BaseResources {
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
    BaseResources { world_encoded_format, globals_bind_group_layout, ui_shader, vector_shader, world_shader, world_lines_shader, depth_state, overlay_depth_state, quad_vertex_buffer, globals_buffer, glyph_texture, glyph_sampler, icon_texture, icon_sampler, glyph_bind_group }
}

/// 🗂 The ui, vector and mask pipelines.
pub(super) struct UiPipelineSet {
    #[cfg(any(test,feature="testkit"))]
    pub(super) mask_pipeline: gpu_device::RenderPipeline,
    pub(super) ui_pipeline: gpu_device::RenderPipeline,
    pub(super) vector_pipeline: gpu_device::RenderPipeline,
}

/// 🖌 Builds the ui pipeline layout, the silhouette mask pipeline (tests and testkit) and the ui and vector pipelines.
pub(super) fn build_ui_pipelines(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    globals_bind_group_layout: &gpu_device::BindGroupLayout,
    ui_shader: &gpu_device::ShaderModule,
    vector_shader: &gpu_device::ShaderModule,
    overlay_depth_state: Option<gpu_device::DepthStencilState>,
) -> UiPipelineSet {
    let ui_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("ui_pipeline_layout"), bind_group_layouts: &[Some(globals_bind_group_layout)], immediate_size: 0 });
    #[cfg(any(test,feature="testkit"))]
    let mask_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("silhouette_mask_pipeline"),
        layout: Some(&ui_pipeline_layout),
        vertex: gpu_device::VertexState {
            module: ui_shader,
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
        fragment: Some(gpu_device::FragmentState { module: ui_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format, blend: None, write_mask: gpu_device::ColorWrites::empty() })], compilation_options: Default::default() }),
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
            module: ui_shader,
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
            module: ui_shader,
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

    let vector_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("vector_pipeline_layout"), bind_group_layouts: &[Some(globals_bind_group_layout)], immediate_size: 0 });
    let vector_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("vector_pipeline"),
        layout: Some(&vector_pipeline_layout),
        vertex: gpu_device::VertexState {
            module: vector_shader,
            entry_point: Some("vs_main"),
            buffers: &[gpu_device::VertexBufferLayout {
                array_stride: size_of::<VectorVertex>() as gpu_device::BufferAddress,
                step_mode: gpu_device::VertexStepMode::Vertex,
                attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x2 }, gpu_device::VertexAttribute { offset: 8, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(gpu_device::FragmentState {
            module: vector_shader,
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
    UiPipelineSet { #[cfg(any(test,feature="testkit"))] mask_pipeline, ui_pipeline, vector_pipeline }
}
