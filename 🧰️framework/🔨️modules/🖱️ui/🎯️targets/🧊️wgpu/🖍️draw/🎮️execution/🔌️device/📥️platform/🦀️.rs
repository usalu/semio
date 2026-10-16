//! 📥️ Platform ingress captures real surfaces before asynchronous device preparation.
use super::*;
pub struct Surface{raw:wgpu::Surface<'static>,session:Session}
pub struct SurfaceTexture{raw:wgpu::SurfaceTexture,pub texture:Texture}
#[derive(Debug)]
pub enum CurrentSurfaceTexture{Success(SurfaceTexture),Suboptimal(SurfaceTexture),Unavailable(String)}
impl std::fmt::Debug for SurfaceTexture{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("SurfaceTexture").finish_non_exhaustive()}}
pub struct PreparedDevice{pub device:Device,pub queue:Queue,pub surface:Option<Surface>,pub config:SurfaceConfiguration,pub logical_width:f32,pub logical_height:f32,pub dpr:f32}
impl Surface{
    pub fn configure(&self,device:&Device,config:&SurfaceConfiguration){self.session.check(&device.session).expect("same-session surface configuration");self.raw.configure(&device.session.0.device,&conversion::surface_config(config));}
    pub fn get_current_texture(&self)->CurrentSurfaceTexture{
        let wrap=|raw:wgpu::SurfaceTexture|SurfaceTexture{texture:Texture{raw:raw.texture.clone(),session:self.session.clone()},raw};
        match self.raw.get_current_texture(){wgpu::CurrentSurfaceTexture::Success(raw)=>CurrentSurfaceTexture::Success(wrap(raw)),wgpu::CurrentSurfaceTexture::Suboptimal(raw)=>CurrentSurfaceTexture::Suboptimal(wrap(raw)),other=>CurrentSurfaceTexture::Unavailable(format!("{other:?}"))}
    }
}
impl SurfaceTexture{pub fn present(self){self.raw.present();}}
fn instance()->wgpu::Instance{wgpu::Instance::new(wgpu::InstanceDescriptor{backends:if cfg!(target_arch="wasm32"){wgpu::Backends::BROWSER_WEBGPU}else{wgpu::Backends::PRIMARY},..wgpu::InstanceDescriptor::new_without_display_handle()})}
async fn prepare(instance:wgpu::Instance,surface:Option<wgpu::Surface<'static>>,width:f32,height:f32,dpr:f32)->Result<PreparedDevice,String>{
    if ![width,height,dpr].iter().all(|value|value.is_finite()&&*value>0.0){return Err("finite positive render extent required".into());}
    let adapter=instance.request_adapter(&wgpu::RequestAdapterOptions{power_preference:wgpu::PowerPreference::HighPerformance,compatible_surface:surface.as_ref(),force_fallback_adapter:false}).await.map_err(|error|format!("{}adapter: {error}",if surface.is_none(){"offscreen "}else{""}))?;
    let limits=wgpu::Limits::default().using_resolution(adapter.limits());
    let physical_width=(width*dpr).max(1.0)as u32;let physical_height=(height*dpr).max(1.0)as u32;
    if physical_width>limits.max_texture_dimension_2d||physical_height>limits.max_texture_dimension_2d{return Err("render extent exceeds actual device limits".into());}
    let(format,alpha_mode)=if let Some(surface)=&surface{let caps=surface.get_capabilities(&adapter);let raw=caps.formats.iter().copied().find(|format|!format.is_srgb()).or_else(||caps.formats.first().copied()).ok_or("surface advertises no format")?;let format=conversion::format(raw).ok_or("surface format is outside the declared render contract")?;let alpha=match caps.alpha_modes.first().ok_or("surface advertises no alpha mode")?{wgpu::CompositeAlphaMode::Auto=>CompositeAlphaMode::Auto,wgpu::CompositeAlphaMode::Opaque=>CompositeAlphaMode::Opaque,wgpu::CompositeAlphaMode::PreMultiplied=>CompositeAlphaMode::PreMultiplied,wgpu::CompositeAlphaMode::PostMultiplied=>CompositeAlphaMode::PostMultiplied,wgpu::CompositeAlphaMode::Inherit=>CompositeAlphaMode::Inherit};(format,alpha)}else{(TextureFormat::Rgba8Unorm,CompositeAlphaMode::PreMultiplied)};
    let(device,queue)=adapter.request_device(&wgpu::DeviceDescriptor{label:Some(if surface.is_some(){"ui_wgpu"}else{"ui_wgpu_export"}),required_features:wgpu::Features::empty(),required_limits:limits,memory_hints:Default::default(),trace:wgpu::Trace::Off,experimental_features:Default::default()}).await.map_err(|error|format!("{}device: {error}",if surface.is_none(){"offscreen "}else{""}))?;
    let(device,queue)=Device::issued(device,queue);
    let config=SurfaceConfiguration{usage:TextureUsages::RENDER_ATTACHMENT,format,width:physical_width,height:physical_height,present_mode:PresentMode::AutoVsync,alpha_mode,view_formats:vec![format.add_srgb_suffix()],desired_maximum_frame_latency:2};
    let surface=surface.map(|raw|Surface{raw,session:device.session.clone()});
    if let Some(surface)=&surface{surface.configure(&device,&config);}
    Ok(PreparedDevice{device,queue,surface,config,logical_width:width,logical_height:height,dpr})
}
pub(super) async fn headless()->Result<(Device,Queue),String>{let prepared=prepare(instance(),None,1.0,1.0,1.0).await?;Ok((prepared.device,prepared.queue))}
pub async fn prepare_headless(width:u32,height:u32,readback_bytes:u64)->Result<PreparedDevice,String>{let prepared=prepare(instance(),None,width as f32,height as f32,1.0).await?;if readback_bytes>prepared.device.limits().max_buffer_size{return Err("prepared offscreen dimensions exceed the device budget".into());}Ok(prepared)}

#[cfg(not(target_os="wasi"))]
pub use winit::window::Window as NativeWindow;
#[cfg(not(target_os="wasi"))]
pub struct NativeSurfaceCapture{instance:wgpu::Instance,surface:wgpu::Surface<'static>,width:f32,height:f32,dpr:f32}
#[cfg(not(target_os="wasi"))]
impl NativeSurfaceCapture{
    pub fn capture(window:Arc<NativeWindow>)->Result<Self,String>{let dpr=window.scale_factor()as f32;let size=window.inner_size();let width=size.width as f32/dpr;let height=size.height as f32/dpr;let instance=wgpu::Instance::new(wgpu::InstanceDescriptor{backends:wgpu::Backends::PRIMARY,..wgpu::InstanceDescriptor::new_with_display_handle(Box::new(window.clone()))});let surface=instance.create_surface(wgpu::SurfaceTarget::Window(Box::new(window))).map_err(|error|format!("surface: {error:?}"))?;Ok(Self{instance,surface,width,height,dpr})}
    pub async fn prepare(self)->Result<PreparedDevice,String>{prepare(self.instance,Some(self.surface),self.width,self.height,self.dpr).await}
}
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use web_sys::OffscreenCanvas as TransferredCanvas;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub async fn prepare_canvas(canvas:TransferredCanvas,width:f32,height:f32,dpr:f32)->Result<PreparedDevice,String>{let instance=instance();let surface=instance.create_surface(wgpu::SurfaceTarget::OffscreenCanvas(canvas)).map_err(|error|format!("offscreen surface: {error:?}"))?;prepare(instance,Some(surface),width,height,dpr).await}

#[cfg(all(feature="canvas-raster",not(all(target_arch="wasm32",target_env="p2"))))]
pub struct CanvasRasterizer{raw:semio_framework_canvas_scene::native_render::CanvasRasterSession,session:Session}
#[cfg(all(feature="canvas-raster",not(all(target_arch="wasm32",target_env="p2"))))]
impl CanvasRasterizer{
    pub fn new(device:&Device)->Result<Self,String>{Ok(Self{raw:semio_framework_canvas_scene::native_render::CanvasRasterSession::new(&device.session.0.device).map_err(|error|format!("vello renderer: {error:?}"))?,session:device.session.clone()})}
    pub fn render(&mut self,device:&Device,queue:&Queue,scene:&semio_framework_canvas_scene::Scene,view:&TextureView,clear:semio_framework_canvas_scene::Color,width:u32,height:u32)->Result<(),String>{self.session.check(&device.session).map_err(|error|error.to_string())?;self.session.check(&queue.session).map_err(|error|error.to_string())?;self.session.check(&view.session).map_err(|error|error.to_string())?;self.raw.render(&device.session.0.device,&queue.session.0.queue,scene,&view.raw,clear,width,height)}
}
