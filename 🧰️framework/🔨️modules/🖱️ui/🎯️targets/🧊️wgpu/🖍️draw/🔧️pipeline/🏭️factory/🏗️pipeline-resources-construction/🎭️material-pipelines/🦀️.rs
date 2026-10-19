//! 🎭️ Material pipelines: the authored, painted and celebration layouts and the thirteen material pipelines. See [`super`].

use super::*;

/// 🎚 The blur, postprocess, scene and authored layouts and the material shaders and pipeline layouts.
pub(super) struct MaterialLayouts {
    pub(super) blur_shader: gpu_device::ShaderModule,
    pub(super) scene_blit_shader: gpu_device::ShaderModule,
    pub(super) glass_shader: gpu_device::ShaderModule,
    pub(super) world_postprocess_shader: gpu_device::ShaderModule,
    pub(super) blur_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) postprocess_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) scene_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) world_authored_bind_group_layout: gpu_device::BindGroupLayout,
    pub(super) world_authored_white: gpu_device::TextureView,
    pub(super) world_authored_shader: gpu_device::ShaderModule,
    pub(super) world_authored_pipeline_layout: gpu_device::PipelineLayout,
    pub(super) world_painted_shader: gpu_device::ShaderModule,
    pub(super) world_celebration_shader: gpu_device::ShaderModule,
    pub(super) world_painted_pipeline_layout: gpu_device::PipelineLayout,
    pub(super) world_celebration_pipeline_layout: gpu_device::PipelineLayout,
}

/// 🎛 Builds the post processing shaders, the blur, postprocess, scene and authored layouts, the white authored texture and the authored, painted and celebration shaders and pipeline layouts.
pub(super) fn build_material_layouts_and_shaders(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    queue: &gpu_device::Queue,
    world_bind_group_layout: &gpu_device::BindGroupLayout,
    world_shadow_bind_group_layout: &gpu_device::BindGroupLayout,
) -> MaterialLayouts {
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
    let world_authored_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_authored_pipeline_layout"), bind_group_layouts: &[Some(world_bind_group_layout), Some(world_shadow_bind_group_layout), Some(&world_authored_bind_group_layout)], immediate_size: 0 });
    let world_painted_source = world3d_painted_shader();
    let world_painted_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_painted_shader"), source: gpu_device::ShaderSource::Wgsl(world_painted_source.into()) });
    let world_celebration_shader = device.create_shader_module(gpu_device::ShaderModuleDescriptor { label: Some("world3d_celebration_shader"), source: gpu_device::ShaderSource::Wgsl(WORLD3D_CELEBRATION_SHADER.into()) });
    let world_painted_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor {
        label: Some("world3d_painted_pipeline_layout"),
        bind_group_layouts: &[Some(world_bind_group_layout), Some(world_shadow_bind_group_layout), Some(&scene_bind_group_layout)],
        immediate_size: 0,
    });
    let world_celebration_pipeline_layout = device.create_pipeline_layout(&gpu_device::PipelineLayoutDescriptor { label: Some("world3d_celebration_pipeline_layout"), bind_group_layouts: &[Some(world_bind_group_layout)], immediate_size: 0 });
    MaterialLayouts { blur_shader, scene_blit_shader, glass_shader, world_postprocess_shader, blur_bind_group_layout, postprocess_bind_group_layout, scene_bind_group_layout, world_authored_bind_group_layout, world_authored_white, world_authored_shader, world_authored_pipeline_layout, world_painted_shader, world_celebration_shader, world_painted_pipeline_layout, world_celebration_pipeline_layout }
}

/// 🖍 The thirteen material pipelines.
pub(super) struct MaterialPipelines {
    pub(super) world_standard_translucent_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_front_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_double_translucent_pipeline: gpu_device::RenderPipeline,
    pub(super) world_painted_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_painted_double_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_painted_double_translucent_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_painted_front_pipeline: gpu_device::RenderPipeline,
    pub(super) world_authored_painted_front_translucent_pipeline: gpu_device::RenderPipeline,
    pub(super) world_painted_pipeline_translucent: gpu_device::RenderPipeline,
    pub(super) world_celebration_pipeline: gpu_device::RenderPipeline,
    pub(super) world_celebration_pipeline_translucent: gpu_device::RenderPipeline,
}

/// 🎨 Builds the standard, authored, painted and celebration material pipelines in their opaque and translucent variants.
pub(super) fn build_material_pipelines(
    device: &gpu_device::Device,
    format: gpu_device::TextureFormat,
    world_encoded_format: gpu_device::TextureFormat,
    world_shader: &gpu_device::ShaderModule,
    world_mesh_pipeline_layout: &gpu_device::PipelineLayout,
    world_authored_shader: &gpu_device::ShaderModule,
    world_authored_pipeline_layout: &gpu_device::PipelineLayout,
    world_painted_shader: &gpu_device::ShaderModule,
    world_celebration_shader: &gpu_device::ShaderModule,
    world_painted_pipeline_layout: &gpu_device::PipelineLayout,
    world_celebration_pipeline_layout: &gpu_device::PipelineLayout,
) -> MaterialPipelines {
    let world_standard_translucent_pipeline = device.create_render_pipeline(&gpu_device::RenderPipelineDescriptor {
        label: Some("world3d_standard_translucent_pipeline"),
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_mesh_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_shader,
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
        layout: Some(world_painted_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_painted_shader,
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
        layout: Some(world_authored_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_authored_shader,
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
        layout: Some(world_authored_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_authored_shader,
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
        layout: Some(world_authored_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_authored_shader,
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
        layout: Some(world_authored_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_authored_shader, entry_point: Some("vs_main"), buffers: &[world_surface_vertex_layout(), world_surface_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_authored_shader,
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
        layout: Some(world_painted_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_painted_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), world_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_painted_shader,
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
        layout: Some(world_celebration_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_celebration_shader,
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
        layout: Some(world_celebration_pipeline_layout),
        vertex: gpu_device::VertexState { module: world_celebration_shader, entry_point: Some("vs_main"), buffers: &[world_vertex_layout(), celebration_instance_layout()], compilation_options: Default::default() },
        fragment: Some(gpu_device::FragmentState {
            module: world_celebration_shader,
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
    MaterialPipelines { world_standard_translucent_pipeline, world_authored_front_pipeline, world_authored_double_translucent_pipeline, world_painted_pipeline, world_authored_painted_double_pipeline, world_authored_painted_double_translucent_pipeline, world_authored_painted_front_pipeline, world_authored_painted_front_translucent_pipeline, world_painted_pipeline_translucent, world_celebration_pipeline, world_celebration_pipeline_translucent }
}
