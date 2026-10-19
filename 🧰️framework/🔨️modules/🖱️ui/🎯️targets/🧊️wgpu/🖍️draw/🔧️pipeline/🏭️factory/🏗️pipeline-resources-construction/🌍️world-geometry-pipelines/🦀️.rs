//! 🌍️ World geometry pipelines: the world layouts, shadow target and the mesh, section, shadow and line pipelines. See [`super`].

use super::*;

/// 🧭 The world bind group layouts, the globals ring, the shadow target and the world pipeline layouts.
pub(super) struct WorldLayouts {
    pub(super) world_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) world_globals_ring: WorldGlobalsRing,
    pub(super) world_shadow_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) world_shadow_target: WorldShadowTarget,
    pub(super) world_pipeline_layout: gpu_device::PipelineLayout,
    pub(super) world_mesh_pipeline_layout: gpu_device::PipelineLayout,
}

/// 🌐 Builds the world and shadow bind group layouts, the globals ring, the shadow target and the world pipeline layouts.
pub(super) fn build_world_layouts_and_targets(device: &gpu_device::Device) -> WorldLayouts {
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
    WorldLayouts { world_bind_group_layout, world_globals_ring, world_shadow_bind_group_layout, world_shadow_target, world_pipeline_layout, world_mesh_pipeline_layout }
}

/// 🏗 The world mesh, section, shadow and line pipelines and the line depth state.
pub(super) struct WorldGeometryPipelines {
    pub(super) world_pipeline: gpu_device::RenderPipeline,
    pub(super) world_pipeline_translucent: gpu_device::RenderPipeline,
    pub(super) world_section_stencil_pipeline: gpu_device::RenderPipeline,
    pub(super) world_section_cap_pipeline: gpu_device::RenderPipeline,
    pub(super) world_shadow_pipeline: gpu_device::RenderPipeline,
    pub(super) world_line_pipeline: gpu_device::RenderPipeline,
}

/// 🏔 Builds the world mesh, translucent, section stencil, section cap, shadow and line pipelines and their depth states.
pub(super) fn build_world_geometry_pipelines(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    world_encoded_format: gpu_device::TextureFormat,
    world_shader: &gpu_device::ShaderModule,
    world_lines_shader: &gpu_device::ShaderModule,
    depth_state: Option<gpu_device::DepthStencilState>,
    world_pipeline_layout: &gpu_device::PipelineLayout,
    world_mesh_pipeline_layout: &gpu_device::PipelineLayout,
) -> WorldGeometryPipelines {
    let world_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("world3d_pipeline"),
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_shadow"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
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
        layout: Some(world_pipeline_layout),
        vertex: gpu_device::VertexState {
            module: world_lines_shader,
            entry_point: Some("vs_main"),
            buffers: &[gpu_device::VertexBufferLayout {
                array_stride: size_of::<WorldLineGpuVertex>() as gpu_device::BufferAddress,
                step_mode: gpu_device::VertexStepMode::Vertex,
                attributes: &[gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 }, gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x4 }],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(gpu_device::FragmentState {
            module: world_lines_shader,
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
    WorldGeometryPipelines { world_pipeline, world_pipeline_translucent, world_section_stencil_pipeline, world_section_cap_pipeline, world_shadow_pipeline, world_line_pipeline }
}
