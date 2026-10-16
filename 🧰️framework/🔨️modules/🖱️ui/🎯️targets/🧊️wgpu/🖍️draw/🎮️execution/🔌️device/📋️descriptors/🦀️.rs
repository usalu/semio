//! 📋️ Concrete first-party resource specialization of the neutral borrowed work contract.
use super::*;
pub type ShaderSource<'a>=semio_framework_ui_render_device::ShaderSource<'a>;
pub type ShaderModuleDescriptor<'a>=semio_framework_ui_render_device::ShaderModuleDescriptor<'a>;
pub type BufferBinding<'a>=semio_framework_ui_render_device::BufferBinding<'a,Buffer>;
pub type BindingResource<'a>=semio_framework_ui_render_device::BindingResource<'a,Buffer,TextureView,Sampler>;
pub type BindGroupEntry<'a>=semio_framework_ui_render_device::BindGroupEntry<'a,Buffer,TextureView,Sampler>;
pub type BindGroupDescriptor<'a>=semio_framework_ui_render_device::BindGroupDescriptor<'a,BindGroupLayout,Buffer,TextureView,Sampler>;
pub type PipelineLayoutDescriptor<'a>=semio_framework_ui_render_device::PipelineLayoutDescriptor<'a,BindGroupLayout>;
pub type VertexState<'a>=semio_framework_ui_render_device::VertexState<'a,ShaderModule>;
pub type FragmentState<'a>=semio_framework_ui_render_device::FragmentState<'a,ShaderModule>;
pub type RenderPipelineDescriptor<'a>=semio_framework_ui_render_device::RenderPipelineDescriptor<'a,PipelineLayout,ShaderModule>;
pub type RenderPassColorAttachment<'a>=semio_framework_ui_render_device::RenderPassColorAttachment<'a,TextureView>;
pub type RenderPassDepthStencilAttachment<'a>=semio_framework_ui_render_device::RenderPassDepthStencilAttachment<'a,TextureView>;
pub type RenderPassDescriptor<'a>=semio_framework_ui_render_device::RenderPassDescriptor<'a,TextureView>;
pub type TexelCopyTextureInfo<'a>=semio_framework_ui_render_device::TexelCopyTextureInfo<'a,Texture>;
pub type TexelCopyBufferInfo<'a>=semio_framework_ui_render_device::TexelCopyBufferInfo<'a,Buffer>;
