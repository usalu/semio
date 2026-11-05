//! 📜️ Raw semantic preparation requests the next actual retained effect instead of reserving a maximum frame.
use super::*;
type Demand=semio_framework_value::RetirementDemand;
fn quote(copy:usize,capacity:usize,release:usize)->Demand{Demand{copy_bytes:copy,capacity_bytes:capacity,release_bytes:release,depth:1,..Default::default()}}
impl RasterMutationCandidateAuthority{
 fn forward_demands(&self,operation:&RasterMutation)->Result<Demand,&'static str>{
  let(copy,capacity)=match operation{
   RasterMutation::RenameLayer(value)=>(2*size_of::<String>()+value.new_name.len(),value.new_name.len()),
   RasterMutation::ChangeLayerBlendMode(value)=>(2*size_of::<String>()+value.new_blend_mode.len(),value.new_blend_mode.len()),
   RasterMutation::ChangeLayerAdjustmentKind(value)=>(2*size_of::<String>()+value.new_adjustment_kind.len(),value.new_adjustment_kind.len()),
   RasterMutation::ChangeLayerLocked(_)|RasterMutation::ChangeLayerVisible(_)=>(size_of::<bool>(),0),RasterMutation::ChangeLayerOpacity(_)=>(size_of::<f32>(),0),RasterMutation::MoveLayer(_)=>(2*size_of::<f64>(),0),RasterMutation::ResizeLayer(_)=>(2*size_of::<Option<u32>>(),0),RasterMutation::ChangeLayerTransform(_)=>(size_of::<RasterTransform>(),0),
   RasterMutation::ChangeLayerMask(value)=>{let cap=value.mask.as_ref().and_then(|mask|mask.image_key.as_ref()).map_or(0,String::len);(2*size_of::<Option<semio_s_artifact_raster_raster_model::RasterLayerMask>>()+cap,cap)},
   RasterMutation::ChangeLayerPixels(value)=>{let cap=value.content.image_key.as_ref().map_or(0,String::len);(2*size_of::<semio_s_artifact_raster_raster_model::RasterPixelContent>()+size_of::<RasterTransform>()+cap,cap)},
   RasterMutation::ChangeLayerAdjustmentParameter(value)=>{let snapshot=self.value.as_ref().ok_or("raster-store.demand-snapshot")?;let RasterLayerNode::Adjustment{params,..}=RasterLayerLocator::node_at(snapshot,self.primary.ok_or("raster-store.demand-address")?).ok_or("raster-store.demand-target")?else{return Err("raster-store.demand-parameter-target")};if value.value.is_some()&&params.page_required_for_insert(&value.parameter){let page=RasterOwnedMap::<semio_framework_value::DslValue>::conservative_page_credit_bytes();(page,page)}else{let cap=if value.value.is_some(){value.parameter.len()}else{0};(3*size_of::<(String,semio_framework_value::DslValue)>()+cap,cap)}},
   _=>(0,0),
  };Ok(quote(copy,capacity,0))
 }
 fn demands(&self,base:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<Demand,&'static str>{
  if !self.retirement.is_empty(){return self.retirement.demands(body).map_err(|_|"raster-store.demand-retirement")}
  Ok(match self.phase{
   RasterMutationCandidatePhase::Clone=>if let Some(clone)=self.clone.as_ref(){if clone.terminal{quote(size_of::<RasterSnapshot>(),0,size_of::<RasterSnapshotCloneAuthority>())}else{return clone.demands(base,body)}}else{let bytes=size_of::<RasterSnapshotCloneAuthority>();quote(bytes,bytes,0)},
   RasterMutationCandidatePhase::PrepareLayer=>{let RasterMutation::CreateLayer(value)=operation else{return Err("raster-store.demand-layer-operation")};if let Some(clone)=self.layer_clone.as_ref(){if clone.terminal{quote(size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>())}else{return clone.demands(&value.layer,body)}}else{let bytes=size_of::<RasterLayerCloneAuthority>();quote(bytes,bytes,0)}},
   RasterMutationCandidatePhase::Apply=>{
    if pixel_target(operation).is_some()||matches!(operation,RasterMutation::AddLayerAsset(_)|RasterMutation::RemoveLayerAsset(_)){
     if let Some(work)=self.pixel.as_ref(){if self.pixel_ready{if work.terminal_is_empty(){quote(0,0,size_of::<RasterRetainedMutationWork>())}else{return work.close_demands(body).map_err(|_|"raster-store.demand-pixel-close")}}else if work.output_ready(){quote(size_of::<bool>()+if self.prepare_inverse{2*size_of::<Option<Vec<RasterMutation>>>()}else{0},0,0)}else{return work.demands(base,self.value.as_ref().ok_or("raster-store.demand-snapshot")?,operation,self.primary,body)}}else{let bytes=size_of::<RasterRetainedMutationWork>();quote(bytes,bytes,0)}
    }else if self.prepare_inverse&&!self.inverse_ready{return self.inverse_demands(base,operation,body)}else{return self.forward_demands(operation)}
   },
   RasterMutationCandidatePhase::BeginInsert=>quote(2*size_of::<RasterLayerNode>(),0,0),
   RasterMutationCandidatePhase::ShiftInsert=>if self.shift_index>self.shift_target{quote(3*size_of::<RasterLayerNode>(),0,0)}else{quote(0,0,0)},
   RasterMutationCandidatePhase::ShiftRemove=>{let snapshot=self.value.as_ref().ok_or("raster-store.demand-snapshot")?;let values=if let Some(parent)=self.container{let RasterLayerNode::Group{children,..}=RasterLayerLocator::node_at(snapshot,parent).ok_or("raster-store.demand-parent")?else{return Err("raster-store.demand-parent-kind")};children}else{&snapshot.layers};quote(if self.shift_index+1<values.len(){3}else{2}*size_of::<RasterLayerNode>(),0,0)},
   _=>quote(0,0,0),
  })
 }
}
impl RasterOneItemApply{
 /// 🎟️ Quotes one current effect; CPU-only cursor turns request no fabricated payload capacity.
 pub fn advance_demands(&self,base:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<Demand,ValueError>{if let Some((_,source,mutation))=self.binding{if source!=base as*const RasterSnapshot as usize||mutation!=operation as*const RasterMutation as usize{return Err(invariant("raster-store.one-item-original-source-changed"))}}else{return Ok(Self::binding_demands())}self.candidate.as_ref().ok_or_else(||invariant("raster-store.demand-candidate"))?.demands(base,operation,body).map_err(invariant)}
}
