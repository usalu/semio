//! ⚠️ Original operation diagnostics retain byte publication and exact acknowledgement custody.
#[path="🌲️source/🦀️.rs"]
mod source;
use semio_framework_diagnostic::Diagnostic;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection,admit_retained_clone_progress}};
use crate::typed_operation_page::{TypedOperationResultPage,TypedOperationResultToken,TYPED_OPERATION_RESULT_PAGE_BYTES};
use crate::typed_operation_page::TypedOperationResultLane;
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value::retirement::controlled::ControlledRetirement;

/// 📬️ A single original diagnostic stream holds encoded fragments until exact recipient acknowledgement.
pub struct OperationDiagnosticPublication{
 input:ManuallyDrop<Option<Vec<Diagnostic>>>,
 original:Option<RetainedCloneSource<Vec<Diagnostic>>>,
 projected:Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>,
 encoder:Option<ArtifactCanonicalJsonTreeCursor>,
 bytes:ManuallyDrop<Option<Vec<u8>>>,bytes_close:Option<ControlledRetirement<Vec<u8>>>,
 len:usize,count:usize,index:u32,ready:bool,last:bool,acknowledged:bool,
 awaiting:Option<TypedOperationResultToken>,closing:bool,
}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation diagnostic publication has no admitted next action")}
fn source_birth()->RetirementDemand{RetirementDemand{copy_bytes:RetainedCloneSource::<Vec<Diagnostic>>::constructor_copy_bytes(),capacity_bytes:RetainedCloneSource::<Vec<Diagnostic>>::owned_constructor_capacity_bytes::<()>(),depth:1,..Default::default()}}
fn source_close(owner:&RetainedCloneSource<Vec<Diagnostic>>,body:usize)->Result<RetirementDemand,ValueError>{owner.next_close_demands(body)}
fn projection_close(owner:&RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>,body:usize)->Result<RetirementDemand,ValueError>{owner.close_demand(body)}
impl OperationDiagnosticPublication{
 /// 📏️ Small retained framing is admitted before moving the original diagnostic vector.
 pub fn capture_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()*2+size_of::<Vec<Diagnostic>>(),depth:1,..Default::default()}}
 pub fn capture(rows:Vec<Diagnostic>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,Vec<Diagnostic>)>{let d=Self::capture_demand();if !fits(d,grant){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original diagnostic capture frame is not funded"),rows))}Ok((Self{count:rows.len(),input:ManuallyDrop::new(Some(rows)),original:None,projected:None,encoder:None,bytes:ManuallyDrop::new(None),bytes_close:None,len:5,index:0,ready:false,last:false,acknowledged:false,awaiting:None,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn page_ready(&self)->bool{!self.closing&&self.ready&&self.awaiting.is_none()&&!self.acknowledged}
 pub fn terminal_is_empty(&self)->bool{self.input.is_none()&&self.original.is_none()&&self.projected.is_none()&&self.encoder.is_none()&&self.bytes.is_none()&&self.bytes_close.is_none()&&self.awaiting.is_none()&&!self.ready&&self.len==0}
 fn metadata()->usize{size_of::<usize>()+size_of::<u32>()+size_of::<bool>()*3+size_of::<Option<TypedOperationResultToken>>()}
 fn reset_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:Self::metadata()+if self.last&&!self.closing{size_of::<Option<ArtifactCanonicalJsonTreeCursor>>()}else{0},depth:1,..Default::default()}}
 pub fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}
  if self.closing&&(self.len!=0||self.ready||self.awaiting.is_some()||self.acknowledged){return Ok(self.reset_demand())}
  if self.acknowledged{return Ok(self.reset_demand())}
  if self.ready||self.awaiting.is_some(){return Ok(Default::default())}
  if self.closing{if let Some(owner)=self.bytes_close.as_ref(){let mut d=owner.next_demand(body.saturating_sub(size_of::<Option<ControlledRetirement<Vec<u8>>>>()))?;d.copy_bytes+=size_of::<Option<ControlledRetirement<Vec<u8>>>>();return Ok(d)}if self.bytes.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Vec<u8>>>()*2,depth:1,..Default::default()})}}
  else if (self.index as usize)<self.count{if self.bytes.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Vec<u8>>>()*2,capacity_bytes:TYPED_OPERATION_RESULT_PAGE_BYTES,depth:1,..Default::default()})}let initialized=self.bytes.as_ref().unwrap().len();if initialized<TYPED_OPERATION_RESULT_PAGE_BYTES{return Ok(RetirementDemand{copy_bytes:(TYPED_OPERATION_RESULT_PAGE_BYTES-initialized).min(2048)+size_of::<usize>(),depth:1,..Default::default()})}}
  if let Some(cursor)=self.encoder.as_ref(){if self.closing{return if cursor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactCanonicalJsonTreeCursor>>(),depth:1,..Default::default()})}else{cursor.next_demand()}}let mut d=cursor.next_demand()?;d.copy_bytes=d.copy_bytes.checked_add(Self::metadata()).ok_or_else(invalid)?;return Ok(d)}
  if let Some(owner)=self.projected.as_ref(){if self.closing{return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>>(),depth:1,..Default::default()})}else{projection_close(owner,body)}}return Ok(ArtifactCanonicalJsonTreeCursor::constructor_demand())}
  if self.input.is_some(){return Ok(source_birth())}
  if let Some(owner)=self.original.as_ref(){if self.closing||owner.terminal_is_empty()||self.index as usize==self.count{return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<RetainedCloneSource<Vec<Diagnostic>>>>(),depth:1,..Default::default()})}else{source_close(owner,body)}}return Ok(RetirementDemand{copy_bytes:owner.borrow().binding_copy_bytes(),depth:1,..Default::default()})}
  Err(invalid())
 }
 /// 🧵️ Every turn forwards an actual central encoder or original typed-retirement receipt.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.next_demand(grant.maximum_copy_bytes)?;if !fits(d,grant)||self.ready&&!self.acknowledged&&!self.closing||self.awaiting.is_some()&&!self.closing{return Ok(RetainedCloneStep::Progress(Default::default()))}
  let progress=if self.closing&&(self.len!=0||self.ready||self.awaiting.is_some()||self.acknowledged){self.len=0;self.ready=false;self.awaiting=None;self.acknowledged=false;self.last=false;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else if self.acknowledged{if self.last{assert!(self.encoder.as_ref().is_some_and(ArtifactCanonicalJsonTreeCursor::terminal_is_empty));let next=self.index.checked_add(1).ok_or_else(invalid)?;self.encoder=None;self.index=next;}self.len=5;self.ready=false;self.acknowledged=false;self.last=false;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else if self.closing&&self.bytes_close.is_some(){let owner=self.bytes_close.as_mut().unwrap();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<ControlledRetirement<Vec<u8>>>>(),..grant};let mut p=owner.step(child)?.progress();if owner.terminal_is_empty(){self.bytes_close=None;p.copied_bytes+=size_of::<Option<ControlledRetirement<Vec<u8>>>>();}p}
  else if self.closing&&self.bytes.is_some(){let bytes=self.bytes.take().unwrap();self.bytes_close=Some(ControlledRetirement::new(bytes).unwrap_or_else(|_|unreachable!("original page supports typed retirement")));RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else if !self.closing&&(self.index as usize)<self.count&&self.bytes.is_none(){*self.bytes=Some(Vec::with_capacity(TYPED_OPERATION_RESULT_PAGE_BYTES));assert_eq!(self.bytes.as_ref().unwrap().capacity(),TYPED_OPERATION_RESULT_PAGE_BYTES);RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}}
  else if !self.closing&&self.bytes.as_ref().is_some_and(|bytes|bytes.len()<TYPED_OPERATION_RESULT_PAGE_BYTES){let bytes=self.bytes.as_mut().unwrap();let initialized=(TYPED_OPERATION_RESULT_PAGE_BYTES-bytes.len()).min(2048);bytes.resize(bytes.len()+initialized,0);RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
  else if let Some(cursor)=self.encoder.as_mut(){if self.closing{cursor.begin_close();if cursor.terminal_is_empty(){self.encoder=None;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<ArtifactCanonicalJsonTreeCursor>>(),..Default::default()}}else{cursor.close_step(grant)?.progress()}}else{let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::metadata(),..grant};let step=cursor.advance(&mut self.bytes.as_mut().ok_or_else(invalid)?[self.len..],child)?;let last=cursor.is_complete();let ready=self.len+step.written_bytes==TYPED_OPERATION_RESULT_PAGE_BYTES||last;let changed=step.written_bytes!=0||self.last!=last||self.ready!=ready;self.len+=step.written_bytes;self.last=last;self.ready=ready;let mut p=step.ownership.progress();if changed{p.copied_bytes=p.copied_bytes.checked_add(Self::metadata()).ok_or_else(invalid)?;}p}}
  else if let Some(owner)=self.projected.as_mut(){if self.closing{if owner.terminal_is_empty(){self.projected=None;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}else{owner.close_step(grant)?.progress()}}else{let owner=self.projected.take().unwrap();match ArtifactCanonicalJsonTreeCursor::admit(owner,grant){Ok((cursor,p))=>{self.encoder=Some(cursor);p},Err((error,owner))=>{self.projected=Some(owner);return Err(error)}}}}
  else if self.input.is_some(){let original=self.input.take().unwrap();match RetainedCloneSource::admit_owned(original,(),grant){Ok((owner,p))=>{self.original=Some(owner);p},Err((error,original,_))=>{*self.input=Some(original);return Err(error)}}}
  else if let Some(owner)=self.original.as_mut(){if self.closing||owner.terminal_is_empty()||self.index as usize==self.count{if owner.terminal_is_empty(){self.original=None;self.len=0;RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}else{owner.close_step(grant)?.progress()}}else{let index=self.index as usize;let(projected,p)=owner.project_owned(index,|rows|source::DiagnosticTree::original(&rows[index])as&dyn ArtifactCanonicalJsonTree,grant)?;self.projected=Some(projected);p}}
  else{return Err(invalid())};
  let p=admit_retained_clone_progress(grant,progress,"original operation diagnostic publication")?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 pub fn page_demand(&self)->Result<RetirementDemand,ValueError>{if !self.page_ready(){return Err(invalid())}Ok(RetirementDemand{copy_bytes:size_of::<TypedOperationResultPage>()*2+size_of::<Option<Vec<u8>>>()+Self::metadata()+5,depth:1,..Default::default()})}
 pub fn take_page(&mut self,token:TypedOperationResultToken,grant:RetainedCloneGrant)->Result<Option<(TypedOperationResultPage,RetainedCloneProgress)>,ValueError>{let d=self.page_demand()?;if !fits(d,grant){return Ok(None)}let mut bytes=self.bytes.take().ok_or_else(invalid)?;bytes[..4].copy_from_slice(&self.index.to_le_bytes());bytes[4]=u8::from(self.last);let page=TypedOperationResultPage::from_original(token,TypedOperationResultLane::Diagnostic,bytes,self.len);self.awaiting=Some(token);Ok(Some((page,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 /// 🪪️ A stale or duplicate recipient token cannot consume original source custody.
 pub fn acknowledge(&mut self,token:TypedOperationResultToken)->bool{if self.awaiting!=Some(token){return false}self.awaiting=None;self.acknowledged=true;true}
 pub fn begin_close(&mut self){self.closing=true;if let Some(cursor)=self.encoder.as_mut(){cursor.begin_close()}}
 #[cfg(test)]
 fn original_rows(&self)->Option<&[Diagnostic]>{self.input.as_ref().map(|rows|rows.as_slice()).or_else(||self.original.as_ref().and_then(|owner|if owner.terminal_is_empty(){None}else{Some(owner.borrow().get().as_slice())}))}
}
impl Drop for OperationDiagnosticPublication{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"operation diagnostics reached Drop before exact ACK and original retirement")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
