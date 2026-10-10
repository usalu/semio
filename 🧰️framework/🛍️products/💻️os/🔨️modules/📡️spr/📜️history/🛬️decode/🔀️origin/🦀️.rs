//! 🔀️ Original decoded origin cells project into typed recipients while all discarded native storage remains granted.
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ErasedSnapshotRetirement,admit_owned_retirement,owned_retirement_birth_bytes};
use std::mem::{ManuallyDrop,size_of,size_of_val};
use ::protocol::{MutationOrigin,ForeignTarget,SchemaId,PayloadHash};
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original History origin has invalid typed fields")}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn metadata()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
#[path="🔣️json/🦀️.rs"]
mod json;
pub use json::{RetainedHistoryOriginJsonDecode,RetainedHistoryOriginJsonStep};
pub struct RetainedHistoryOriginPartial{
 cells:ManuallyDrop<[Option<DslValue>;10]>,key:ManuallyDrop<Option<String>>,retiring:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
 hash:[u8;32],hash_ready:bool,field:usize,kind:u8,
}
impl RetainedHistoryOriginPartial{
 fn empty(original:DslValue)->Self{let mut cells=std::array::from_fn(|_|None);cells[0]=Some(original);Self{cells:ManuallyDrop::new(cells),key:ManuallyDrop::new(None),retiring:ManuallyDrop::new(None),hash:[0;32],hash_ready:false,field:usize::MAX,kind:0}}
 pub fn original_cell(&self,index:usize)->Option<&DslValue>{self.cells.get(index)?.as_ref()}
 pub fn terminal_is_empty(&self)->bool{self.key.is_none()&&self.retiring.is_none()&&self.cells.iter().all(Option::is_none)}
 fn child_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let Some(child)=self.retiring.as_ref()else{return Ok(Default::default())};
  if child.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(child.as_ref()),depth:2,..Default::default()})}
  Ok(RetirementDemand{copy_bytes:child.next_copy_byte_demand()?,capacity_bytes:child.next_capacity_byte_demand(body)?,release_bytes:child.next_release_byte_demand()?,depth:child.next_depth_demand()?.checked_add(2).ok_or_else(refusal)?})
 }
 fn child_close(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let d=self.child_demands(g.maximum_copy_bytes)?;if !permits(g,d){return Ok(Default::default())}
  let child=self.retiring.as_mut().unwrap();if child.terminal_is_empty(){drop(self.retiring.take());return Ok(RetainedCloneProgress{released_bytes:d.release_bytes,..metadata()})}
  let nested=RetainedCloneGrant{maximum_depth:g.maximum_depth-2,..g};let result=child.close_step(nested);
  let p=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};if !p.fits(nested){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original origin child exceeded its supplied grant").with_retained_progress(p))}result.map(|_|p)
 }
 fn discard_demands(&self,index:usize)->RetirementDemand{RetirementDemand{capacity_bytes:if self.cells[index].is_some(){owned_retirement_birth_bytes::<DslValue>()}else{0},depth:2,..Default::default()}}
 fn discard(&mut self,index:usize,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let d=self.discard_demands(index);if !permits(g,d){return Ok(Default::default())}
  let original=self.cells[index].take().unwrap();let nested=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};
  match admit_owned_retirement(original,nested){Ok((owner,p))=>{*self.retiring=Some(owner);Ok(p)},Err((error,original))=>{self.cells[index]=Some(original);Err(error)}}
 }
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.retiring.is_some(){return self.child_demands(body)}
  if let Some(key)=self.key.as_ref(){return Ok(RetirementDemand{release_bytes:key.capacity(),depth:1,..Default::default()})}
  if let Some(index)=self.cells.iter().position(Option::is_some){return Ok(self.discard_demands(index))}
  Ok(Default::default())
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if !permits(g,self.close_demands(g.maximum_copy_bytes)?){return Ok(Default::default())}
  if self.retiring.is_some(){return self.child_close(g)}
  if let Some(key)=self.key.take(){let bytes=key.capacity();drop(key);return Ok(RetainedCloneProgress{released_bytes:bytes,..metadata()})}
  self.discard(self.cells.iter().position(Option::is_some).unwrap(),g)
 }
}
impl Drop for RetainedHistoryOriginPartial{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original History origin partial abandoned native fields");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.cells);ManuallyDrop::drop(&mut self.key);ManuallyDrop::drop(&mut self.retiring)}}}}
pub struct RetainedHistoryOriginProjection{partial:ManuallyDrop<Option<RetainedHistoryOriginPartial>>,receipt:Option<(RetainedCloneGrant,RetainedCloneProgress)>,closing:bool,ready:bool,closed:bool}
impl RetainedHistoryOriginProjection{
 pub fn admit_original(original:&mut Option<DslValue>,g:RetainedCloneGrant)->Result<Option<Self>,ValueError>{if original.is_none()||g.maximum_items==0||g.maximum_depth==0{return Ok(None)}Ok(Some(Self{partial:ManuallyDrop::new(Some(RetainedHistoryOriginPartial::empty(original.take().unwrap()))),receipt:Some((g,metadata())),closing:false,ready:false,closed:false}))}
 pub fn receipt(&self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt}
 pub fn take_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.receipt.take()}
 fn publish(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{self.receipt=Some((g,p));if !p.fits(g){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original origin receipt exceeded supplied authority").with_retained_progress(p))}Ok(())}
 fn cleanup_index(p:&RetainedHistoryOriginPartial)->Option<usize>{let kept=match p.kind{1=>0,2=>(1<<4)|(1<<5),3=>(1<<7)|(1<<8)|(1<<9),_=>0};(3..10).find(|&index|p.cells[index].is_some()&&(kept&(1<<index)==0))}
 pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.receipt.is_some()||self.closed||self.ready{return Ok(Default::default())}let Some(p)=self.partial.as_ref()else{return Ok(RetirementDemand{depth:1,..Default::default()})};
  if self.closing{return p.close_demands(body)}
  if p.retiring.is_some(){return p.child_demands(body)}
  if let Some(key)=p.key.as_ref(){return Ok(RetirementDemand{release_bytes:key.capacity(),depth:1,..Default::default()})}
  if p.cells[2].is_some()&&p.field==usize::MAX{return Ok(p.discard_demands(2))}
  if let Some(DslValue::Array(values))=p.cells[6].as_ref(){return Ok(RetirementDemand{copy_bytes:usize::from(!values.is_empty()),release_bytes:if values.is_empty(){values.capacity().checked_mul(size_of::<DslValue>()).ok_or_else(refusal)?}else{0},depth:1,..Default::default()})}
  for index in [1,0]{if let Some(DslValue::Object(fields))=p.cells[index].as_ref(){return Ok(RetirementDemand{release_bytes:if fields.is_empty(){fields.capacity().checked_mul(size_of::<(String,DslValue)>()).ok_or_else(refusal)?}else{0},depth:1,..Default::default()})}}
  if let Some(index)=Self::cleanup_index(p){return Ok(p.discard_demands(index))}
  Ok(RetirementDemand{depth:1,..Default::default()})
 }
 pub fn step(&mut self,g:RetainedCloneGrant)->Result<bool,ValueError>{
  if self.receipt.is_some()||self.closing||self.closed||self.ready{return Ok(false)}let d=self.demands(g.maximum_copy_bytes)?;if !permits(g,d){return Ok(false)}
  let p=self.partial.as_mut().ok_or_else(refusal)?;let progress=if p.retiring.is_some(){p.child_close(g)?}
  else if let Some(key)=p.key.take(){let released_bytes=key.capacity();drop(key);RetainedCloneProgress{released_bytes,..metadata()}}
  else if p.cells[2].is_some(){
   if p.field==usize::MAX{p.discard(2,g)?}else{
    let field=p.field;let value=p.cells[2].as_ref().unwrap();
    let valid=match field{1=>matches!(value,DslValue::Object(_)),6=>matches!(value,DslValue::Array(values)if values.len()==32),9=>matches!(value,DslValue::Null|DslValue::String(_)),_=>matches!(value,DslValue::String(_))};if !valid||p.cells[field].is_some(){return Err(refusal())}
    if field==3{p.kind=match value{DslValue::String(s)if s=="owner"=>1,DslValue::String(s)if s=="contributed"=>2,DslValue::String(s)if s=="transaction"=>3,_=>return Err(refusal())}}
    p.cells[field]=p.cells[2].take();p.field=usize::MAX;metadata()
   }
  }else if let Some(DslValue::Array(values))=p.cells[6].as_mut(){
   if values.is_empty(){let Some(DslValue::Array(values))=p.cells[6].take()else{unreachable!()};let released_bytes=values.capacity()*size_of::<DslValue>();drop(values);p.hash_ready=true;RetainedCloneProgress{released_bytes,..metadata()}}
   else{let index=values.len()-1;let byte=match values.last().unwrap(){DslValue::Number(Number::UInt(v))if *v<=255=>*v as u8,DslValue::Number(Number::Int(v))if (0..=255).contains(v)=>*v as u8,_=>return Err(refusal())};values.pop();p.hash[index]=byte;RetainedCloneProgress{copied_bytes:1,..metadata()}}
  }else if let Some(index)=[1,0].into_iter().find(|&index|p.cells[index].is_some()){
   let nested=index==1;let Some(DslValue::Object(fields))=p.cells[index].as_mut()else{return Err(refusal())};
   if fields.is_empty(){let Some(DslValue::Object(fields))=p.cells[index].take()else{unreachable!()};let released_bytes=fields.capacity()*size_of::<(String,DslValue)>();drop(fields);RetainedCloneProgress{released_bytes,..metadata()}}
   else{let(key,value)=fields.pop().unwrap();p.field=if nested{match key.as_str(){"artifactId"=>7,"artifactKind"=>8,"dialect"=>9,_=>usize::MAX}}else{match key.as_str(){"kind"=>3,"pluginId"=>4,"mutationId"=>5,"payloadHash"=>6,"initiator"=>1,_=>usize::MAX}};*p.key=Some(key);p.cells[2]=Some(value);metadata()}
  }else if let Some(index)=Self::cleanup_index(p){p.discard(index,g)?}
  else{let valid=match p.kind{1=>true,2=>p.cells[4].is_some()&&p.cells[5].is_some()&&p.hash_ready,3=>p.cells[7].is_some()&&p.cells[8].is_some(),_=>false};if !valid{return Err(refusal())}self.ready=true;metadata()};
  self.publish(g,progress)?;Ok(true)
 }
 pub fn is_ready(&self)->bool{self.ready&&!self.closing}
 fn text(cell:&mut Option<DslValue>)->String{let Some(DslValue::String(text))=cell.take()else{unreachable!()};text}
 pub fn take_ready_into(&mut self,recipient:&mut Option<MutationOrigin>,g:RetainedCloneGrant)->Result<bool,ValueError>{
  if !self.is_ready()||self.receipt.is_some()||recipient.is_some()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}let p=self.partial.as_mut().ok_or_else(refusal)?;
  *recipient=Some(match p.kind{1=>MutationOrigin::Owner,2=>MutationOrigin::Contributed{plugin_id:Self::text(&mut p.cells[4]),mutation_id:SchemaId(Self::text(&mut p.cells[5])),payload_hash:PayloadHash(p.hash)},3=>MutationOrigin::Transaction{initiator:ForeignTarget{artifact_id:Self::text(&mut p.cells[7]),artifact_kind:Self::text(&mut p.cells[8]),dialect:match p.cells[9].take(){Some(DslValue::String(text))=>Some(text),Some(DslValue::Null)|None=>None,_=>unreachable!()}}},_=>return Err(refusal())});self.ready=false;self.closing=true;self.publish(g,metadata())?;Ok(true)
 }
 pub fn cancel(&mut self){self.closing=true;self.ready=false}
 pub fn take_partial_into(&mut self,recipient:&mut Option<RetainedHistoryOriginPartial>,g:RetainedCloneGrant)->Result<bool,ValueError>{if !self.closing||self.receipt.is_some()||recipient.is_some()||self.partial.is_none()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}*recipient=self.partial.take();self.publish(g,metadata())?;Ok(true)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<bool,ValueError>{
  if !self.closing||self.receipt.is_some()||self.closed{return Ok(false)}
  if self.partial.as_ref().is_some_and(|p|!p.terminal_is_empty()){return Ok(false)}
  if g.maximum_items==0||g.maximum_depth==0{return Ok(false)}drop(self.partial.take());self.closed=true;self.publish(g,metadata())?;Ok(true)
 }
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.partial.is_none()&&self.receipt.is_none()}
}
impl Drop for RetainedHistoryOriginProjection{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original origin projection abandoned partial custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.partial)}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
