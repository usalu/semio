//! 🖼️ Textured, grid and post processing pipelines: the textured world samplers and pipeline, the grid, the postprocess, blur, blit and glass pipelines. See [`super`].

use super::*;

/// 🧶 The textured and grid shaders, the plane buffer and sampler, the material samplers and the textured pipeline.
pub(super) struct TexturedResources {
    pub(super) world_grid_shader: gpu_device::ShaderModule,
    pub(super) world_plane_vertex_buffer: gpu_device::Buffer,
    pub(super) world_plane_sampler: gpu_device::Sampler,
    pub(super) world_material_samplers: Vec<(crate::kernel_3d_scene::SceneTextureSampler3d, gpu_device::Sampler)>,
    pub(super) world_textured_pipeline: gpu_device::RenderPipeline,
}

/// 🧵 Builds the textured and grid shaders, the plane buffer and sampler, the material sampler matrix and the textured world pipeline.
pub(super) fn build_textured_resources(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    world_encoded_format: gpu_device::TextureFormat,
    world_bind_group_layout: &gpu_device::BindGroupLayout,
    scene_bind_group_layout: &gpu_device::BindGroupLayout,
) -> TexturedResources {
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
        device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_textured_pipeline_layout"), bind_group_layouts: &[Some(world_bind_group_layout), Some(scene_bind_group_layout)], immediate_size: 0 });
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
    TexturedResources { world_grid_shader, world_plane_vertex_buffer, world_plane_sampler, world_material_samplers, world_textured_pipeline }
}

/// 🕸 The procedural grid uniform buffer, bind group and pipeline.
pub(super) struct GridResources {
    pub(super) world_grid_uniform_buffer: gpu_device::Buffer,
    pub(super) world_grid_bind_group: gpu_device::BindGroup,
    pub(super) world_grid_pipeline: gpu_device::RenderPipeline,
}

/// 🔲 Builds the procedural grid layout, uniform buffer, bind group and pipeline.
pub(super) fn build_grid_resources(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    world_encoded_format: gpu_device::TextureFormat,
    world_bind_group_layout: &gpu_device::BindGroupLayout,
    world_grid_shader: &gpu_device::ShaderModule,
) -> GridResources {
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
        device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_grid_pipeline_layout"), bind_group_layouts: &[Some(world_bind_group_layout), Some(&world_grid_bind_group_layout)], immediate_size: 0 });
    let world_grid_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("world3d_grid_pipeline"),
        layout: Some(&world_grid_pipeline_layout),
        vertex: gpu_device::VertexState {
            module: world_grid_shader,
            entry_point: Some("vs_main"),
            buffers: &[gpu_device::VertexBufferLayout { array_stride: 20, step_mode: gpu_device::VertexStepMode::Vertex, attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }] }],
            compilation_options: Default::default(),
        },
        fragment: Some(gpu_device::FragmentState {
            module: world_grid_shader,
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
    GridResources { world_grid_uniform_buffer, world_grid_bind_group, world_grid_pipeline }
}

/// 🪄 The blur and postprocess globals buffers and the postprocess, blur, blit and glass pipelines.
pub(super) struct PostProcessingResources {
    pub(super) blur_globals_buffer: gpu_device::Buffer,
    pub(super) postprocess_globals_buffer: gpu_device::Buffer,
    pub(super) world_postprocess_pipeline: gpu_device::RenderPipeline,
    pub(super) blur_downsample_pipeline: gpu_device::RenderPipeline,
    pub(super) scene_blit_pipeline: gpu_device::RenderPipeline,
    pub(super) glass_pipeline: gpu_device::RenderPipeline,
}

/// ✨ Builds the blur and postprocess globals buffers and the postprocess, blur downsample, scene blit and glass pipelines.
pub(super) fn build_post_processing_resources(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    world_encoded_format: gpu_device::TextureFormat,
    globals_bind_group_layout: &gpu_device::BindGroupLayout,
    blur_shader: &gpu_device::ShaderModule,
    scene_blit_shader: &gpu_device::ShaderModule,
    glass_shader: &gpu_device::ShaderModule,
    world_postprocess_shader: &gpu_device::ShaderModule,
    blur_bind_group_layout: &gpu_device::BindGroupLayout,
    postprocess_bind_group_layout: &gpu_device::BindGroupLayout,
    scene_bind_group_layout: &gpu_device::BindGroupLayout,
) -> PostProcessingResources {
    let blur_globals_buffer =
        device.create_buffer_init(&gpu_device::util::BufferInitDescriptor { label: Some("blur_globals"), contents: bytemuck::bytes_of(&BlurGlobals { src_mip: 0.0, _pad: [0.0; 7] }), usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST });
    let postprocess_globals_buffer = device.create_buffer_init(&gpu_device::util::BufferInitDescriptor {
        label: Some("world_postprocess_globals"),
        contents: bytemuck::bytes_of(&WorldPostprocessGlobals::zeroed()),
        usage: gpu_device::BufferUsages::UNIFORM | gpu_device::BufferUsages::COPY_DST,
    });

    let world_postprocess_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world_postprocess_pipeline_layout"), bind_group_layouts: &[Some(postprocess_bind_group_layout)], immediate_size: 0 });
    let world_postprocess_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("world_postprocess_pipeline"),
        layout: Some(&world_postprocess_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_postprocess_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState { module: world_postprocess_shader, entry_point: Some("fs_main"), targets: &[Some(gpu_device::ColorTargetState { format: world_encoded_format, blend: Some(gpu_device::BlendState::REPLACE), write_mask: gpu_device::ColorWrites::ALL })], compilation_options: Default::default() }),
        primitive: gpu_device::PrimitiveState::default(),
        depth_stencil: None,
        multisample: gpu_device::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    let blur_downsample_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("blur_downsample_pipeline_layout"), bind_group_layouts: &[Some(blur_bind_group_layout)], immediate_size: 0 });
    let blur_downsample_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("blur_downsample_pipeline"),
        layout: Some(&blur_downsample_pipeline_layout),
        vertex: gpu_device::VertexState { module: blur_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: blur_shader,
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

    let scene_blit_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("scene_blit_pipeline_layout"), bind_group_layouts: &[Some(scene_bind_group_layout)], immediate_size: 0 });
    let scene_blit_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("scene_blit_pipeline"),
        layout: Some(&scene_blit_pipeline_layout),
        vertex: gpu_device::VertexState { module: scene_blit_shader, entry_point: Some("vs_main"), buffers: &[], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: scene_blit_shader,
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

    let glass_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("glass_pipeline_layout"), bind_group_layouts: &[Some(globals_bind_group_layout), Some(scene_bind_group_layout)], immediate_size: 0 });
    let glass_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("glass_pipeline"),
        layout: Some(&glass_pipeline_layout),
        vertex: gpu_device::VertexState {
            module: glass_shader,
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
            module: glass_shader,
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
    PostProcessingResources { blur_globals_buffer, postprocess_globals_buffer, world_postprocess_pipeline, blur_downsample_pipeline, scene_blit_pipeline, glass_pipeline }
}
