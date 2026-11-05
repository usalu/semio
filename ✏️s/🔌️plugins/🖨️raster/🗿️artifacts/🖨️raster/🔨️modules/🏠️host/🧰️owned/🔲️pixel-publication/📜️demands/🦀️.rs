//! 📜️ Publication quotes its next image custody, inverse row or attachment effect.
use super::*;
type Demand=semio_framework_value::RetirementDemand;
fn quote(copy:usize,capacity:usize)->Demand{Demand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()}}
impl RasterPixelMutationWork{
 fn attachment_demand(&self,restore:bool,detached:bool)->Result<Demand,&'static str>{let data=self.data()?;let result=self.ready()?;let bytes=data.layer_id.len()+if restore{data.previous.as_ref().map_or(0,String::len)}else{0}+if detached{0}else{result.key.as_ref().map_or(0,String::len)};Ok(quote(bytes+size_of::<RasterTransform>(),bytes))}
 pub(in super::super) fn demands(&self,base:&RasterSnapshot,candidate:&RasterSnapshot,operation:&RasterMutation,address:RasterLayerAddress,body:usize)->Result<Demand,&'static str>{
  Ok(match self.phase{
   Phase::Compute=>return self.result.as_ref().ok_or("raster-store.pixel-demand-result")?.demands(base,operation,body).map_err(|_|"raster-store.pixel-demand-compute"),
   Phase::Capture=>{if !self.ready()?.changed{return Ok(quote(0,0))}let(id,target)=pixel_target(operation).ok_or("raster-store.pixel-demand-operation")?;let layer=RasterLayerLocator::node_at(base,address).ok_or("raster-store.pixel-demand-layer")?;let(mask,prior)=match(target,layer){("mask",RasterLayerNode::Pixel{mask:Some(mask),..}|RasterLayerNode::Group{mask:Some(mask),..})=>(mask.image_key.as_ref().map_or(0,String::len),mask.image_key.as_ref().map_or(0,String::len)),("pixels",RasterLayerNode::Pixel{image_key,..})=>(0,image_key.as_ref().map_or(0,String::len)),_=>return Err("raster-store.pixel-demand-target")};let bytes=id.len()+prior+mask;quote(bytes+size_of::<RasterTransform>(),bytes)},
   Phase::Root=>if self.ready()?.reuse_destination{quote(0,0)}else{RasterImagePayload::owned_birth_demands()},
   Phase::ReserveInverse=>if self.inverse{quote(0,4*size_of::<RasterMutation>())}else{quote(0,0)},
   Phase::Detach=>if self.detach{self.attachment_demand(false,false)?}else{quote(0,0)},
   Phase::Remove=>if self.detach{let bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-demand-key")?.len();quote(bytes,bytes)}else{quote(0,0)},
   Phase::AddRoot=>if self.release{RasterImagePayload::alias_demands()}else{quote(0,0)},
   Phase::Add=>{let bytes=self.data()?.previous.as_ref().ok_or("raster-store.pixel-demand-previous")?.len();quote(bytes+size_of::<RasterMutation>(),bytes)},
   Phase::Restore=>self.attachment_demand(true,self.detach)?,
   Phase::RemoveFinal|Phase::Attach=>{let bytes=self.ready()?.key.as_ref().ok_or("raster-store.pixel-demand-key")?.len();quote(bytes,bytes)},
   Phase::ChildRoot=>if self.ready()?.reuse_destination{quote(0,0)}else{quote(size_of::<(String,RasterAssetChild)>(),0)},
   Phase::Child=>{if self.ready()?.reuse_destination{return Ok(quote(0,0))}let bytes=2*self.ready()?.key.as_ref().ok_or("raster-store.pixel-demand-key")?.len()+6+RASTER_IMAGE_CHILD_ID_BYTES+b"s.stdio.semio".len()+b"v1".len()+b"image".len();quote(bytes+size_of::<RasterAssetChild>(),bytes)},
   Phase::Release=>if self.release{quote(size_of::<(String,RasterAssetChild)>(),0)}else{quote(0,0)},
   Phase::ReserveMap=>{if self.ready()?.reuse_destination{return Ok(quote(0,0))}let key=&self.data()?.pending.as_ref().ok_or("raster-store.pixel-demand-child")?.0;if candidate.assets.page_required_for_insert(key){quote(0,RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes())}else{quote(0,0)}},
   Phase::Insert=>quote(size_of::<(String,RasterAssetChild)>(),0),
   Phase::References|Phase::Ready=>quote(0,0),
  })
 }
}
