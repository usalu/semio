//! 📏️ Vertex layouts and material depth: the vertex and instance buffer layouts and the depth stencil state shared by the world pipelines. See [`super`].

use super::*;

/// 🧊 Vertex buffer layout of the world3d mesh vertex stream.
pub(super) fn world_vertex_layout() -> gpu_device::VertexBufferLayout<'static> {
    gpu_device::VertexBufferLayout {
        array_stride: size_of::<World3dVertex>() as gpu_device::BufferAddress,
        step_mode: gpu_device::VertexStepMode::Vertex,
        attributes: &[
            gpu_device::VertexAttribute { offset: 0, shader_location: 0, format: gpu_device::VertexFormat::Float32x3 },
            gpu_device::VertexAttribute { offset: 12, shader_location: 1, format: gpu_device::VertexFormat::Float32x3 },
            gpu_device::VertexAttribute { offset: 24, shader_location: 2, format: gpu_device::VertexFormat::Float32x4 },
            gpu_device::VertexAttribute { offset: 40, shader_location: 9, format: gpu_device::VertexFormat::Float32x2 },
        ],
    }
}

/// 🪞 Instance buffer layout of the world3d mesh instance stream.
pub(super) fn world_instance_layout() -> gpu_device::VertexBufferLayout<'static> {
    gpu_device::VertexBufferLayout {
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
    }
}

/// 🏷 Vertex buffer layout of the authored surface vertex stream.
pub(super) fn world_surface_vertex_layout() -> gpu_device::VertexBufferLayout<'static> {
    gpu_device::VertexBufferLayout {array_stride:size_of::<World3dVertex>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Vertex,attributes:&[
        gpu_device::VertexAttribute{offset:0,shader_location:0,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:12,shader_location:1,format:gpu_device::VertexFormat::Float32x3},gpu_device::VertexAttribute{offset:24,shader_location:2,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:40,shader_location:9,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:56,shader_location:11,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:72,shader_location:12,format:gpu_device::VertexFormat::Float32x2},gpu_device::VertexAttribute{offset:80,shader_location:13,format:gpu_device::VertexFormat::Float32x4},]}
}

/// 🔗 Instance buffer layout of the authored surface instance stream.
pub(super) fn world_surface_instance_layout() -> gpu_device::VertexBufferLayout<'static> {
    gpu_device::VertexBufferLayout{array_stride:size_of::<World3dGpuInstance>() as gpu_device::BufferAddress,step_mode:gpu_device::VertexStepMode::Instance,attributes:&[
        gpu_device::VertexAttribute{offset:0,shader_location:3,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:16,shader_location:4,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:32,shader_location:5,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:48,shader_location:6,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:64,shader_location:7,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:80,shader_location:8,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:96,shader_location:10,format:gpu_device::VertexFormat::Float32x4},gpu_device::VertexAttribute{offset:112,shader_location:15,format:gpu_device::VertexFormat::Float32x4},]}
}

/// 🎉 Instance buffer layout of the celebration instance stream.
pub(super) fn celebration_instance_layout() -> gpu_device::VertexBufferLayout<'static> {
    gpu_device::VertexBufferLayout {
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
    }
}

/// 🧱 Depth stencil state of the material passes: depth write or the depth biased read only variant.
pub(super) fn material_depth(write: bool) -> gpu_device::DepthStencilState {
    gpu_device::DepthStencilState {
        format: gpu_device::TextureFormat::Depth24PlusStencil8,
        depth_write_enabled: Some(write),
        depth_compare: Some(gpu_device::CompareFunction::LessEqual),
        stencil: content_stencil_state(),
        bias: if write { gpu_device::DepthBiasState::default() } else { gpu_device::DepthBiasState { constant: -2, slope_scale: -1.0, clamp: 0.0 } },
    }
}
