//! 🏗️ Original typed fields enter one final native Tick frame without an assembled Tick copy.
use crate::{ToolRunCounter,ToolRunIdentity,ToolRunIdentityReceiving,ToolRunReceivingLimits,ToolRunState,ToolRunStep,ToolRunTraceOp};
use pack::record::RetainedRecordBodySourceDecode;
use semio_framework_value::{list::PagedList,paged::PagedBytes,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use std::{mem::{ManuallyDrop,MaybeUninit,size_of},ptr::addr_of_mut};

pub struct ToolRunReceivedProgress{pub identity:ToolRunIdentity,pub sequence:u64,pub state:ToolRunState,pub stage:u16,pub completed:u64,pub total:Option<u64>,pub counters:PagedList<ToolRunCounter,8>,pub units_per_second:f32,pub conflicts:u32,pub steps:PagedList<ToolRunStep,64>}
pub struct ToolRunReceivedTracePage{pub identity:ToolRunIdentity,pub page:u32,pub ops:PagedList<ToolRunTraceOp,4096>}
pub struct ToolRunReceivedTick{
 identity:ToolRunIdentity,sequence:u64,progress:Option<ToolRunReceivedProgress>,steps:PagedList<ToolRunStep,262144>,trace:PagedList<ToolRunReceivedTracePage,262144>,append_ops:PagedList<PagedBytes<262144>,262144>,append_entities:PagedList<u64,262144>,retract_to:Option<u32>,payload:Option<PagedBytes<262144>>,
}
type Receipt=Option<(RetainedCloneGrant,RetainedCloneProgress)>;
type Witness=(usize,usize,Option<(u64,u64)>,(usize,usize));
fn original(source:&RetainedRecordBodySourceDecode)->Witness{let s=source.source_identity();(s.address,s.extent,s.authority,source.original_range())}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original final native Tick source or recipient is invalid")}
fn funded(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_depth>=d.depth&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes}
fn metadata()->RetirementDemand{RetirementDemand{depth:2,..Default::default()}}
pub struct ToolRunNativeTickPreparation{
 frame:ManuallyDrop<Option<Box<MaybeUninit<ToolRunReceivedTick>>>>,source:Witness,limits:ToolRunReceivingLimits,headers:u8,identity_at:u8,identity_owner:Option<usize>,sequence_received:bool,entity_issuer:bool,receipt:Receipt,closing:bool,closed:bool,
}
pub struct ToolRunReceivedTickSource{frame:ManuallyDrop<Option<Box<MaybeUninit<ToolRunReceivedTick>>>>,source:Witness,limits:ToolRunReceivingLimits,headers:u8,receipt:Receipt,closing:bool,closed:bool}
impl ToolRunReceivedTick{
 pub fn identity(&self)->&ToolRunIdentity{&self.identity}
 pub fn sequence(&self)->u64{self.sequence}
 pub fn steps(&self)->&PagedList<ToolRunStep,262144>{&self.steps}
 pub fn trace(&self)->&PagedList<ToolRunReceivedTracePage,262144>{&self.trace}
 pub fn append_ops(&self)->&PagedList<PagedBytes<262144>,262144>{&self.append_ops}
 pub fn append_entities(&self)->&PagedList<u64,262144>{&self.append_entities}
 pub fn progress(&self)->Option<&ToolRunReceivedProgress>{self.progress.as_ref()}
 pub fn retract_to(&self)->Option<u32>{self.retract_to}
 pub fn payload(&self)->Option<&PagedBytes<262144>>{self.payload.as_ref()}
}
impl ToolRunNativeTickPreparation{
 pub const fn birth_bytes()->usize{size_of::<ToolRunReceivedTick>()}
 pub fn admit(source:&RetainedRecordBodySourceDecode,limits:ToolRunReceivingLimits,g:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
  limits.validate()?;if source.original_range().1-source.original_range().0>limits.tick.maximum_file_bytes as usize{return Err(fault())}if source.allocated_bytes().checked_add(Self::birth_bytes()).is_none_or(|bytes|bytes>limits.tick.maximum_total_allocation_bytes as usize){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original final native Tick frame exceeds captured allocation ceiling"))}if source.receipt().is_some()||!funded(g,RetirementDemand{capacity_bytes:Self::birth_bytes(),..metadata()}){return Ok(None)}
  let frame=Box::new_uninit();let p=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:Self::birth_bytes(),..Default::default()};Ok(Some((Self{frame:ManuallyDrop::new(Some(frame)),source:original(source),limits,headers:0,identity_at:0,identity_owner:None,sequence_received:false,entity_issuer:false,receipt:Some((g,p)),closing:false,closed:false},p)))
 }
 pub fn native_address(&self)->Option<usize>{self.frame.as_ref().map(|f|f.as_ptr()as usize)}
 pub(crate) fn entity_limits(&self)->ToolRunReceivingLimits{self.limits}
 pub(crate) fn begin_entity_issuer(&mut self,source:&RetainedRecordBodySourceDecode)->Result<bool,ValueError>{if original(source)!=self.source{return Err(fault())}if self.receipt.is_some()||source.receipt().is_some()||self.closing||self.closed||self.headers!=9||self.entity_issuer{return Ok(false)}self.entity_issuer=true;Ok(true)}
 pub(crate) fn entity_column(&self,source:&RetainedRecordBodySourceDecode,address:usize)->Result<&PagedList<u64,262144>,ValueError>{if original(source)!=self.source||self.native_address()!=Some(address)||!self.entity_issuer||self.headers!=9||self.receipt.is_some(){return Err(fault())}let target=self.frame.as_ref().ok_or_else(fault)?.as_ptr();Ok(unsafe{&*std::ptr::addr_of!((*target).append_entities)})}
 pub(crate) fn entity_column_mut(&mut self,source:&RetainedRecordBodySourceDecode,address:usize)->Result<&mut PagedList<u64,262144>,ValueError>{self.entity_column(source,address)?;let target=self.frame.as_mut().ok_or_else(fault)?.as_mut_ptr();Ok(unsafe{&mut *addr_of_mut!((*target).append_entities)})}
 pub(crate) fn end_entity_issuer(&mut self,source:&RetainedRecordBodySourceDecode,address:usize)->Result<(),ValueError>{self.entity_column(source,address)?;self.entity_issuer=false;Ok(())}
 pub fn receipt(&self)->Receipt{self.receipt}
 pub fn take_receipt(&mut self)->Receipt{self.receipt.take()}
 pub fn initialize_header_step(&mut self,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{
  if self.receipt.is_some()||self.closing||self.headers==9||!funded(g,metadata()){return None}let target=self.frame.as_mut()?.as_mut_ptr();unsafe{match self.headers{0=>addr_of_mut!((*target).steps).write(PagedList::empty()),1=>addr_of_mut!((*target).trace).write(PagedList::empty()),2=>addr_of_mut!((*target).append_ops).write(PagedList::empty()),3=>addr_of_mut!((*target).append_entities).write(PagedList::empty()),4=>addr_of_mut!((*target).payload).write(None),5=>addr_of_mut!((*target).progress).write(None),6=>addr_of_mut!((*target).retract_to).write(None),7=>addr_of_mut!((*target).sequence).write(0),8=>addr_of_mut!((*target).identity.base_revision).write([0;32]),_=>unreachable!()}}
  self.headers+=1;let p=RetainedCloneProgress{copied_items:1,..Default::default()};self.receipt=Some((g,p));Some(p)
 }
 pub fn identity_demands(&self)->RetirementDemand{if self.receipt.is_some()||self.closing||self.headers!=9||self.identity_at>=36{return Default::default()}RetirementDemand{copy_bytes:match self.identity_at{0|2=>4,1=>8,3..=34=>1,_=>0},..metadata()}}
 pub fn receive_identity_step(&mut self,child:&mut ToolRunIdentityReceiving,g:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
  if child.source_identity()!=self.source{return Err(fault())}let address=child as *mut ToolRunIdentityReceiving as usize;if self.identity_owner.is_some_and(|p|p!=address){return Err(fault())}
  if self.receipt.is_some()||child.receipt().is_some()||self.closing||self.entity_issuer||self.headers!=9||self.identity_at>=36||child.identity().is_none()||!funded(g,self.identity_demands()){return Ok(None)}
  let target=self.frame.as_mut().ok_or_else(fault)?.as_mut_ptr();let copied=self.identity_demands().copy_bytes;
  if self.identity_at==35{let Some(p)=child.retire_identity_step(g)else{return Ok(None)};if child.take_receipt()!=Some((g,p)){return Err(fault())}}
  else{let identity=child.identity().ok_or_else(fault)?;unsafe{match self.identity_at{0=>addr_of_mut!((*target).identity.id.app_instance_id).write(identity.id.app_instance_id),1=>addr_of_mut!((*target).identity.id.run).write(identity.id.run),2=>addr_of_mut!((*target).identity.generation).write(identity.generation),at=>addr_of_mut!((*target).identity.base_revision).cast::<u8>().add(at as usize-3).write(identity.base_revision[at as usize-3])}}}
  self.identity_owner=Some(address);self.identity_at+=1;let p=RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()};self.receipt=Some((g,p));Ok(Some(p))
 }
 pub fn receive_sequence(&mut self,recipient:&mut Option<u64>,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{
  if self.receipt.is_some()||self.closing||self.entity_issuer||self.headers!=9||self.sequence_received||recipient.is_none()||!funded(g,RetirementDemand{copy_bytes:8,..metadata()}){return None}let target=self.frame.as_mut()?.as_mut_ptr();unsafe{addr_of_mut!((*target).sequence).write(recipient.take()?)}self.sequence_received=true;let p=RetainedCloneProgress{copied_items:1,copied_bytes:8,..Default::default()};self.receipt=Some((g,p));Some(p)
 }
 pub fn publish_into(&mut self,recipient:&mut Option<ToolRunReceivedTickSource>,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{
  if self.receipt.is_some()||self.closing||self.entity_issuer||self.headers!=9||self.identity_at!=36||!self.sequence_received||recipient.is_some()||!funded(g,metadata()){return None}
  let p=RetainedCloneProgress{copied_items:1,..Default::default()};*recipient=Some(ToolRunReceivedTickSource{frame:ManuallyDrop::new(self.frame.take()),source:self.source,limits:self.limits,headers:9,receipt:Some((g,p)),closing:false,closed:false});self.headers=0;self.closed=true;Some(p)
 }
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_demands(&self)->RetirementDemand{if self.entity_issuer{return Default::default()}close_demands(&self.frame,self.headers,self.receipt,self.closing,self.closed)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{let demand=self.close_demands();let p=close_frame(&mut self.frame,&mut self.headers,&mut self.closed,demand,g)?;self.receipt=Some((g,p));Some(p)}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.frame.is_none()&&self.receipt.is_none()}
}
impl ToolRunReceivedTickSource{
 pub fn source_identity(&self)->Witness{self.source}
 pub fn limits(&self)->&ToolRunReceivingLimits{&self.limits}
 pub fn native_address(&self)->Option<usize>{self.frame.as_ref().map(|f|f.as_ptr()as usize)}
 pub fn tick(&self)->Option<&ToolRunReceivedTick>{if self.closing||self.closed||self.receipt.is_some(){return None}self.frame.as_ref().map(|f|unsafe{f.assume_init_ref()})}
 pub fn receipt(&self)->Receipt{self.receipt}
 pub fn take_receipt(&mut self)->Receipt{self.receipt.take()}
 pub fn cancel(&mut self){self.closing=true}
 pub fn close_demands(&self)->RetirementDemand{close_demands(&self.frame,self.headers,self.receipt,self.closing,self.closed)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{let demand=self.close_demands();let p=close_frame(&mut self.frame,&mut self.headers,&mut self.closed,demand,g)?;self.receipt=Some((g,p));Some(p)}
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.frame.is_none()&&self.receipt.is_none()}
}
fn close_demands(frame:&Option<Box<MaybeUninit<ToolRunReceivedTick>>>,headers:u8,receipt:Receipt,closing:bool,closed:bool)->RetirementDemand{
 if receipt.is_some()||!closing||closed{return Default::default()}
 if headers==4{let target=frame.as_ref().expect("initialized Tick body retains its frame").as_ptr();let column=unsafe{&*std::ptr::addr_of!((*target).append_entities)};if !column.is_empty(){return RetirementDemand{depth:column.next_pop_depth_demand().expect("original entity tail has a native page path")+1,..Default::default()}}if !column.terminal_is_empty(){return RetirementDemand{release_bytes:column.next_release_allocation_bytes().expect("original entity column has no live scalar"),depth:column.next_release_depth_demand().expect("original entity column has a native release path")+1,..Default::default()}}}
 RetirementDemand{release_bytes:if frame.is_some()&&headers==0{ToolRunNativeTickPreparation::birth_bytes()}else{0},..metadata()}
}
fn close_frame(frame:&mut Option<Box<MaybeUninit<ToolRunReceivedTick>>>,headers:&mut u8,closed:&mut bool,d:RetirementDemand,g:RetainedCloneGrant)->Option<RetainedCloneProgress>{
 if d.depth==0||!funded(g,d){return None}if *headers>0{let target=frame.as_mut()?.as_mut_ptr();if *headers==4{let column=unsafe{&mut *addr_of_mut!((*target).append_entities)};if !column.is_empty(){column.truncate_retired_last().expect("original entity tail is retained");return Some(RetainedCloneProgress{copied_items:1,..Default::default()})}if !column.terminal_is_empty(){let p=column.release_empty_page(g.maximum_release_bytes).expect("original entity column release is quoted");assert!(p.progressed);return Some(RetainedCloneProgress{copied_items:1,released_bytes:p.released_allocation_bytes,..Default::default()})}}
 unsafe{match *headers-1{0=>addr_of_mut!((*target).steps).drop_in_place(),1=>addr_of_mut!((*target).trace).drop_in_place(),2=>addr_of_mut!((*target).append_ops).drop_in_place(),3=>addr_of_mut!((*target).append_entities).drop_in_place(),4=>addr_of_mut!((*target).payload).drop_in_place(),5=>addr_of_mut!((*target).progress).drop_in_place(),_=>{}}}*headers-=1}else if frame.is_some(){drop(frame.take())}else{*closed=true}Some(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})
}
impl Drop for ToolRunNativeTickPreparation{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original native Tick preparation abandoned its frame or receipt")}}
impl Drop for ToolRunReceivedTickSource{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original received Tick source abandoned its frame or receipt")}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
