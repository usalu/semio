use super::*;

#[test]
fn authored_glyph_runs_transform_the_actual_font_atlas_quads() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🖍️draw/🏷️types/↗️affine/🧫️fixtures/🔣️.json")).unwrap();
    let mut atlas=FontAtlas::builtin();
    let mut plain=DrawList::default();
    draw_text_face_on(&mut plain,&mut atlas,TextFace::Sans,"AV Ä",2.0,24.0,18.0,Rgba::new(0.2,0.4,0.6,0.8));
    let plain:Vec<_>=plain.layers.iter().flat_map(|layer|layer.ui_instances.iter()).copied().collect();
    assert!(!plain.is_empty());
    for sample in fixture["cases"].as_array().unwrap() {
        let matrix=std::array::from_fn(|index|sample["matrix"][index].as_f64().unwrap() as f32);
        let oracle=tiny_skia::Transform::from_row(matrix[0],matrix[1],matrix[2],matrix[3],matrix[4],matrix[5]);
        let mut transformed=DrawList::default();
        draw_text_face_affine_on(&mut transformed,&mut atlas,TextFace::Sans,"AV Ä",(2.0,24.0),18.0,Rgba::new(0.2,0.4,0.6,0.8),matrix);
        let transformed:Vec<_>=transformed.layers.iter().flat_map(|layer|layer.ui_instances.iter()).copied().collect();
        assert_eq!(plain.len(),transformed.len());
        for (source,target) in plain.iter().zip(transformed.iter()) {
            assert_eq!(target.params[2],crate::wgpu::draw_types::KIND_AFFINE_GLYPH);
            assert_eq!(source.color,target.color);
            assert_eq!(source.uv_rect,target.uv_rect);
            for [x,y] in [[0.0,0.0],[1.0,0.0],[1.0,1.0],[0.0,1.0]] {
                let mut expected=tiny_skia::Point::from_xy(source.rect[0]+x*source.rect[2],source.rect[1]+y*source.rect[3]);
                oracle.map_point(&mut expected);
                let actual=[target.rect[0]+x*target.rect[2]+y*target.params[0],target.rect[1]+x*target.rect[3]+y*target.params[1]];
                assert!((actual[0]-expected.x).abs()<0.0001);
                assert!((actual[1]-expected.y).abs()<0.0001);
            }
        }
    }
    eprintln!("[DEBUG] Actual native font atlas affine quads matched Tiny Skia for five transforms");
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn authored_glyph_shader_accepts_complete_affine_basis() {
    let (device,_queue)=semio_framework_async::block_on(async {
        let instance=wgpu::Instance::new(wgpu::InstanceDescriptor {backends:wgpu::Backends::PRIMARY,..wgpu::InstanceDescriptor::new_without_display_handle()});
        let adapter=instance.request_adapter(&wgpu::RequestAdapterOptions {power_preference:wgpu::PowerPreference::HighPerformance,compatible_surface:None,force_fallback_adapter:false}).await.expect("affine shader requires a GPU adapter");
        adapter.request_device(&wgpu::DeviceDescriptor {label:Some("authored_affine_shader"),required_features:wgpu::Features::empty(),required_limits:wgpu::Limits::default(),memory_hints:Default::default(),trace:wgpu::Trace::Off,experimental_features:Default::default()}).await.expect("affine shader device")
    });
    let validation=device.push_error_scope(wgpu::ErrorFilter::Validation);
    let _shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("authored_affine_ui_shader"),source:wgpu::ShaderSource::Wgsl(crate::wgpu::shaders::UI_SHADER.into())});
    device.poll(wgpu::PollType::Poll).unwrap();
    assert!(semio_framework_async::block_on(validation.pop()).is_none());
    eprintln!("[DEBUG] Actual GPU validated the production affine glyph/image shader");
}

#[cfg(not(target_arch="wasm32"))]
#[test]
fn authored_glyph_image_shader_paints_the_actual_affine_quad() {
    use wgpu::util::DeviceExt;
    let (device,queue)=semio_framework_async::block_on(async {
        let instance=wgpu::Instance::new(wgpu::InstanceDescriptor {backends:wgpu::Backends::PRIMARY,..wgpu::InstanceDescriptor::new_without_display_handle()});
        let adapter=instance.request_adapter(&wgpu::RequestAdapterOptions {power_preference:wgpu::PowerPreference::HighPerformance,compatible_surface:None,force_fallback_adapter:false}).await.expect("affine framebuffer requires a GPU adapter");
        adapter.request_device(&wgpu::DeviceDescriptor {label:Some("authored_affine_framebuffer"),required_features:wgpu::Features::empty(),required_limits:wgpu::Limits::default(),memory_hints:Default::default(),trace:wgpu::Trace::Off,experimental_features:Default::default()}).await.expect("affine framebuffer device")
    });
    let validation=device.push_error_scope(wgpu::ErrorFilter::Validation);
    let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("authored_affine_framebuffer_shader"),source:wgpu::ShaderSource::Wgsl(crate::wgpu::shaders::UI_SHADER.into())});
    let globals=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("affine_globals"),contents:bytemuck::cast_slice(&[32.0_f32,32.0,0.0,0.0]),usage:wgpu::BufferUsages::UNIFORM});
    let texture=device.create_texture(&wgpu::TextureDescriptor {label:Some("affine_source"),size:wgpu::Extent3d {width:1,height:1,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::COPY_DST,view_formats:&[]});
    queue.write_texture(wgpu::TexelCopyTextureInfo {texture:&texture,mip_level:0,origin:wgpu::Origin3d::ZERO,aspect:wgpu::TextureAspect::All},&[255,0,0,255],wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(4),rows_per_image:Some(1)},wgpu::Extent3d {width:1,height:1,depth_or_array_layers:1});
    let view=texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler=device.create_sampler(&wgpu::SamplerDescriptor::default());
    let layout=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {label:Some("affine_layout"),entries:&[
        wgpu::BindGroupLayoutEntry {binding:0,visibility:wgpu::ShaderStages::VERTEX|wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Buffer {ty:wgpu::BufferBindingType::Uniform,has_dynamic_offset:false,min_binding_size:None},count:None},
        wgpu::BindGroupLayoutEntry {binding:1,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture {sample_type:wgpu::TextureSampleType::Float {filterable:true},view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},
        wgpu::BindGroupLayoutEntry {binding:2,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None},
        wgpu::BindGroupLayoutEntry {binding:3,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Texture {sample_type:wgpu::TextureSampleType::Float {filterable:true},view_dimension:wgpu::TextureViewDimension::D2,multisampled:false},count:None},
        wgpu::BindGroupLayoutEntry {binding:4,visibility:wgpu::ShaderStages::FRAGMENT,ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),count:None},
    ]});
    let group=device.create_bind_group(&wgpu::BindGroupDescriptor {label:Some("affine_group"),layout:&layout,entries:&[
        wgpu::BindGroupEntry {binding:0,resource:globals.as_entire_binding()},
        wgpu::BindGroupEntry {binding:1,resource:wgpu::BindingResource::TextureView(&view)},
        wgpu::BindGroupEntry {binding:2,resource:wgpu::BindingResource::Sampler(&sampler)},
        wgpu::BindGroupEntry {binding:3,resource:wgpu::BindingResource::TextureView(&view)},
        wgpu::BindGroupEntry {binding:4,resource:wgpu::BindingResource::Sampler(&sampler)},
    ]});
    let pipeline_layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {label:Some("affine_pipeline_layout"),bind_group_layouts:&[Some(&layout)],immediate_size:0});
    let pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {label:Some("affine_pipeline"),layout:Some(&pipeline_layout),vertex:wgpu::VertexState {module:&shader,entry_point:Some("vs_main"),compilation_options:Default::default(),buffers:&[
        wgpu::VertexBufferLayout {array_stride:8,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![0=>Float32x2]},
        wgpu::VertexBufferLayout {array_stride:std::mem::size_of::<crate::wgpu::draw_types::UiInstance>() as u64,step_mode:wgpu::VertexStepMode::Instance,attributes:&wgpu::vertex_attr_array![1=>Float32x4,2=>Float32x4,3=>Float32x4,4=>Float32x4,5=>Float32x4]},
    ]},primitive:Default::default(),depth_stencil:None,multisample:Default::default(),fragment:Some(wgpu::FragmentState {module:&shader,entry_point:Some("fs_main"),compilation_options:Default::default(),targets:&[Some(wgpu::ColorTargetState {format:wgpu::TextureFormat::Rgba8Unorm,blend:None,write_mask:wgpu::ColorWrites::ALL})]}),multiview_mask:None,cache:None});
    let vertices=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("affine_vertices"),contents:bytemuck::cast_slice(&[[0.0_f32,0.0],[1.0,0.0],[0.0,1.0],[0.0,1.0],[1.0,0.0],[1.0,1.0]]),usage:wgpu::BufferUsages::VERTEX});
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🖍️draw/🏷️types/↗️affine/🧫️fixtures/🔣️.json")).unwrap();
    for sample in fixture["cases"].as_array().unwrap() {
        let rect=std::array::from_fn(|index|sample["rect"][index].as_f64().unwrap() as f32);
        let mut matrix=std::array::from_fn(|index|sample["matrix"][index].as_f64().unwrap() as f32);
        matrix[4]+=0.125;
        matrix[5]+=0.125;
        let item=crate::wgpu::draw_types::UiInstance::raster_affine(rect,[0.0,0.0,1.0,1.0],1.0,matrix);
        let items=device.create_buffer_init(&wgpu::util::BufferInitDescriptor {label:Some("affine_instances"),contents:bytemuck::bytes_of(&item),usage:wgpu::BufferUsages::VERTEX});
        let target=device.create_texture(&wgpu::TextureDescriptor {label:Some("affine_target"),size:wgpu::Extent3d {width:32,height:32,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:wgpu::TextureFormat::Rgba8Unorm,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});
        let target_view=target.create_view(&wgpu::TextureViewDescriptor::default());
        let readback=device.create_buffer(&wgpu::BufferDescriptor {label:Some("affine_readback"),size:256*32,usage:wgpu::BufferUsages::COPY_DST|wgpu::BufferUsages::MAP_READ,mapped_at_creation:false});
        let mut encoder=device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("affine_paint"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:&target_view,resolve_target:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store},depth_slice:None})],depth_stencil_attachment:None,timestamp_writes:None,occlusion_query_set:None,multiview_mask:None});
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0,&group,&[]);
            pass.set_vertex_buffer(0,vertices.slice(..));
            pass.set_vertex_buffer(1,items.slice(..));
            pass.draw(0..6,0..1);
        }
        encoder.copy_texture_to_buffer(wgpu::TexelCopyTextureInfo {texture:&target,mip_level:0,origin:wgpu::Origin3d::ZERO,aspect:wgpu::TextureAspect::All},wgpu::TexelCopyBufferInfo {buffer:&readback,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(256),rows_per_image:Some(32)}},wgpu::Extent3d {width:32,height:32,depth_or_array_layers:1});
        queue.submit(Some(encoder.finish()));
        let (sender,receiver)=std::sync::mpsc::channel();
        readback.slice(..).map_async(wgpu::MapMode::Read,move|result|{let _=sender.send(result);});
        let deadline=std::time::Instant::now()+std::time::Duration::from_secs(10);
        loop {
            device.poll(wgpu::PollType::Poll).unwrap();
            match receiver.try_recv() {Ok(result)=>{result.unwrap();break;},Err(std::sync::mpsc::TryRecvError::Empty)=>{},Err(error)=>panic!("{error}")}
            assert!(std::time::Instant::now()<deadline,"affine readback completes");
            std::thread::yield_now();
        }
        let mapped=readback.slice(..).get_mapped_range();
        let mut reference=tiny_skia::Pixmap::new(32,32).unwrap();
        let mut builder=tiny_skia::PathBuilder::new();
        builder.move_to(sample["corners"][0][0].as_f64().unwrap() as f32+0.125,sample["corners"][0][1].as_f64().unwrap() as f32+0.125);
        for corner in sample["corners"].as_array().unwrap().iter().skip(1) {builder.line_to(corner[0].as_f64().unwrap() as f32+0.125,corner[1].as_f64().unwrap() as f32+0.125);}
        builder.close();
        if let Some(path)=builder.finish() {
            let mut paint=tiny_skia::Paint::default();
            paint.set_color_rgba8(255,0,0,255);
            paint.anti_alias=false;
            reference.fill_path(&path,&paint,tiny_skia::FillRule::Winding,tiny_skia::Transform::identity(),None);
        }
        for y in 0..32 {for x in 0..32 {
            let source=y*256+x*4;
            let expected=(y*32+x)*4;
            assert_eq!(&mapped[source..source+4],&reference.data()[expected..expected+4],"{} pixel ({x},{y})",sample["name"]);
        }}
        drop(mapped);
        readback.unmap();
    }
    assert!(semio_framework_async::block_on(validation.pop()).is_none());
    eprintln!("[DEBUG] Actual GPU affine image framebuffer matched Tiny Skia across five transforms and 5120 pixels");
}
