//! 📥️ GLTF extras receive their flat node list through the derived receiver, then rebuild the tree one node per funded turn.
use super::*;
use semio_framework_dsl_record::receiving::owned::*;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,retirement::RetireOwned};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
pub struct GltfJsonReceiving{json:OwnedSlot<Json>,values:OwnedSlot<Vec<Option<GltfJson>>>,output:OwnedSlot<GltfJson>,child:OwnedReceivingSlot<Json>,total:usize,live:usize,phase:u8,closing:bool,failed:bool}
impl Default for GltfJsonReceiving{fn default()->Self{Self{json:Default::default(),values:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),total:0,live:0,phase:0,closing:false,failed:false}}}
impl DslOwnedReceivingField for GltfJson{type Cursor=GltfJsonReceiving;fn receiving_cursor()->Self::Cursor{GltfJsonReceiving::default()}}
fn refuse(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}
fn node_demand(node:&JsonNode)->RetirementDemand{let slot=std::mem::size_of::<GltfJson>()+std::mem::size_of::<usize>()+std::mem::size_of::<String>();let children=node.items.len()+node.members.len();RetirementDemand{copy_bytes:std::mem::size_of::<JsonNode>()+node.text.as_ref().map_or(0,String::len)+children*slot,capacity_bytes:children*slot,depth:1,..Default::default()}}
fn take_child(values:&mut [Option<GltfJson>],total:usize,parent:usize,key:u64)->Result<(usize,GltfJson),ValueError>{
 let key=usize::try_from(key).map_err(|_|refuse("GLTF extras child index exceeds native domain"))?;
 if key<=parent||key>=total{return Err(refuse("GLTF extras child topology differs"))}
 let position=total-1-key;
 let value=values.get_mut(position).and_then(Option::take).ok_or_else(||refuse("GLTF extras child has multiple owners"))?;
 Ok((position,value))
}
fn restore(values:&mut [Option<GltfJson>],positions:Vec<usize>,children:Vec<GltfJson>){for(position,value)in positions.into_iter().zip(children){if let Some(slot)=values.get_mut(position){*slot=Some(value)}}}
fn gather(values:&mut [Option<GltfJson>],total:usize,parent:usize,keys:impl Iterator<Item=u64>,count:usize)->Result<(Vec<usize>,Vec<GltfJson>),ValueError>{
 let mut positions=Vec::with_capacity(count);let mut children=Vec::with_capacity(count);
 for key in keys{match take_child(values,total,parent,key){Ok((position,value))=>{positions.push(position);children.push(value)},Err(error)=>{restore(values,positions,children);return Err(error)}}}
 Ok((positions,children))
}
impl GltfJsonReceiving{
 fn rebuild(&mut self)->Result<(),ValueError>{
  let total=self.total;
  let json=self.json.value.as_mut().ok_or_else(invariant)?;
  let values=self.values.value.as_mut().ok_or_else(invariant)?;
  let index=json.nodes.len().checked_sub(1).ok_or_else(invariant)?;
  let node=json.nodes.last_mut().ok_or_else(invariant)?;
  let consumed;
  let value=match(node.kind,node.boolean,node.number,node.text.is_some(),node.items.is_empty(),node.members.is_empty()){
   (JsonKind::Null,None,None,false,true,true)=>{consumed=0;GltfJson::Null},
   (JsonKind::Boolean,Some(flag),None,false,true,true)=>{consumed=0;GltfJson::Bool(flag)},
   (JsonKind::Number,None,Some(number),false,true,true)=>{consumed=0;GltfJson::Number(number)},
   (JsonKind::String,None,None,true,true,true)=>{consumed=0;GltfJson::String(node.text.take().ok_or_else(invariant)?)},
   (JsonKind::Array,None,None,false,_,true)=>{let(_,children)=gather(values,total,index,node.items.iter().copied(),node.items.len())?;consumed=children.len();node.items.clear();GltfJson::Array(children)},
   (JsonKind::Object,None,None,false,true,_)=>{let(_,children)=gather(values,total,index,node.members.iter().map(|member|member.value),node.members.len())?;consumed=children.len();let members=std::mem::take(&mut node.members);GltfJson::Object(members.into_iter().zip(children).map(|(member,child)|(member.name,child)).collect())},
   _=>return Err(refuse("GLTF extras record contains unrelated variant fields"))
  };
  values.push(Some(value));
  json.nodes.pop();
  self.live=self.live+1-consumed;
  Ok(())
 }
 fn selected(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if self.failed||self.closing{return RetainedWorkOutcome{result:Err(invariant().into()),work:0}}
  if self.phase==6{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let d=match self.demands(source,body){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};
  if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  match self.phase{
   0 if !self.child.is_ready()=>return self.child.step(source,native,body,child_grant(g)),
   1 if self.json.value.is_none()=>return normal(self.child.take_into(&mut self.json.value,child_grant(g))),
   2 if !self.child.terminal_is_empty()=>return post_child_close(self.child.close_step(child_grant(g)),native,g),
   3 if self.values.value.is_none()=>return physical(native.admit_vec(&mut self.values.value,self.total,g)),
   _=>{}
  }
  if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}
  let result=(||{match self.phase{
   0=>self.phase=1,
   1=>self.phase=2,
   2=>{self.total=self.json.value.as_ref().ok_or_else(invariant)?.nodes.len();if self.total==0{return Err(refuse("GLTF extras root is missing"))}self.phase=3},
   3=>self.phase=4,
   4=>{if self.json.value.as_ref().ok_or_else(invariant)?.nodes.is_empty(){self.phase=5}else{self.rebuild()?}},
   5=>{let values=self.values.value.as_mut().ok_or_else(invariant)?;if self.live!=1{return Err(refuse("GLTF extras contains an unowned node"))}let root=values.pop().flatten().ok_or_else(||refuse("GLTF extras root is missing"))?;*self.output.value=Some(root);self.phase=6}
   _=>return Err(invariant())}Ok(receipt(d))})();
  RetainedWorkOutcome{result:result.map_err(|error:ValueError|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>())))),work:1}
 }
}
impl OwnedFieldReceiving<GltfJson>for GltfJsonReceiving{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{
  0 if !self.child.is_ready()=>self.child.demands(source,body).and_then(nested),
  1 if self.json.value.is_none()=>nested(nested(metadata::<()>())?),
  2 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),
  3 if self.values.value.is_none()=>Ok(RetirementDemand{capacity_bytes:self.total.checked_mul(std::mem::size_of::<Option<GltfJson>>()).ok_or_else(invariant)?,depth:1,..Default::default()}),
  4=>match self.json.value.as_ref().and_then(|json|json.nodes.last()){Some(node)=>Ok(node_demand(node)),None=>Ok(metadata::<()>())},
  5=>Ok(payload::<GltfJson>()),
  6=>Ok(Default::default()),
  _=>Ok(metadata::<()>())}}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.selected(source,native,body,g);if turn.result.is_err()&&turn.work>0{self.failed=true}turn}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==6&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<GltfJson>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if !self.is_ready(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}move_into(&mut self.output.value,output,g)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested)}if !self.json.terminal_is_empty(){return self.json.close_demands(body)}if !self.values.terminal_is_empty(){return self.values.close_demands(body)}self.output.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}if !self.child.terminal_is_empty(){return self.child.close_step(child_grant(g))}if !self.json.terminal_is_empty(){return self.json.close_step(g)}if !self.values.terminal_is_empty(){return self.values.close_step(g)}self.output.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.json.terminal_is_empty()&&self.values.terminal_is_empty()&&self.output.terminal_is_empty()}
}
impl semio_framework_dsl_record::DslReceivingField for GltfJson{
 fn receive_value(value:&FieldValue,owner:&mut semio_framework_dsl_record::receiving::NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<Self,semio_framework_dsl_record::receiving::NativeSnapshotFailure>{
  let outcome=<Json as semio_framework_dsl_record::DslReceivingField>::receive_value(value,owner);let work=outcome.work;
  match outcome.result{Ok(json)=>match GltfJson::try_from(json){Ok(value)=>RetainedWorkOutcome{result:Ok(value),work},Err(message)=>RetainedWorkOutcome{result:Err(ValueError::new(ValueRefusalKind::InvalidValue,message).into()),work}},Err(error)=>RetainedWorkOutcome{result:Err(error),work}}
 }
}
