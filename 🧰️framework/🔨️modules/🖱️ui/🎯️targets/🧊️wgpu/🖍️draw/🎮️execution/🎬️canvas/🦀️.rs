//! 🎬️ Exact admitted EngineCanvas GPU resources retained through stage refusal and close.
use semio_framework_ui_wgpu_device as gpu_device;
use super::{GpuContext,GpuSessionRoots};
use crate::draw::{RasterTextureAdmission,RasterTextureCancelFault,RasterTextureStageFault,RasterTextureWitness};
use crate::prepared::RasterContentIdentity;
use semio_framework_canvas_scene::{Scene,Color};
use gpu_device::platform::CanvasRasterizer as CanvasRasterSession;
use std::sync::Arc;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum EngineRasterBuildPhase {Reserve,Texture,View,Renderer,Render,Stage,RetireRenderer,Publish,ClosingAdmission,ClosingRenderer,ClosingView,ClosingTexture,Terminal}
pub struct EngineRasterBuild {roots:Option<Arc<GpuSessionRoots>>,admission:Option<RasterTextureAdmission>,renderer:Option<CanvasRasterSession>,texture:Option<gpu_device::Texture>,view:Option<gpu_device::TextureView>,width:u32,height:u32,phase:EngineRasterBuildPhase}
pub enum EngineRasterStageFault {Returned{fault:String,build:EngineRasterBuild},Retained{fault:String,build:EngineRasterBuild}}
impl EngineRasterBuild {
 pub fn phase(&self)->EngineRasterBuildPhase{self.phase}
 pub fn begin_close(&mut self){if !self.terminal_is_empty(){self.phase=EngineRasterBuildPhase::ClosingAdmission;}}
 pub fn terminal_is_empty(&self)->bool{matches!(self.phase,EngineRasterBuildPhase::Terminal|EngineRasterBuildPhase::Publish)&&self.roots.is_none()&&self.admission.is_none()&&self.renderer.is_none()&&self.texture.is_none()&&self.view.is_none()}
}
impl Drop for EngineRasterBuild {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original EngineCanvas GPU build dropped before exact retirement");}}
impl GpuContext {
 #[expect(clippy::too_many_arguments,reason="The exact pre-existing raster reservation tuple remains visible at admission.")]
 pub fn begin_engine_raster(&mut self,key:&str,width:u32,height:u32,identity:RasterContentIdentity,candidate:RasterTextureWitness,expected:RasterTextureWitness)->Result<EngineRasterBuild,String>{let admission=self.reserve_engine_texture(key,width,height,1,identity,candidate,expected)?;Ok(EngineRasterBuild{roots:Some(Arc::clone(&self.roots)),admission:Some(admission),renderer:None,texture:None,view:None,width,height,phase:EngineRasterBuildPhase::Texture})}
 #[expect(clippy::result_large_err,reason="Refusal retains the exact original resource bundle without allocating another owner.")]
 pub fn engine_raster_step(&mut self,mut build:EngineRasterBuild,scene:&Scene,clear:Color,expected:RasterTextureWitness)->Result<EngineRasterBuild,EngineRasterStageFault>{
  let result=(||->Result<(),(String,bool)>{if !build.roots.as_ref().is_some_and(|roots|Arc::ptr_eq(roots,&self.roots)){return Err(("engine GPU build belongs to another actual session".into(),false));}match build.phase {
   EngineRasterBuildPhase::Texture=>{let admission=build.admission.as_ref().ok_or_else(||("engine texture admission was missing".into(),false))?;self.validate_engine_target_texture_allocation(admission,expected).map_err(|fault|(fault,false))?;build.texture=Some(self.roots.device.create_texture(&gpu_device::TextureDescriptor{label:Some("engine_canvas_target"),size:gpu_device::Extent3d{width:build.width,height:build.height,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:gpu_device::TextureDimension::D2,format:gpu_device::TextureFormat::Rgba8Unorm,usage:gpu_device::TextureUsages::STORAGE_BINDING|gpu_device::TextureUsages::TEXTURE_BINDING,view_formats:&[]}));build.phase=EngineRasterBuildPhase::View;}
   EngineRasterBuildPhase::View=>{let admission=build.admission.as_ref().ok_or_else(||("engine view admission was missing".into(),false))?;self.validate_engine_target_view_allocation(admission,expected).map_err(|fault|(fault,false))?;let texture=build.texture.as_ref().ok_or_else(||("engine texture owner was missing".into(),false))?;build.view=Some(texture.create_view(&gpu_device::TextureViewDescriptor::default()));build.phase=EngineRasterBuildPhase::Renderer;}
   EngineRasterBuildPhase::Renderer=>{let admission=build.admission.as_ref().ok_or_else(||("engine renderer admission was missing".into(),false))?;self.validate_engine_renderer_allocation(admission,expected).map_err(|fault|(fault,false))?;build.renderer=Some(CanvasRasterSession::new(&self.roots.device).map_err(|fault|(format!("vello renderer: {fault:?}"),false))?);build.phase=EngineRasterBuildPhase::Render;}
   EngineRasterBuildPhase::Render=>{let admission=build.admission.as_ref().ok_or_else(||("engine render admission was missing".into(),false))?;self.validate_engine_renderer_allocation(admission,expected).map_err(|fault|(fault,false))?;let renderer=build.renderer.as_mut().ok_or_else(||("engine renderer owner was missing".into(),false))?;let view=build.view.as_ref().ok_or_else(||("engine render view owner was missing".into(),false))?;renderer.render(&self.roots.device,&self.roots.queue,scene,view,clear,build.width,build.height).map_err(|fault|(fault,false))?;build.phase=EngineRasterBuildPhase::Stage;}
   EngineRasterBuildPhase::Stage=>{let admission=build.admission.take().ok_or_else(||("engine stage admission was missing".into(),false))?;let texture=build.texture.take().expect("admitted engine texture owner");let view=build.view.take().expect("admitted engine view owner");match self.stage_engine_texture(admission,texture,view,expected){Ok(())=>build.phase=EngineRasterBuildPhase::RetireRenderer,Err(RasterTextureStageFault::Returned{fault,admission,texture,view})=>{build.admission=Some(admission);build.texture=Some(texture);build.view=Some(view);return Err((fault.into(),false));},Err(RasterTextureStageFault::Retained(fault))=>return Err((fault.into(),true)),}}
   EngineRasterBuildPhase::RetireRenderer=>{if build.renderer.take().is_none(){build.roots.take();build.phase=EngineRasterBuildPhase::Publish;}}
   _=>return Err(("engine GPU build is not accepting rendering".into(),false)),}Ok(())})();
  match result{Ok(())=>Ok(build),Err((fault,retained))=>{build.begin_close();if retained{Err(EngineRasterStageFault::Retained{fault,build})}else{Err(EngineRasterStageFault::Returned{fault,build})}}}
 }
 pub fn close_engine_raster_step(&mut self,build:&mut EngineRasterBuild)->Result<bool,String>{if build.terminal_is_empty(){return Ok(true)}if !build.roots.as_ref().is_some_and(|roots|Arc::ptr_eq(roots,&self.roots)){return Err("engine GPU close belongs to another actual session".into());}match build.phase {
  EngineRasterBuildPhase::ClosingAdmission=>{if let Some(admission)=build.admission.take(){match self.cancel_engine_texture_admission(admission){Ok(())=>{},Err(RasterTextureCancelFault::Returned{fault,admission})=>{build.admission=Some(admission);return Err(fault.into())},Err(RasterTextureCancelFault::Retained(fault))=>return Err(fault.into())}return Ok(false)}build.phase=EngineRasterBuildPhase::ClosingRenderer;}
  EngineRasterBuildPhase::ClosingRenderer=>{if build.renderer.take().is_some(){return Ok(false)}build.phase=EngineRasterBuildPhase::ClosingView;}
  EngineRasterBuildPhase::ClosingView=>{if build.view.take().is_some(){return Ok(false)}build.phase=EngineRasterBuildPhase::ClosingTexture;}
  EngineRasterBuildPhase::ClosingTexture=>{if build.texture.take().is_some(){return Ok(false)}build.roots.take();build.phase=EngineRasterBuildPhase::Terminal;}
  _=>build.begin_close(),}Ok(build.terminal_is_empty())}
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="../../🧪️tests/🎬️engine-raster/🦀️.rs"]
mod tests;
