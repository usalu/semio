//! 📜️ The next clone effect is quoted from borrowed source headers and the current bounded path.
use super::*;
type Demand=semio_framework_value::RetirementDemand;
fn quote(copy:usize,capacity:usize,release:usize)->Demand{Demand{copy_bytes:copy,capacity_bytes:capacity,release_bytes:release,depth:1,..Default::default()}}
fn text(value:&str,header:usize)->Demand{quote(header+value.len(),value.len(),0)}

impl RasterSnapshotCloneAuthority{
 pub(in super::super) fn demands(&self,source:&RasterSnapshot,body:usize)->Result<Demand,&'static str>{
  Ok(match self.phase{
   1=>text(&source.schema,size_of::<String>()),2=>text(&source.id,size_of::<String>()),3=>source.title.as_ref().map_or(quote(0,0,0),|value|text(value,size_of::<Option<String>>())),
   4=>{let target=self.value.as_ref().ok_or("raster-store.clone-demand-snapshot")?;if self.index==0&&target.layers.capacity()==0{quote(size_of::<Vec<RasterLayerNode>>(),source.layers.len().saturating_add(1)*size_of::<RasterLayerNode>(),0)}else if let Some(layer)=self.layer.as_ref(){if layer.terminal{quote(size_of::<RasterLayerNode>(),0,size_of::<RasterLayerCloneAuthority>())}else{return layer.demands(source.layers.get(self.index).ok_or("raster-store.clone-demand-layer")?,body)}}else if source.layers.get(self.index).is_some(){let bytes=size_of::<RasterLayerCloneAuthority>();quote(bytes,bytes,0)}else{quote(0,0,0)}},
   5=>{let target=self.value.as_ref().ok_or("raster-store.clone-demand-snapshot")?;if let Some((key,_))=self.pending_asset.as_ref(){if self.asset_field>=5{if target.assets.page_required_for_insert(key){let page=RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes();quote(page,page,0)}else{quote(2*size_of::<(String,RasterAssetChild)>(),0,0)}}else{let child=source.assets.get(key).ok_or("raster-store.clone-demand-asset")?;let value=match self.asset_field{0=>&child.child_id,1=>&child.target.artifact_id,2=>&child.target.dialect.artifact_kind,3=>&child.target.dialect.standard,4=>&child.target.dialect.subset,_=>return Err("raster-store.clone-demand-field")};text(value,size_of::<String>())}}else if let Some((key,_))=self.asset_key.next(&source.assets){text(key,size_of::<(String,RasterAssetChild)>())}else{quote(0,0,0)}},
   _=>quote(0,0,0),
  })
 }
}

impl RasterDslValueCloneAuthority{
 pub(in super::super) fn demands(&self,root:&semio_framework_value::DslValue,body:usize)->Result<Demand,&'static str>{
  use semio_framework_value::DslValue as Value;
  if self.terminal{return Ok(quote(0,0,0))}if self.value.is_none(){return Ok(quote(size_of::<Value>(),Self::skeleton_capacity(root),0))}if !self.root_capacity_observed{return Ok(quote(0,0,0))}
  let path=&self.path[..self.depth];let source=RasterDslValueBoundsAuthority::value_at(root,path).ok_or("raster-store.clone-demand-value-source")?;let target=RasterDslValueBoundsAuthority::value_at(self.value.as_ref().ok_or("raster-store.clone-demand-value-target")?,path).ok_or("raster-store.clone-demand-value-path")?;let frame=self.frames[self.depth];
  if frame.phase==0{return Ok(match(source,target){(Value::String(value),_)=>text(value,size_of::<String>()),(Value::Bytes(value),Value::Bytes(target))=>{if target.capacity()==0&&!value.is_empty(){quote(size_of::<Vec<u8>>(),value.len(),0)}else if target.len()<value.len(){let bytes=(value.len()-target.len()).min(body.saturating_sub(size_of::<Vec<u8>>())).min(32768);quote(size_of::<Vec<u8>>()+bytes,0,0)}else{quote(0,0,0)}},_=>quote(0,0,0)})}
  Ok(match source{Value::Array(values)if frame.child<values.len()=>quote(size_of::<Value>(),Self::skeleton_capacity(&values[frame.child]),0),Value::Object(values)if frame.child<values.len()=>{if frame.phase==1{text(&values[frame.child].0,size_of::<String>())}else{quote(size_of::<(String,Value)>(),Self::skeleton_capacity(&values[frame.child].1),0)}},_=>quote(0,0,0)})
 }
}

impl RasterLayerCloneAuthority{
 pub(in super::super) fn demands(&self,root:&RasterLayerNode,body:usize)->Result<Demand,&'static str>{
  if self.terminal||!self.admitted{return Ok(quote(0,0,0))}if self.value.is_none(){return Ok(quote(size_of::<RasterLayerNode>(),Self::skeleton_capacity(root),0))}if !self.root_capacity_observed{return Ok(quote(0,0,0))}
  let path=&self.path[..self.depth];let source=RasterLayerBoundsAuthority::layer_at(root,path).ok_or("raster-store.clone-demand-layer-source")?;let target=RasterLayerBoundsAuthority::layer_at(self.value.as_ref().ok_or("raster-store.clone-demand-layer-target")?,path).ok_or("raster-store.clone-demand-layer-path")?;let frame=self.frames[self.depth];
  Ok(match frame.phase{
   0..=2=>{let(id,name,blend)=match source{RasterLayerNode::Pixel{id,name,blend_mode,..}|RasterLayerNode::Group{id,name,blend_mode,..}|RasterLayerNode::Adjustment{id,name,blend_mode,..}=>(id,name,blend_mode)};text(match frame.phase{0=>id,1=>name,_=>blend},size_of::<String>())},
   3=>match source{RasterLayerNode::Pixel{image_key:Some(key),..}=>text(key,size_of::<Option<String>>()),RasterLayerNode::Adjustment{adjustment_kind,..}=>text(adjustment_kind,size_of::<String>()),_=>quote(0,0,0)},
   4=>if let(RasterLayerNode::Adjustment{params:source,..},RasterLayerNode::Adjustment{params:target,..})=(source,target){if let Some(authority)=self.parameter_value.as_ref(){let(_,value)=self.parameter_key.next(source).ok_or("raster-store.clone-demand-parameter")?;if !authority.terminal{return authority.demands(value,body)}let key=self.pending_parameter_key.as_ref().ok_or("raster-store.clone-demand-parameter-key")?;if target.page_required_for_insert(key){let page=RasterOwnedMap::<semio_framework_value::DslValue>::conservative_page_credit_bytes();quote(page,page,0)}else{quote(2*size_of::<(String,semio_framework_value::DslValue)>(),0,size_of::<RasterDslValueCloneAuthority>())}}else if let Some((key,_))=self.parameter_key.next(source){if self.pending_parameter_key.is_none(){text(key,size_of::<String>())}else{let bytes=size_of::<RasterDslValueCloneAuthority>();quote(bytes,bytes,0)}}else{quote(0,0,0)}}else{quote(0,0,0)},
   5=>if let RasterLayerNode::Group{children,..}=source{children.get(frame.child).map_or(quote(0,0,0),|child|quote(2*size_of::<RasterLayerNode>(),Self::skeleton_capacity(child),0))}else{quote(0,0,0)},
   6=>match source{RasterLayerNode::Pixel{mask:Some(mask),..}|RasterLayerNode::Group{mask:Some(mask),..}=>mask.image_key.as_ref().map_or(quote(0,0,0),|key|text(key,size_of::<Option<String>>())),_=>quote(0,0,0)},
   _=>quote(0,0,0),
  })
 }
}
