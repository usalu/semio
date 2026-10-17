use super::*;
fn devices()->Option<(Device,Queue,Device,Queue)>{
    match semio_framework_async::block_on(async{let(a,aq)=Device::headless().await?;let(b,bq)=Device::headless().await?;Ok::<_,String>((a,aq,b,bq))}){Ok(devices)=>Some(devices),Err(fault)=>{None}}
}
#[test]
fn foreign_actual_binding_is_refused_before_driver_access_and_original_resources_remain_owned(){
    let Some((a,_aq,b,_bq))=devices()else{return};
    assert!(!a.same_session(&b));
    let layout=a.create_bind_group_layout(&BindGroupLayoutDescriptor{label:Some("session-law"),entries:&[BindGroupLayoutEntry{binding:0,visibility:ShaderStages::VERTEX,ty:BindingType::Buffer{ty:BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:None},count:None}]});
    let original=b.create_buffer(&BufferDescriptor{label:Some("foreign-original"),size:16,usage:BufferUsages::UNIFORM,mapped_at_creation:false});
    let roots=Arc::clone(&original.session.0);
    let result=a.try_create_bind_group(&BindGroupDescriptor{label:Some("foreign-binding"),layout:&layout,entries:&[BindGroupEntry{binding:0,resource:original.as_entire_binding()}]});
    assert!(matches!(result,Err(ResourceFault::ForeignSession)));
    assert!(Arc::ptr_eq(&roots,&original.session.0));
    assert_eq!(original.size(),16);
}
#[test]
fn foreign_submission_returns_exact_actual_command_and_completion_requires_callback(){
    let Some((a,aq,b,bq))=devices()else{return};
    let command=b.create_command_encoder(&CommandEncoderDescriptor{label:Some("original-owned-command")}).finish();
    let returned=match aq.submit(command){Err(OwnedFailure::Returned{fault:ResourceFault::ForeignSession,resource})=>resource,_=>panic!("foreign actual command must return before submission")};
    let submission=bq.submit(returned).unwrap_or_else(|_|panic!("issuer submits exact returned command"));
    for _ in 0..100_000{if submission.poll().unwrap(){break}std::thread::yield_now();}
    assert!(submission.complete(),"actual submitted-work callback must settle");
    assert!(!a.same_session(&b));
}
#[test]
fn foreign_actual_render_attachment_is_refused_before_a_pass_is_born(){
    let Some((a,_aq,b,_bq))=devices()else{return};
    let texture=b.create_texture(&TextureDescriptor{label:Some("foreign-target"),size:Extent3d{width:1,height:1,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:TextureDimension::D2,format:TextureFormat::Rgba8Unorm,usage:TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});
    let view=texture.create_view(&TextureViewDescriptor::default());
    let mut encoder=a.create_command_encoder(&CommandEncoderDescriptor::default());
    assert!(matches!(encoder.try_begin_render_pass(&RenderPassDescriptor{label:None,color_attachments:&[Some(RenderPassColorAttachment{view:&view,depth_slice:None,resolve_target:None,ops:Operations{load:LoadOp::Clear(Color::TRANSPARENT),store:StoreOp::Store}})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None}),Err(ResourceFault::ForeignSession)));
    assert!(Arc::ptr_eq(&texture.session.0,&view.session.0));
}
