//! 🔒️ Actual backend resources retain their issuing device and reject foreign sessions before execution.
use std::sync::{Arc,atomic::{AtomicBool,Ordering}};
pub use semio_framework_ui_render_device::*;

#[path="🗺️conversion/🦀️.rs"]
mod conversion;
#[path="📋️descriptors/🦀️.rs"]
mod descriptors;
pub use descriptors::{ShaderSource,ShaderModuleDescriptor,BufferBinding,BindingResource,BindGroupEntry,BindGroupDescriptor,PipelineLayoutDescriptor,VertexState,FragmentState,RenderPipelineDescriptor,RenderPassColorAttachment,RenderPassDepthStencilAttachment,RenderPassDescriptor,TexelCopyTextureInfo,TexelCopyBufferInfo};
#[path="🖌️commands/🦀️.rs"]
mod commands;
pub use commands::{CommandEncoder,CommandBuffer,RenderPass,Submission};
#[path="📥️platform/🦀️.rs"]
pub mod platform;
pub use platform::{Surface,SurfaceTexture,CurrentSurfaceTexture};

#[derive(Debug)]
struct SessionRoots{device:wgpu::Device,queue:wgpu::Queue}
#[derive(Clone,Debug)]
struct Session(Arc<SessionRoots>);
impl Session{
    fn check(&self,other:&Self)->Result<(),ResourceFault>{if Arc::ptr_eq(&self.0,&other.0){Ok(())}else{Err(ResourceFault::ForeignSession)}}
}
#[derive(Clone,Debug)]
pub struct Device{session:Session}
#[derive(Clone,Debug)]
pub struct Queue{session:Session}
#[derive(Clone,Copy,Debug)]
pub struct DeviceLimits{pub max_buffer_size:u64,pub max_texture_dimension_2d:u32}
pub enum ErrorFilter{Validation}
pub struct ErrorScopeGuard{raw:wgpu::ErrorScopeGuard,session:Session}
pub type Error=String;
impl ErrorScopeGuard{pub async fn pop(self)->Option<Error>{let error=self.raw.pop().await;drop(self.session);error.map(|error|error.to_string())}}
macro_rules! resource{
    ($name:ident,$raw:ty)=>{#[derive(Clone,Debug)]pub struct $name{raw:$raw,session:Session}};
}
resource!(Buffer,wgpu::Buffer);
resource!(Texture,wgpu::Texture);
resource!(TextureView,wgpu::TextureView);
resource!(Sampler,wgpu::Sampler);
resource!(BindGroupLayout,wgpu::BindGroupLayout);
resource!(BindGroup,wgpu::BindGroup);
resource!(PipelineLayout,wgpu::PipelineLayout);
resource!(RenderPipeline,wgpu::RenderPipeline);
resource!(ShaderModule,wgpu::ShaderModule);

impl Device{
    fn issued(device:wgpu::Device,queue:wgpu::Queue)->(Self,Queue){let session=Session(Arc::new(SessionRoots{device,queue}));(Self{session:session.clone()},Queue{session})}
    pub fn limits(&self)->DeviceLimits{let limits=self.session.0.device.limits();DeviceLimits{max_buffer_size:limits.max_buffer_size,max_texture_dimension_2d:limits.max_texture_dimension_2d}}
    pub fn same_session(&self,other:&Self)->bool{self.session.check(&other.session).is_ok()}
    pub fn push_error_scope(&self,_filter:ErrorFilter)->ErrorScopeGuard{ErrorScopeGuard{raw:self.session.0.device.push_error_scope(wgpu::ErrorFilter::Validation),session:self.session.clone()}}
    pub fn poll(&self,_mode:PollType)->Result<(),ResourceFault>{self.session.0.device.poll(wgpu::PollType::Poll).map(|_|()).map_err(|_|ResourceFault::Backend("device poll"))}
    pub async fn headless()->Result<(Self,Queue),String>{platform::headless().await}
    pub fn create_buffer(&self,descriptor:&BufferDescriptor<'_>)->Buffer{self.buffer(descriptor).expect("validated device buffer descriptor")}
    pub fn create_texture(&self,descriptor:&TextureDescriptor<'_>)->Texture{self.texture(descriptor).expect("validated device texture descriptor")}
    pub fn create_sampler(&self,descriptor:&SamplerDescriptor<'_>)->Sampler{
        let raw=self.session.0.device.create_sampler(&wgpu::SamplerDescriptor{label:descriptor.label,address_mode_u:conversion::address_mode(descriptor.address_mode_u),address_mode_v:conversion::address_mode(descriptor.address_mode_v),address_mode_w:conversion::address_mode(descriptor.address_mode_w),mag_filter:conversion::filter_mode(descriptor.mag_filter),min_filter:conversion::filter_mode(descriptor.min_filter),mipmap_filter:conversion::mipmap_filter_mode(descriptor.mipmap_filter),lod_min_clamp:descriptor.lod_min_clamp,lod_max_clamp:descriptor.lod_max_clamp,compare:descriptor.compare.map(conversion::compare_function),anisotropy_clamp:descriptor.anisotropy_clamp,..Default::default()});
        Sampler{raw,session:self.session.clone()}
    }
    pub fn create_bind_group_layout(&self,descriptor:&BindGroupLayoutDescriptor<'_>)->BindGroupLayout{
        assert!(descriptor.entries.len()<=32,"bounded binding layout");
        let entries:Vec<_>=descriptor.entries.iter().map(|entry|wgpu::BindGroupLayoutEntry{binding:entry.binding,visibility:conversion::shader_stages(entry.visibility),ty:conversion::binding_type(entry.ty),count:entry.count}).collect();
        BindGroupLayout{raw:self.session.0.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:descriptor.label,entries:&entries}),session:self.session.clone()}
    }
    pub fn try_create_bind_group(&self,descriptor:&BindGroupDescriptor<'_>)->Result<BindGroup,ResourceFault>{
        self.session.check(&descriptor.layout.session)?;
        if descriptor.entries.len()>32{return Err(ResourceFault::InvalidDescriptor("bounded binding entries"));}
        for entry in descriptor.entries{match &entry.resource{BindingResource::Buffer(buffer)=>self.session.check(&buffer.buffer.session)?,BindingResource::TextureView(view)=>self.session.check(&view.session)?,BindingResource::Sampler(sampler)=>self.session.check(&sampler.session)?}}
        let entries:Vec<_>=descriptor.entries.iter().map(|entry|wgpu::BindGroupEntry{binding:entry.binding,resource:match &entry.resource{BindingResource::Buffer(buffer)=>wgpu::BindingResource::Buffer(wgpu::BufferBinding{buffer:&buffer.buffer.raw,offset:buffer.offset,size:buffer.size}),BindingResource::TextureView(view)=>wgpu::BindingResource::TextureView(&view.raw),BindingResource::Sampler(sampler)=>wgpu::BindingResource::Sampler(&sampler.raw)}}).collect();
        Ok(BindGroup{raw:self.session.0.device.create_bind_group(&wgpu::BindGroupDescriptor{label:descriptor.label,layout:&descriptor.layout.raw,entries:&entries}),session:self.session.clone()})
    }
    pub fn create_bind_group(&self,descriptor:&BindGroupDescriptor<'_>)->BindGroup{self.try_create_bind_group(descriptor).expect("same-session bind group resources")}
    pub fn create_shader_module(&self,descriptor:ShaderModuleDescriptor<'_>)->ShaderModule{
        let ShaderSource::Wgsl(source)=descriptor.source;
        assert!(source.len()<=262144,"bounded shader program");
        ShaderModule{raw:self.session.0.device.create_shader_module(wgpu::ShaderModuleDescriptor{label:descriptor.label,source:wgpu::ShaderSource::Wgsl(source)}),session:self.session.clone()}
    }
    pub fn create_pipeline_layout(&self,descriptor:&PipelineLayoutDescriptor<'_>)->PipelineLayout{
        assert!(descriptor.bind_group_layouts.len()<=8&&descriptor.immediate_size==0,"bounded pipeline layout");
        for layout in descriptor.bind_group_layouts.iter().flatten(){self.session.check(&layout.session).expect("same-session pipeline layout");}
        let layouts:Vec<_>=descriptor.bind_group_layouts.iter().map(|layout|layout.map(|layout|&layout.raw)).collect();
        PipelineLayout{raw:self.session.0.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:descriptor.label,bind_group_layouts:&layouts,immediate_size:descriptor.immediate_size}),session:self.session.clone()}
    }
    pub fn create_render_pipeline(&self,descriptor:&RenderPipelineDescriptor<'_>)->RenderPipeline{
        self.session.check(&descriptor.vertex.module.session).expect("same-session vertex shader");
        if let Some(fragment)=&descriptor.fragment{self.session.check(&fragment.module.session).expect("same-session fragment shader");}
        if let Some(layout)=descriptor.layout{self.session.check(&layout.session).expect("same-session render pipeline layout");}
        assert!(descriptor.vertex.buffers.len()<=16&&descriptor.vertex.buffers.iter().all(|buffer|buffer.attributes.len()<=32)&&descriptor.fragment.as_ref().is_none_or(|fragment|fragment.targets.len()<=8)&&descriptor.cache.is_none(),"bounded render pipeline descriptor");
        let attributes:Vec<Vec<_>>=descriptor.vertex.buffers.iter().map(|buffer|buffer.attributes.iter().map(|attribute|wgpu::VertexAttribute{format:conversion::vertex_format(attribute.format),offset:attribute.offset,shader_location:attribute.shader_location}).collect()).collect();
        let buffers:Vec<_>=descriptor.vertex.buffers.iter().zip(&attributes).map(|(buffer,attributes)|wgpu::VertexBufferLayout{array_stride:buffer.array_stride,step_mode:conversion::vertex_step_mode(buffer.step_mode),attributes}).collect();
        let targets:Vec<_>=descriptor.fragment.as_ref().map(|fragment|fragment.targets.iter().map(|target|target.map(|target|wgpu::ColorTargetState{format:conversion::texture_format(target.format),blend:target.blend.map(conversion::blend_state),write_mask:conversion::color_writes(target.write_mask)})).collect()).unwrap_or_default();
        let fragment=descriptor.fragment.as_ref().map(|fragment|wgpu::FragmentState{module:&fragment.module.raw,entry_point:fragment.entry_point,compilation_options:Default::default(),targets:&targets});
        let raw=self.session.0.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:descriptor.label,layout:descriptor.layout.map(|layout|&layout.raw),vertex:wgpu::VertexState{module:&descriptor.vertex.module.raw,entry_point:descriptor.vertex.entry_point,compilation_options:Default::default(),buffers:&buffers},primitive:wgpu::PrimitiveState{topology:conversion::primitive_topology(descriptor.primitive.topology),cull_mode:descriptor.primitive.cull_mode.map(conversion::face),..Default::default()},depth_stencil:descriptor.depth_stencil.as_ref().map(conversion::depth),multisample:wgpu::MultisampleState{count:descriptor.multisample.count,mask:descriptor.multisample.mask,alpha_to_coverage_enabled:descriptor.multisample.alpha_to_coverage_enabled},fragment,multiview_mask:descriptor.multiview_mask,cache:None});
        RenderPipeline{raw,session:self.session.clone()}
    }
    pub fn create_command_encoder(&self,descriptor:&CommandEncoderDescriptor<'_>)->CommandEncoder{CommandEncoder{raw:self.session.0.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label:descriptor.label}),session:self.session.clone()}}
}
impl RenderDevice for Device{
    type Buffer=Buffer;
    type Texture=Texture;
    fn buffer(&self,descriptor:&BufferDescriptor<'_>)->Result<Buffer,ResourceFault>{
        ResourceSpec{kind:ResourceKind::Buffer,bytes:descriptor.size,width:1,height:1,layers:1,mips:1,samples:1}.validate(ResourceLimits{bytes:self.limits().max_buffer_size,dimension:self.limits().max_texture_dimension_2d})?;
        Ok(Buffer{raw:self.session.0.device.create_buffer(&wgpu::BufferDescriptor{label:descriptor.label,size:descriptor.size,usage:conversion::buffer_usages(descriptor.usage),mapped_at_creation:descriptor.mapped_at_creation}),session:self.session.clone()})
    }
    fn texture(&self,descriptor:&TextureDescriptor<'_>)->Result<Texture,ResourceFault>{
        if descriptor.size.width==0||descriptor.size.height==0||descriptor.size.width>self.limits().max_texture_dimension_2d||descriptor.size.height>self.limits().max_texture_dimension_2d||descriptor.size.depth_or_array_layers!=1||descriptor.mip_level_count==0||descriptor.mip_level_count>u32::BITS-descriptor.size.width.max(descriptor.size.height).leading_zeros()||!matches!(descriptor.sample_count,1|4)||descriptor.view_formats.len()>2{return Err(ResourceFault::InvalidDescriptor("bounded texture descriptor"));}
        let formats:Vec<_>=descriptor.view_formats.iter().copied().map(conversion::texture_format).collect();
        Ok(Texture{raw:self.session.0.device.create_texture(&wgpu::TextureDescriptor{label:descriptor.label,size:conversion::extent(descriptor.size),mip_level_count:descriptor.mip_level_count,sample_count:descriptor.sample_count,dimension:conversion::texture_dimension(descriptor.dimension),format:conversion::texture_format(descriptor.format),usage:conversion::texture_usages(descriptor.usage),view_formats:&formats}),session:self.session.clone()})
    }
}
impl Texture{
    pub fn create_view(&self,descriptor:&TextureViewDescriptor<'_>)->TextureView{TextureView{raw:self.raw.create_view(&wgpu::TextureViewDescriptor{label:descriptor.label,format:descriptor.format.map(conversion::texture_format),dimension:descriptor.dimension.map(conversion::texture_view_dimension),base_mip_level:descriptor.base_mip_level,mip_level_count:descriptor.mip_level_count,..Default::default()}),session:self.session.clone()}}
    pub fn destroy(&self){self.raw.destroy();}
    pub fn format(&self)->TextureFormat{conversion::format(self.raw.format()).expect("issued supported texture format")}
}
#[derive(Clone,Copy,Debug)]
pub struct BufferSlice<'a>{buffer:&'a Buffer,start:u64,end:u64}
impl Buffer{
    pub fn size(&self)->u64{self.raw.size()}
    pub fn destroy(&self){self.raw.destroy();}
    pub fn unmap(&self){self.raw.unmap();}
    pub fn slice(&self,bounds:impl std::ops::RangeBounds<u64>)->BufferSlice<'_>{use std::ops::Bound;let start=match bounds.start_bound(){Bound::Included(value)=>*value,Bound::Excluded(value)=>value.checked_add(1).expect("buffer slice start overflow"),Bound::Unbounded=>0};let end=match bounds.end_bound(){Bound::Included(value)=>value.checked_add(1).expect("buffer slice end overflow"),Bound::Excluded(value)=>*value,Bound::Unbounded=>self.size()};assert!(start<=end&&end<=self.size(),"bounded buffer slice");BufferSlice{buffer:self,start,end}}
    pub fn as_entire_binding(&self)->BindingResource<'_>{BindingResource::Buffer(BufferBinding{buffer:self,offset:0,size:None})}
}
pub struct BufferView{raw:wgpu::BufferView}
impl std::ops::Deref for BufferView{type Target=[u8];fn deref(&self)->&Self::Target{&self.raw}}
impl BufferSlice<'_>{
    /// 🔪️ Narrows this slice to a sub-range relative to its own start, never beyond its own end.
    pub fn slice(&self,bounds:impl std::ops::RangeBounds<u64>)->BufferSlice<'_>{use std::ops::Bound;let length=self.end-self.start;let start=match bounds.start_bound(){Bound::Included(value)=>*value,Bound::Excluded(value)=>value.checked_add(1).expect("buffer slice start overflow"),Bound::Unbounded=>0};let end=match bounds.end_bound(){Bound::Included(value)=>value.checked_add(1).expect("buffer slice end overflow"),Bound::Excluded(value)=>*value,Bound::Unbounded=>length};assert!(start<=end&&end<=length,"bounded buffer slice");BufferSlice{buffer:self.buffer,start:self.start+start,end:self.start+end}}
    fn raw(&self)->wgpu::BufferSlice<'_>{self.buffer.raw.slice(self.start..self.end)}
    pub fn map_async(&self,_mode:MapMode,completion:impl FnOnce(Result<(),ResourceFault>)+Send+'static){self.raw().map_async(wgpu::MapMode::Read,move|result|completion(result.map_err(|_|ResourceFault::Backend("buffer mapping"))));}
    pub fn get_mapped_range(&self)->BufferView{BufferView{raw:self.raw().get_mapped_range()}}
}
impl Queue{
    pub fn write_buffer(&self,buffer:&Buffer,offset:u64,bytes:&[u8]){self.session.check(&buffer.session).expect("same-session buffer write");assert!(offset.checked_add(bytes.len()as u64).is_some_and(|end|end<=buffer.size()),"bounded buffer write");self.session.0.queue.write_buffer(&buffer.raw,offset,bytes);}
    pub fn write_texture(&self,target:TexelCopyTextureInfo<'_>,bytes:&[u8],layout:TexelCopyBufferLayout,extent:Extent3d){self.session.check(&target.texture.session).expect("same-session texture write");self.session.0.queue.write_texture(conversion::texture_copy(target),bytes,conversion::copy_layout(layout),conversion::extent(extent));}
}
pub mod util{
    use super::*;
    pub struct BufferInitDescriptor<'a>{pub label:Option<&'a str>,pub contents:&'a[u8],pub usage:BufferUsages}
    pub trait DeviceExt{fn create_buffer_init(&self,descriptor:&BufferInitDescriptor<'_>)->Buffer;}
    impl DeviceExt for Device{fn create_buffer_init(&self,descriptor:&BufferInitDescriptor<'_>)->Buffer{use wgpu::util::DeviceExt;assert!(descriptor.contents.len()as u64<=self.limits().max_buffer_size,"bounded initialized buffer");Buffer{raw:self.session.0.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:descriptor.label,contents:descriptor.contents,usage:conversion::buffer_usages(descriptor.usage)}),session:self.session.clone()}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
