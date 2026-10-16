//! 🖌️ Borrowed passes and actual completion keep the encoder and issuing session alive.
use super::*;
pub struct CommandEncoder{pub(super) raw:wgpu::CommandEncoder,pub(super) session:Session}
#[derive(Debug)]
pub struct CommandBuffer{raw:wgpu::CommandBuffer,session:Session}
pub struct RenderPass<'a>{raw:wgpu::RenderPass<'a>,session:Session}
pub struct Submission{session:Session,completed:Arc<AtomicBool>}
impl Submission{
    pub fn complete(&self)->bool{self.completed.load(Ordering::Acquire)}
    pub fn poll(&self)->Result<bool,ResourceFault>{self.session.0.device.poll(wgpu::PollType::Poll).map_err(|_|ResourceFault::Backend("submission poll"))?;Ok(self.complete())}
}
impl Queue{
    pub fn submit(&self,command:CommandBuffer)->Result<Submission,OwnedFailure<CommandBuffer>>{
        if let Err(fault)=self.session.check(&command.session){return Err(OwnedFailure::Returned{fault,resource:command});}
        self.session.0.queue.submit(Some(command.raw));
        let completed=Arc::new(AtomicBool::new(false));let witness=completed.clone();
        self.session.0.queue.on_submitted_work_done(move||witness.store(true,Ordering::Release));
        Ok(Submission{session:self.session.clone(),completed})
    }
}
impl CommandEncoder{
    pub fn finish(self)->CommandBuffer{CommandBuffer{raw:self.raw.finish(),session:self.session}}
    pub fn try_begin_render_pass<'a>(&'a mut self,descriptor:&RenderPassDescriptor<'_>)->Result<RenderPass<'a>,ResourceFault>{
        if descriptor.color_attachments.len()>8||descriptor.timestamp_writes.is_some()||descriptor.occlusion_query_set.is_some(){return Err(ResourceFault::InvalidDescriptor("bounded render pass"));}
        for attachment in descriptor.color_attachments.iter().flatten(){self.session.check(&attachment.view.session)?;if let Some(resolve)=attachment.resolve_target{self.session.check(&resolve.session)?;}}
        if let Some(attachment)=&descriptor.depth_stencil_attachment{self.session.check(&attachment.view.session)?;}
        let colors:Vec<_>=descriptor.color_attachments.iter().map(|attachment|attachment.as_ref().map(|attachment|wgpu::RenderPassColorAttachment{view:&attachment.view.raw,depth_slice:attachment.depth_slice,resolve_target:attachment.resolve_target.map(|target|&target.raw),ops:conversion::operations(attachment.ops,conversion::color)})).collect();
        let depth=descriptor.depth_stencil_attachment.as_ref().map(|attachment|wgpu::RenderPassDepthStencilAttachment{view:&attachment.view.raw,depth_ops:attachment.depth_ops.map(|ops|conversion::operations(ops,|value|value)),stencil_ops:attachment.stencil_ops.map(|ops|conversion::operations(ops,|value|value))});
        Ok(RenderPass{raw:self.raw.begin_render_pass(&wgpu::RenderPassDescriptor{label:descriptor.label,color_attachments:&colors,depth_stencil_attachment:depth,timestamp_writes:None,occlusion_query_set:None,multiview_mask:descriptor.multiview_mask}),session:self.session.clone()})
    }
    pub fn begin_render_pass<'a>(&'a mut self,descriptor:&RenderPassDescriptor<'_>)->RenderPass<'a>{self.try_begin_render_pass(descriptor).expect("same-session render attachments")}
    pub fn copy_texture_to_texture(&mut self,source:TexelCopyTextureInfo<'_>,target:TexelCopyTextureInfo<'_>,extent:Extent3d){self.session.check(&source.texture.session).expect("same-session copy source");self.session.check(&target.texture.session).expect("same-session copy target");self.raw.copy_texture_to_texture(conversion::texture_copy(source),conversion::texture_copy(target),conversion::extent(extent));}
    pub fn copy_texture_to_buffer(&mut self,source:TexelCopyTextureInfo<'_>,target:TexelCopyBufferInfo<'_>,extent:Extent3d){self.session.check(&source.texture.session).expect("same-session readback source");self.session.check(&target.buffer.session).expect("same-session readback target");self.raw.copy_texture_to_buffer(conversion::texture_copy(source),wgpu::TexelCopyBufferInfo{buffer:&target.buffer.raw,layout:conversion::copy_layout(target.layout)},conversion::extent(extent));}
}
impl RenderPass<'_>{
    pub fn set_pipeline(&mut self,pipeline:&RenderPipeline){self.session.check(&pipeline.session).expect("same-session pass pipeline");self.raw.set_pipeline(&pipeline.raw);}
    pub fn set_bind_group(&mut self,index:u32,group:&BindGroup,offsets:&[u32]){self.session.check(&group.session).expect("same-session pass bindings");assert!(offsets.len()<=32,"bounded dynamic offsets");self.raw.set_bind_group(index,&group.raw,offsets);}
    pub fn set_vertex_buffer(&mut self,index:u32,buffer:BufferSlice<'_>){self.session.check(&buffer.buffer.session).expect("same-session vertex buffer");self.raw.set_vertex_buffer(index,buffer.raw());}
    pub fn set_index_buffer(&mut self,buffer:BufferSlice<'_>,format:IndexFormat){self.session.check(&buffer.buffer.session).expect("same-session index buffer");self.raw.set_index_buffer(buffer.raw(),conversion::index_format(format));}
    pub fn set_scissor_rect(&mut self,x:u32,y:u32,width:u32,height:u32){self.raw.set_scissor_rect(x,y,width,height);}
    pub fn set_viewport(&mut self,x:f32,y:f32,width:f32,height:f32,near:f32,far:f32){self.raw.set_viewport(x,y,width,height,near,far);}
    pub fn set_stencil_reference(&mut self,value:u32){self.raw.set_stencil_reference(value);}
    pub fn draw(&mut self,vertices:std::ops::Range<u32>,instances:std::ops::Range<u32>){self.raw.draw(vertices,instances);}
    pub fn draw_indexed(&mut self,indices:std::ops::Range<u32>,base:i32,instances:std::ops::Range<u32>){self.raw.draw_indexed(indices,base,instances);}
}
