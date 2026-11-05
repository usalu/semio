//! ↩️ Original tree addresses and bounded field copies prepare point inverses before candidate mutation.
use super::*;
use semio_s_artifact_raster_raster_model::{RasterLayerMask,RasterPixelContent,RasterTransform};
use semio_s_artifact_raster_raster_mutations_layers::{create_layer,delete_layer,reorder_layers,rename_layer,move_layer};
use semio_s_artifact_raster_raster_mutations_layer_properties::{change_layer_visible,change_layer_locked,change_layer_opacity,change_layer_blend_mode,change_layer_adjustment_kind,change_layer_transform,change_layer_adjustment_parameter};
use semio_s_artifact_raster_raster_mutations_pixels::{change_layer_pixels,change_layer_mask};

enum Row<'a>{
 Delete(&'a str),Create(Option<&'a str>,usize),Reorder(&'a str,Option<&'a str>,usize),Rename(&'a str,&'a str),Visible(&'a str,bool),Locked(&'a str,bool,bool),Opacity(&'a str,f32),Blend(&'a str,&'a str),Move(&'a str,f64,f64),Adjustment(&'a str,&'a str),Transform(&'a str,&'a RasterTransform,&'a RasterTransform),Parameter(&'a change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter),Pixels(&'a str,Option<&'a str>,Option<&'a str>,Option<u32>,Option<u32>,Option<&'a RasterTransform>),Mask(&'a str,&'a Option<RasterLayerMask>,&'a Option<RasterLayerMask>),
}

fn text(value:&str)->String{raster_exact_string_from_parts(&[value.as_bytes()]).expect("prequoted original UTF8 field")}
fn mask(value:&Option<RasterLayerMask>)->Option<RasterLayerMask>{value.as_ref().map(|value|RasterLayerMask{enabled:value.enabled,linked:value.linked,invert:value.invert,width:value.width,height:value.height,image_key:value.image_key.as_deref().map(text),transform:value.transform.clone()})}
impl Row<'_>{
 fn strings(&self)->[Option<&str>;3]{match self{
  Self::Delete(id)=>[Some(id),None,None],Self::Create(parent,_)=>[*parent,None,None],Self::Reorder(id,parent,_)=>[Some(id),*parent,None],Self::Rename(id,value)|Self::Blend(id,value)|Self::Adjustment(id,value)=>[Some(id),Some(value),None],Self::Parameter(value)=>[Some(&value.layer_id),Some(&value.parameter),None],Self::Pixels(id,expected,key,..)=>[Some(id),*expected,*key],Self::Mask(id,expected,value)=>[Some(id),expected.as_ref().and_then(|mask|mask.image_key.as_deref()),value.as_ref().and_then(|mask|mask.image_key.as_deref())],Self::Visible(id,_)|Self::Locked(id,..)|Self::Opacity(id,_)|Self::Move(id,..)|Self::Transform(id,..)=>[Some(id),None,None],
 }}
 fn demand(&self)->Result<semio_framework_value::RetirementDemand,&'static str>{let mut bytes=0;for value in self.strings().into_iter().flatten(){if value.len()>RASTER_OWNED_FIELD_BYTES{return Err("raster-store.inverse-field-bound")}bytes+=value.len();}let layer=if matches!(self,Self::Create(..)){size_of::<RasterLayerNode>()}else{0};Ok(semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<RasterMutation>()+bytes+layer,capacity_bytes:bytes+layer,depth:1,..Default::default()})}
 fn build(self,layer:&mut Option<RasterLayerNode>)->RasterMutation{match self{
  Self::Delete(id)=>RasterMutation::DeleteLayer(delete_layer::DeleteLayer{layer_id:text(id)}),
  Self::Create(parent,index)=>RasterMutation::CreateLayer(create_layer::CreateLayer{parent_id:parent.map(text),index,layer:Box::new(layer.take().expect("original deleted subtree prepared"))}),
  Self::Reorder(id,parent,index)=>RasterMutation::ReorderLayers(reorder_layers::ReorderLayers{layer_id:text(id),parent_id:parent.map(text),index}),
  Self::Rename(id,value)=>RasterMutation::RenameLayer(rename_layer::RenameLayer{layer_id:text(id),new_name:text(value)}),
  Self::Visible(id,value)=>RasterMutation::ChangeLayerVisible(change_layer_visible::ChangeLayerVisible{layer_id:text(id),new_visible:value}),
  Self::Locked(id,expected,locked)=>RasterMutation::ChangeLayerLocked(change_layer_locked::ChangeLayerLocked{layer_id:text(id),expected,locked}),
  Self::Opacity(id,value)=>RasterMutation::ChangeLayerOpacity(change_layer_opacity::ChangeLayerOpacity{layer_id:text(id),new_opacity:value}),
  Self::Blend(id,value)=>RasterMutation::ChangeLayerBlendMode(change_layer_blend_mode::ChangeLayerBlendMode{layer_id:text(id),new_blend_mode:text(value)}),
  Self::Move(id,x,y)=>RasterMutation::MoveLayer(move_layer::MoveLayer{layer_id:text(id),new_x:x,new_y:y}),
  Self::Adjustment(id,value)=>RasterMutation::ChangeLayerAdjustmentKind(change_layer_adjustment_kind::ChangeLayerAdjustmentKind{layer_id:text(id),new_adjustment_kind:text(value)}),
  Self::Transform(id,expected,value)=>RasterMutation::ChangeLayerTransform(change_layer_transform::ChangeLayerTransform{layer_id:text(id),expected:expected.clone(),transform:value.clone()}),
  Self::Parameter(value)=>RasterMutation::ChangeLayerAdjustmentParameter(change_layer_adjustment_parameter::ChangeLayerAdjustmentParameter{layer_id:text(&value.layer_id),parameter:text(&value.parameter),expected:value.value,value:value.expected}),
  Self::Pixels(id,expected,key,width,height,transform)=>RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels{layer_id:text(id),expected_image_key:expected.map(text),content:RasterPixelContent{image_key:key.map(text),width,height},transform:transform.cloned()}),
  Self::Mask(id,expected,value)=>RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask{layer_id:text(id),expected:mask(expected),mask:mask(value)}),
 }}
}

impl RasterMutationCandidateAuthority{
 pub(super) fn inverse_demands(&self,current:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<semio_framework_value::RetirementDemand,&'static str>{
  if self.inverse.is_none(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Vec<RasterMutation>>(),capacity_bytes:size_of::<RasterMutation>(),depth:1,..Default::default()})}
  if matches!(operation,RasterMutation::DeleteLayer(_))&&self.pending_layer.is_none(){let layer=RasterLayerLocator::node_at(current,self.primary.ok_or("raster-store.inverse-delete-address")?).ok_or("raster-store.inverse-delete-target")?;return if let Some(clone)=self.layer_clone.as_ref(){if clone.terminal{Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<RasterLayerNode>(),release_bytes:size_of::<RasterLayerCloneAuthority>(),depth:1,..Default::default()})}else{clone.demands(layer,body)}}else{let bytes=size_of::<RasterLayerCloneAuthority>();Ok(semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()})}}
  self.inverse_row(current,operation)?.map_or(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}),|row|row.demand())
 }
 fn inverse_row<'a>(&self,current:&'a RasterSnapshot,operation:&'a RasterMutation)->Result<Option<Row<'a>>,&'static str>{
  if let RasterMutation::CreateLayer(value)=operation{return Ok(Some(Row::Delete(RasterLayerLocator::node_id(&value.layer))))}
  let address=self.primary.ok_or("raster-store.inverse-address")?;let layer=RasterLayerLocator::node_at(current,address).ok_or("raster-store.inverse-target")?;let id=RasterLayerLocator::node_id(layer);
  let parent=address.parent().map(|parent|RasterLayerLocator::node_at(current,parent).map(RasterLayerLocator::node_id).ok_or("raster-store.inverse-parent")).transpose()?;
  Ok(match operation{
   RasterMutation::DeleteLayer(_)=>Some(Row::Create(parent,address.index())),RasterMutation::ReorderLayers(_)=>Some(Row::Reorder(id,parent,address.index())),RasterMutation::RenameLayer(_)=>Some(Row::Rename(id,semio_s_artifact_raster_raster_model::layer_name(layer))),RasterMutation::ChangeLayerVisible(_)=>Some(Row::Visible(id,semio_s_artifact_raster_raster_model::layer_visible(layer))),RasterMutation::ChangeLayerLocked(value)=>Some(Row::Locked(id,value.locked,semio_s_artifact_raster_raster_model::layer_locked(layer))),RasterMutation::ChangeLayerOpacity(_)=>Some(Row::Opacity(id,semio_s_artifact_raster_raster_model::layer_opacity(layer))),RasterMutation::ChangeLayerBlendMode(_)=>Some(Row::Blend(id,semio_s_artifact_raster_raster_model::layer_blend_mode(layer))),
   RasterMutation::MoveLayer(_)=>{let transform=semio_s_artifact_raster_raster_model::layer_transform(layer);Some(Row::Move(id,transform.x,transform.y))},
   RasterMutation::ResizeLayer(_)=>match layer{RasterLayerNode::Pixel{image_key,width,height,..}=>Some(Row::Pixels(id,image_key.as_deref(),image_key.as_deref(),*width,*height,None)),_=>None},
   RasterMutation::ChangeLayerAdjustmentKind(_)=>match layer{RasterLayerNode::Adjustment{adjustment_kind,..}=>Some(Row::Adjustment(id,adjustment_kind)),_=>None},
   RasterMutation::ChangeLayerTransform(value)=>Some(Row::Transform(id,&value.transform,&value.expected)),RasterMutation::ChangeLayerAdjustmentParameter(value)=>Some(Row::Parameter(value)),
   RasterMutation::ChangeLayerMask(value)=>match layer{RasterLayerNode::Pixel{mask,..}|RasterLayerNode::Group{mask,..}=>Some(Row::Mask(id,&value.mask,mask)),_=>None},
   RasterMutation::ChangeLayerPixels(value)=>match layer{RasterLayerNode::Pixel{image_key,width,height,transform,..}=>Some(Row::Pixels(id,value.content.image_key.as_deref(),image_key.as_deref(),*width,*height,value.transform.as_ref().map(|_|transform))),_=>None},
   _=>return Err("raster-store.inverse-route"),
  })
 }

 pub(super) fn step_inverse(&mut self,current:&RasterSnapshot,operation:&RasterMutation,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
  if self.inverse.is_none(){let demand=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Vec<RasterMutation>>(),capacity_bytes:size_of::<RasterMutation>(),depth:1,..Default::default()};if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<demand.depth||raster_short(cx.retained_grant(),demand){return Ok(false)}let rows:Vec<RasterMutation>=empty_vec(1);let capacity=rows.capacity()*size_of::<RasterMutation>();*self.inverse=Some(rows);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()}).map_err(|_|"raster-store.inverse-row-receipt")?;return Ok(false)}
  if matches!(operation,RasterMutation::DeleteLayer(_))&&self.pending_layer.is_none(){
   let layer=RasterLayerLocator::node_at(current,self.primary.ok_or("raster-store.inverse-delete-address")?).ok_or("raster-store.inverse-delete-target")?;
   if self.layer_clone.is_none(){let bytes=size_of::<RasterLayerCloneAuthority>();let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<1||raster_short(cx.retained_grant(),demand){return Ok(false)}*self.layer_clone=Some(Box::new(RasterLayerCloneAuthority::new(layer)));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()}).map_err(|_|"raster-store.inverse-clone-birth")?;return Ok(false)}
   let clone=self.layer_clone.as_mut().ok_or("raster-store.inverse-clone")?;if !clone.terminal{clone.step(layer,cx)?;return Ok(false)}let bytes=size_of::<RasterLayerNode>();let shell=size_of::<RasterLayerCloneAuthority>();let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,release_bytes:shell,depth:1,..Default::default()};if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<1||raster_short(cx.retained_grant(),demand){return Ok(false)}*self.pending_layer=clone.take();drop(self.layer_clone.take());cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,released_bytes:shell,..Default::default()}).map_err(|_|"raster-store.inverse-clone-handoff")?;return Ok(false)
  }
  let row=self.inverse_row(current,operation)?;if let Some(row)=row{let demand=row.demand()?;if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth<1||raster_short(cx.retained_grant(),demand){return Ok(false)}let row=row.build(&mut self.pending_layer);self.inverse.as_mut().ok_or("raster-store.inverse-rows")?.push(row);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}).map_err(|_|"raster-store.inverse-build-receipt")?;}else{if !raster_reserve_unit(cx){return Ok(false)}}
  self.inverse_ready=true;Ok(false)
 }
}
