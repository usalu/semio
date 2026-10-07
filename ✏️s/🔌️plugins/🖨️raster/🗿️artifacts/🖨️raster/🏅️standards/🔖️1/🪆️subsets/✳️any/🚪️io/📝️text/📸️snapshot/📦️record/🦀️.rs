//! 🖨️ Literal Raster native forest and complete intrinsic parameter entities.
use crate::{RasterSnapshot,RasterLayerNode,RasterLayerMask,RasterTransform,RasterOwnedMap,RasterAssetChild};
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn limit(kind:ValueRefusalKind,message:&str)->ValueError{ValueError::new(kind,message)}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension="raster")]
pub(crate) struct RasterNativeDocument{
 schema:String,
 id:String,
 title:Option<String>,
 layers:Vec<RasterNativeLayer>,
 assets:Vec<RasterNativeAsset>,
 values:Vec<RasterNativeValue>,
 items:Vec<RasterNativeItem>,
 members:Vec<RasterNativeMember>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeAsset{key:String,child:RasterAssetChild}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeLayer{
 parent:Option<u64>,
 ordinal:u64,
 kind:String,
 id:String,
 name:String,
 visible:bool,
 locked:bool,
 opacity:f32,
 blend:String,
 #[dsl(block)]
 transform:RasterTransform,
 #[dsl(block)]
 mask:Option<RasterLayerMask>,
 width:Option<u32>,
 height:Option<u32>,
 image:Option<String>,
 adjustment:Option<String>,
 parameters:Vec<RasterNativeParameter>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeParameter{key:String,root:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeValue{
 kind:String,
 boolean:Option<bool>,
 signed:Option<i64>,
 unsigned:Option<u64>,
 float:Option<f64>,
 text:Option<String>,
 #[dsl(base64)]
 bytes:Option<Vec<u8>>,
}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeItem{array:u64,ordinal:u64,value:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct RasterNativeMember{object:u64,ordinal:u64,key:String,value:u64}
#[derive(Default)]
struct Plan{layers:usize,values:usize,items:usize,members:usize,parameters:usize,rows:usize}
impl Plan{
 fn add(&mut self,count:usize,maximum:usize)->Result<(),ValueError>{self.rows=self.rows.checked_add(count).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster row count overflow"))?;if self.rows>maximum{return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster domain exceeds row limit"))}Ok(())}
 fn units(&self,assets:usize)->Result<usize,ValueError>{[self.layers,self.values,self.items,self.members,self.parameters,assets].into_iter().try_fold(0usize,|total,n|total.checked_add(n).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native workload overflow")))}
}
fn push<T>(pending:&mut Vec<T>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{if pending.len()==pending.capacity(){control.charge(std::mem::size_of::<T>())?;pending.try_reserve_exact(1).map_err(|_|limit(ValueRefusalKind::AllocationFailed,"Raster borrowed frontier allocation"))?;}pending.push(value);Ok(())}
fn forecast(s:&RasterSnapshot,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<Plan,ValueError>{
 control.begin_stage(0)?;let mut plan=Plan::default();plan.add(1,maximum)?;plan.add(s.assets.len(),maximum)?;if s.assets.len()>crate::RASTER_OWNED_MAP_CAPACITY{return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster asset map capacity"))}
 if s.layers.len()>maximum.saturating_sub(plan.rows){return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster root forest exceeds row limit"))}let mut layers=Vec::new();for layer in s.layers.iter().rev(){plan.add(0,maximum)?;push(&mut layers,layer,control)?;}
 while let Some(node)=layers.pop(){plan.layers=plan.layers.checked_add(1).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster layer count overflow"))?;plan.add(10,maximum)?;control.step()?;
 match node{
 RasterLayerNode::Pixel{mask,..}=>{if mask.is_some(){plan.add(8,maximum)?;}},
 RasterLayerNode::Group{children,mask,..}=>{if mask.is_some(){plan.add(8,maximum)?;}if children.len()>maximum.saturating_sub(plan.rows){return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster borrowed layer frontier exceeds row limit"))}for child in children.iter().rev(){push(&mut layers,child,control)?;}},
 RasterLayerNode::Adjustment{params,..}=>{if params.len()>crate::RASTER_OWNED_MAP_CAPACITY{return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster parameter map capacity"))}plan.parameters=plan.parameters.checked_add(params.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster parameter count overflow"))?;plan.add(params.len(),maximum)?;
 for(_,root)in params.iter(){let mut pending=Vec::new();push(&mut pending,root,control)?;while let Some(value)=pending.pop(){plan.values=plan.values.checked_add(1).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster intrinsic count overflow"))?;plan.add(match value{semio_framework_value::DslValue::Null=>1,semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(_))=>3,_=>2},maximum)?;control.step()?;match value{
 semio_framework_value::DslValue::Array(values)=>{plan.items=plan.items.checked_add(values.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster item count overflow"))?;plan.add(values.len(),maximum)?;if pending.len().checked_add(values.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster frontier overflow"))?>maximum.saturating_sub(plan.rows){return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster intrinsic frontier exceeds row limit"))}for child in values.iter().rev(){push(&mut pending,child,control)?;}},
 semio_framework_value::DslValue::Object(values)=>{plan.members=plan.members.checked_add(values.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster member count overflow"))?;plan.add(values.len(),maximum)?;if pending.len().checked_add(values.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster frontier overflow"))?>maximum.saturating_sub(plan.rows){return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster intrinsic frontier exceeds row limit"))}for(_,child)in values.iter().rev(){push(&mut pending,child,control)?;}},_=>{}}
 }}},
 }
 }Ok(plan)
}
fn mask_copy(mask:&RasterLayerMask,control:&mut NativeEncodeControl<'_>)->Result<RasterLayerMask,ValueError>{Ok(RasterLayerMask{enabled:mask.enabled,linked:mask.linked,invert:mask.invert,width:mask.width,height:mask.height,image_key:mask.image_key.as_deref().map(|v|control.copy_text(v)).transpose()?,transform:mask.transform.clone()})}
impl RasterNativeDocument{
 pub(crate) fn from_snapshot(s:&RasterSnapshot,maximum_rows:usize,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{
 let plan=forecast(s,maximum_rows,control)?;control.begin_stage(plan.units(s.assets.len())?)?;
 let mut output=Self{schema:control.copy_text(&s.schema)?,id:control.copy_text(&s.id)?,title:s.title.as_deref().map(|v|control.copy_text(v)).transpose()?,layers:control.allocate_vec(plan.layers)?,assets:control.allocate_vec(s.assets.len())?,values:control.allocate_vec(plan.values)?,items:control.allocate_vec(plan.items)?,members:control.allocate_vec(plan.members)?};
 for(key,child)in s.assets.iter(){let d=&child.target.dialect;output.assets.push(RasterNativeAsset{key:control.copy_text(key)?,child:store::ArtifactChild::new(control.copy_text(&child.child_id)?,semio_framework_artifact_reference::ArtifactRef{dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:control.copy_text(&d.artifact_kind)?,standard:control.copy_text(&d.standard)?,subset:control.copy_text(&d.subset)?},artifact_id:control.copy_text(&child.target.artifact_id)?})});control.step()?;}
 let mut pending=control.allocate_vec(plan.layers)?;for(i,node)in s.layers.iter().enumerate().rev(){pending.push((node,None,i));}
 while let Some((node,parent,ordinal))=pending.pop(){let(kind,id,name,visible,locked,opacity,blend,transform,mask)=match node{
 RasterLayerNode::Pixel{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("pixel",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Group{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("group",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Adjustment{id,name,visible,locked,opacity,blend_mode,transform,..}=>("adjustment",id,name,visible,locked,opacity,blend_mode,transform,None)};
 let layer_id=u64::try_from(output.layers.len()+1).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?;
 let mut layer=RasterNativeLayer{parent,ordinal:u64::try_from(ordinal).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?,kind:control.copy_text(kind)?,id:control.copy_text(id)?,name:control.copy_text(name)?,visible:*visible,locked:*locked,opacity:*opacity,blend:control.copy_text(blend)?,transform:transform.clone(),mask:mask.map(|v|mask_copy(v,control)).transpose()?,width:None,height:None,image:None,adjustment:None,parameters:Vec::new()};
 match node{
 RasterLayerNode::Pixel{width,height,image_key,..}=>{layer.width=*width;layer.height=*height;layer.image=image_key.as_deref().map(|v|control.copy_text(v)).transpose()?;},
 RasterLayerNode::Group{children,..}=>{for(i,child)in children.iter().enumerate().rev(){pending.push((child,Some(layer_id),i));}},
 RasterLayerNode::Adjustment{adjustment_kind,params,..}=>{layer.adjustment=Some(control.copy_text(adjustment_kind)?);layer.parameters=control.allocate_vec(params.len())?;for(key,value)in params.iter(){let root=output.intrinsic(value,plan.values,control)?;layer.parameters.push(RasterNativeParameter{key:control.copy_text(key)?,root});control.step()?;}},
 }output.layers.push(layer);control.step()?;
 }Ok(output)
 }
 fn intrinsic(&mut self,root:&semio_framework_value::DslValue,_maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<u64,ValueError>{
 enum Edge<'a>{Item(u64,u64),Member(u64,u64,&'a str)}
 let mut pending=Vec::new();push(&mut pending,(root,None),control)?;let root_id=u64::try_from(self.values.len()+1).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?;
 while let Some((value,edge))=pending.pop(){let id=u64::try_from(self.values.len()+1).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?;let kind=match value{semio_framework_value::DslValue::Null=>"null",semio_framework_value::DslValue::Bool(_)=>"boolean",semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_))=>"signed",semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_))=>"unsigned",semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(_))=>"float",semio_framework_value::DslValue::String(_)=>"text",semio_framework_value::DslValue::Bytes(_)=>"bytes",semio_framework_value::DslValue::Array(_)=>"array",semio_framework_value::DslValue::Object(_)=>"object"};let mut row=RasterNativeValue{kind:control.copy_text(kind)?,boolean:None,signed:None,unsigned:None,float:None,text:None,bytes:None};
 match value{semio_framework_value::DslValue::Null=>{},semio_framework_value::DslValue::Bool(v)=>row.boolean=Some(*v),semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(v))=>row.signed=Some(*v),semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(v))=>row.unsigned=Some(*v),semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(v))=>row.float=Some(*v),semio_framework_value::DslValue::String(v)=>row.text=Some(control.copy_text(v)?),semio_framework_value::DslValue::Bytes(v)=>row.bytes=Some(control.copy_bytes(v)?),semio_framework_value::DslValue::Array(v)=>{for(i,child)in v.iter().enumerate().rev(){push(&mut pending,(child,Some(Edge::Item(id,u64::try_from(i).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?))),control)?;}},semio_framework_value::DslValue::Object(v)=>{for(i,(key,child))in v.iter().enumerate().rev(){push(&mut pending,(child,Some(Edge::Member(id,u64::try_from(i).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?,key))),control)?;}}}
 self.values.push(row);control.step()?;match edge{None=>{},Some(Edge::Item(array,ordinal))=>{self.items.push(RasterNativeItem{array,ordinal,value:id});control.step()?;},Some(Edge::Member(object,ordinal,key))=>{self.members.push(RasterNativeMember{object,ordinal,key:control.copy_text(key)?,value:id});control.step()?;}}
 }Ok(root_id)
 }
}
fn index(id:u64,count:usize)->Result<usize,ValueError>{let id=usize::try_from(id).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?;if id==0||id>count{return Err(invalid("Raster native dangling identity"))}Ok(id-1)}
struct Values(Vec<Option<semio_framework_value::DslValue>>);
impl Drop for Values{fn drop(&mut self){for value in &mut self.0{if let Some(value)=value.take(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}}}}
struct Layers(Vec<Option<RasterLayerNode>>);
impl Drop for Layers{fn drop(&mut self){for node in &mut self.0{if let Some(node)=node.take(){crate::retire_raster_layers(vec![node]);}}}}
struct Snapshot(Option<RasterSnapshot>);
impl Drop for Snapshot{fn drop(&mut self){if let Some(s)=self.0.take(){crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(s);}}}
struct Parameters(RasterOwnedMap<semio_framework_value::DslValue>);
impl Drop for Parameters{fn drop(&mut self){while let Some((_,value))=self.0.take_last_entry(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}self.0.retire();}}
fn slots<T>(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Vec<Option<T>>,ValueError>{let mut slots=control.allocate_vec(count)?;slots.resize_with(count,||None);Ok(slots)}
fn keys_before(a:&str,b:&str,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{control.scoped_stage(|control| -> Result<bool,ValueError>{let length=a.len().min(b.len());control.begin_stage(length)?;let mut offset=0;while offset<length{let end=offset.saturating_add(65536).min(length);let order=a.as_bytes()[offset..end].cmp(&b.as_bytes()[offset..end]);control.advance(end-offset)?;if order!=std::cmp::Ordering::Equal{return Ok(order==std::cmp::Ordering::Less)}offset=end;}Ok(a.len()<b.len())})}
impl RasterNativeDocument{
 pub(crate) fn into_snapshot(self,maximum_rows:usize,control:&mut NativeDecodeControl<'_>)->Result<RasterSnapshot,ValueError>{
 let mut rows=1usize.checked_add(self.assets.len()).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native row overflow"))?;
 for row in &self.layers{rows=rows.checked_add(10+usize::from(row.mask.is_some())*8).and_then(|v|v.checked_add(row.parameters.len())).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native row overflow"))?;}
 for row in &self.values{rows=rows.checked_add(match row.kind.as_str(){"null"=>1,"float"=>3,_=>2}).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native row overflow"))?;}
 rows=rows.checked_add(self.items.len()).and_then(|v|v.checked_add(self.members.len())).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native row overflow"))?;if rows>maximum_rows{return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster native domain exceeds row limit"))}
 let total=[self.layers.len(),self.values.len(),self.items.len(),self.members.len(),self.assets.len()].into_iter().try_fold(0usize,|total,count|total.checked_add(count).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster native work overflow")))?;
 control.begin_stage(total)?;let value_count=self.values.len();let layer_count=self.layers.len();
 let mut items_count=control.allocate_vec::<usize>(value_count)?;items_count.resize(value_count,0);
 let mut members_count=control.allocate_vec::<usize>(value_count)?;members_count.resize(value_count,0);
 for row in &self.items{let parent=index(row.array,value_count)?;let child=index(row.value,value_count)?;if child<=parent||self.values[parent].kind!="array"{return Err(invalid("Raster native array ownership"))}items_count[parent]=items_count[parent].checked_add(1).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster item count overflow"))?;control.checkpoint()?;}
 for row in &self.members{let parent=index(row.object,value_count)?;let child=index(row.value,value_count)?;if child<=parent||self.values[parent].kind!="object"{return Err(invalid("Raster native object ownership"))}members_count[parent]=members_count[parent].checked_add(1).ok_or_else(||limit(ValueRefusalKind::WorkLimit,"Raster member count overflow"))?;control.checkpoint()?;}
 let mut item_groups=control.allocate_vec::<Vec<RasterNativeItem>>(value_count)?;let mut member_groups=control.allocate_vec::<Vec<RasterNativeMember>>(value_count)?;for count in items_count{item_groups.push(control.allocate_vec(count)?);}for count in members_count{member_groups.push(control.allocate_vec(count)?);}
 for row in self.items{let parent=index(row.array,value_count)?;if usize::try_from(row.ordinal).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?!=item_groups[parent].len(){return Err(invalid("Raster native array order"))}item_groups[parent].push(row);control.step()?;}
 for row in self.members{let parent=index(row.object,value_count)?;if usize::try_from(row.ordinal).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?!=member_groups[parent].len(){return Err(invalid("Raster native object order"))}member_groups[parent].push(row);control.step()?;}
 let mut values=Values(slots(value_count,control)?);
 for(i,row)in self.values.into_iter().enumerate().rev(){let present=usize::from(row.boolean.is_some())+usize::from(row.signed.is_some())+usize::from(row.unsigned.is_some())+usize::from(row.float.is_some())+usize::from(row.text.is_some())+usize::from(row.bytes.is_some());let composite=matches!(row.kind.as_str(),"null"|"array"|"object");if present!=usize::from(!composite){return Err(invalid("Raster intrinsic variant fields"))}
 let value=match row.kind.as_str(){"null"=>semio_framework_value::DslValue::Null,"boolean"=>semio_framework_value::DslValue::Bool(row.boolean.ok_or_else(||invalid("Raster boolean body"))?),"signed"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(row.signed.ok_or_else(||invalid("Raster signed body"))?)),"unsigned"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(row.unsigned.ok_or_else(||invalid("Raster unsigned body"))?)),"float"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(row.float.ok_or_else(||invalid("Raster float body"))?)),"text"=>semio_framework_value::DslValue::String(row.text.ok_or_else(||invalid("Raster text body"))?),"bytes"=>semio_framework_value::DslValue::Bytes(row.bytes.ok_or_else(||invalid("Raster bytes body"))?),
 "array"=>{let rows=std::mem::take(&mut item_groups[i]);let mut output=Values(slots(rows.len(),control)?);for(j,row)in rows.into_iter().enumerate(){output.0[j]=Some(values.0[index(row.value,value_count)?].take().ok_or_else(||invalid("Raster duplicate array child"))?);control.checkpoint()?;}let mut result=control.allocate_vec(output.0.len())?;for value in &mut output.0{result.push(value.take().ok_or_else(||invalid("Raster array child"))?);}semio_framework_value::DslValue::Array(result)},
 "object"=>{let rows=std::mem::take(&mut member_groups[i]);let mut result=EncodedMembers(control.allocate_vec(rows.len())?);for row in rows{let value=values.0[index(row.value,value_count)?].take().ok_or_else(||invalid("Raster duplicate object child"))?;result.0.push((row.key,value));control.checkpoint()?;}semio_framework_value::DslValue::Object(std::mem::take(&mut result.0))},_=>return Err(invalid("Raster intrinsic variant"))};values.0[i]=Some(value);control.step()?;
 }
 let mut child_counts=control.allocate_vec::<usize>(layer_count)?;child_counts.resize(layer_count,0);let mut roots_count=0;
 for(i,row)in self.layers.iter().enumerate(){if let Some(parent)=row.parent{let parent=index(parent,layer_count)?;if parent>=i||self.layers[parent].kind!="group"{return Err(invalid("Raster layer parent ownership"))}child_counts[parent]+=1;}else{roots_count+=1;}control.checkpoint()?;}
 let mut children=control.allocate_vec::<Vec<usize>>(layer_count)?;for count in child_counts{children.push(control.allocate_vec(count)?);}let mut root_ids=control.allocate_vec(roots_count)?;
 for(i,row)in self.layers.iter().enumerate(){let target=if let Some(parent)=row.parent{&mut children[index(parent,layer_count)?]}else{&mut root_ids};if usize::try_from(row.ordinal).map_err(|_|invalid("Raster native ordinal exceeds the platform word"))?!=target.len(){return Err(invalid("Raster layer forest order"))}target.push(i);control.checkpoint()?;}
 let mut layers=Layers(slots(layer_count,control)?);
 for(i,row)in self.layers.into_iter().enumerate().rev(){let node=match row.kind.as_str(){
 "pixel"=>{if row.adjustment.is_some()||!row.parameters.is_empty(){return Err(invalid("Raster pixel variant fields"))}RasterLayerNode::Pixel{id:row.id,name:row.name,visible:row.visible,locked:row.locked,opacity:row.opacity,blend_mode:row.blend,transform:row.transform,mask:row.mask,width:row.width,height:row.height,image_key:row.image}},
 "group"=>{if row.width.is_some()||row.height.is_some()||row.image.is_some()||row.adjustment.is_some()||!row.parameters.is_empty(){return Err(invalid("Raster group variant fields"))}let mut owned=Layers(slots(children[i].len(),control)?);for(j,child)in std::mem::take(&mut children[i]).into_iter().enumerate(){owned.0[j]=Some(layers.0[child].take().ok_or_else(||invalid("Raster duplicate layer child"))?);control.checkpoint()?;}let mut output=control.allocate_vec(owned.0.len())?;for node in &mut owned.0{output.push(node.take().ok_or_else(||invalid("Raster group child"))?);}RasterLayerNode::Group{id:row.id,name:row.name,visible:row.visible,locked:row.locked,opacity:row.opacity,blend_mode:row.blend,transform:row.transform,mask:row.mask,children:output}},
 "adjustment"=>{if row.width.is_some()||row.height.is_some()||row.image.is_some()||row.mask.is_some()||row.parameters.len()>crate::RASTER_OWNED_MAP_CAPACITY{return Err(invalid("Raster adjustment variant fields"))}let mut params=Parameters(RasterOwnedMap::new());for parameter in row.parameters{if let Some((previous,_))=params.0.entry_at(params.0.len().saturating_sub(1)){if !keys_before(previous,&parameter.key,control)?{return Err(invalid("Raster parameter native key order"))}}if params.0.page_required_for_insert(&parameter.key){control.charge(RasterOwnedMap::<semio_framework_value::DslValue>::conservative_page_credit_bytes())?;params.0.admit_one_page().map_err(|reason|limit(ValueRefusalKind::OwnershipLimit,reason))?;}let value=values.0[index(parameter.root,value_count)?].take().ok_or_else(||invalid("Raster multiply owned parameter"))?;if let Err(rejected)=params.0.insert_pre_admitted(parameter.key,value){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(rejected.value);return Err(invalid(rejected.reason))}control.checkpoint()?;}RasterLayerNode::Adjustment{id:row.id,name:row.name,visible:row.visible,locked:row.locked,opacity:row.opacity,blend_mode:row.blend,transform:row.transform,adjustment_kind:row.adjustment.ok_or_else(||invalid("Raster adjustment kind"))?,params:std::mem::take(&mut params.0)}},_=>return Err(invalid("Raster layer native variant"))};layers.0[i]=Some(node);control.step()?;
 }
 let mut snapshot=Snapshot(Some(RasterSnapshot{schema:self.schema,id:self.id,title:self.title,layers:control.allocate_vec(root_ids.len())?,assets:RasterOwnedMap::new()}));for id in root_ids{snapshot.0.as_mut().unwrap().layers.push(layers.0[id].take().ok_or_else(||invalid("Raster root layer"))?);control.checkpoint()?;}
 if self.assets.len()>crate::RASTER_OWNED_MAP_CAPACITY{return Err(limit(ValueRefusalKind::OwnershipLimit,"Raster asset native map capacity"))}for asset in self.assets{let map=&mut snapshot.0.as_mut().unwrap().assets;if let Some((previous,_))=map.entry_at(map.len().saturating_sub(1)){if !keys_before(previous,&asset.key,control)?{return Err(invalid("Raster asset native key order"))}}if map.page_required_for_insert(&asset.key){control.charge(RasterOwnedMap::<RasterAssetChild>::conservative_page_credit_bytes())?;map.admit_one_page().map_err(|reason|limit(ValueRefusalKind::OwnershipLimit,reason))?;}map.insert_pre_admitted(asset.key,asset.child).map_err(|e|invalid(e.reason))?;control.step()?;}
 if values.0.iter().any(Option::is_some)||layers.0.iter().any(Option::is_some){return Err(invalid("Raster orphan native entity"))}Ok(snapshot.0.take().unwrap())
 }
 pub(crate) fn ordinary(s:&RasterSnapshot)->Self{Self::from_snapshot(s,usize::MAX,&mut semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut |_|true)).expect("literal Raster native projection")}
 pub(crate) fn ordinary_snapshot(self)->Result<RasterSnapshot,ValueError>{self.into_snapshot(usize::MAX,&mut semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut |_|true))}
}
struct EncodedMembers(Vec<(String,semio_framework_value::DslValue)>);
impl Drop for EncodedMembers{fn drop(&mut self){while let Some((_,value))=self.0.pop(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}}}
