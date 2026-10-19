//! 🌍️ Fixed native I/O slots retain one launch-issued caller through path receiving, actual worker turns and World page returns.
use super::RendererOriginalIoNode;
use crate::{renderer_worker_pool,RENDERER_IO_WAKE};
use crate::native_caller::NativeFrameCaller;
use semio_framework_actor::retained_context::{OriginalActorCall,OriginalActorContextError};
use semio_framework_plugin_guest_contract::original_actor_context::OriginalActorEntryFailure;
use semio_framework_job::{OriginalJobPageReturnRegistry,OriginalSuppliedJobPage,OriginalContiguousJobBytes,OriginalJobBytesReturnRegistry,OriginalJobBytesReservation,OriginalSuppliedJobBytes,OriginalTurnFailure,OriginalConstructionDemand,StepContext,JobOutcomeView,admit_original_construction};
use semio_framework_os_services_native_io::{NativeIoJob, NativeIoRequest, NativeIoResultStatus, NativeIoStreamRefusal, OriginalNativePathReturnRegistry, OriginalNativePathReservation, OriginalSuppliedNativePath};
use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use std::sync::{Mutex,MutexGuard};
use std::sync::atomic::{AtomicBool,AtomicU8,AtomicU64,AtomicUsize,Ordering};
use std::task::{Context,Poll,Waker};
use std::future::Future;
use std::pin::Pin;
use std::mem::size_of;

const VACANT:u8=0;
const LIVE:u8=1;
const CHECKED:u8=2;
const EXHAUSTED:u8=3;
const INVOCATIONS:usize=semio_framework_job_worker::WORKER_JOB_SESSION_SLOTS+1;
const RETURN_SLOTS:usize=semio_framework_job_payload::JOB_PAYLOAD_PROCESS_BYTES/semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
static RETURNS:OriginalJobPageReturnRegistry<RETURN_SLOTS>=OriginalJobPageReturnRegistry::with_return_wake(renderer_source_io_return_wake);
static BYTE_RETURNS:OriginalJobBytesReturnRegistry<INVOCATIONS>=OriginalJobBytesReturnRegistry::with_return_wake(renderer_source_io_return_wake);
static PATH_RETURNS:OriginalNativePathReturnRegistry<INVOCATIONS>=OriginalNativePathReturnRegistry::with_return_wake(renderer_source_io_return_wake);
static SLOTS:[RendererSourceIoSlot;INVOCATIONS]=[const{RendererSourceIoSlot::empty()};INVOCATIONS];
static WORKER_SCHEDULED:AtomicBool=AtomicBool::new(false);
static WORK_PENDING:AtomicBool=AtomicBool::new(false);
static WORK_CURSOR:AtomicUsize=AtomicUsize::new(0);

/// 🧭️ Borrowed UTF-8 path components remain in the initiating future until original receiving owns the sole backing.
#[derive(Clone,Copy)]
pub struct RendererIoPath<'a>{parts:[&'a std::ffi::OsStr;3]}
impl<'a> RendererIoPath<'a>{
 pub fn new(parts:[&'a str;3])->Self{Self{parts:parts.map(std::ffi::OsStr::new)}}
 pub fn from_path(path:&'a std::path::Path)->Self{Self{parts:[path.as_os_str(),std::ffi::OsStr::new(""),std::ffi::OsStr::new("")]}}
 fn extent(self)->Result<usize,ValueError>{let mut count=0usize;let mut length=0usize;for part in self.parts.into_iter().filter(|part|!part.is_empty()){length=length.checked_add(part.as_encoded_bytes().len()).and_then(|length|length.checked_add(usize::from(count!=0))).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native path extent overflow"))?;count+=1;}if count==0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original native path requires a component"))}Ok(length)}
 fn receive(self,length:usize,mode:RendererSourceIoMode)->Result<(NativeIoRequest,RetainedCloneProgress),ValueError>{
  let mut bytes=Vec::new();bytes.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original native path allocation refused"))?;let capacity=bytes.capacity();
  for part in self.parts.into_iter().filter(|part|!part.is_empty()){if !bytes.is_empty(){bytes.push(b'/');}bytes.extend_from_slice(part.as_encoded_bytes());}
  let path=std::path::PathBuf::from(unsafe{std::ffi::OsString::from_encoded_bytes_unchecked(bytes)});
  let request=match mode{RendererSourceIoMode::Contiguous{..}=>NativeIoRequest::ReadPage{path,offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES},RendererSourceIoMode::Directory=>NativeIoRequest::ScanDirectory{path,directories_only:false,extension:Some(std::borrow::Cow::Borrowed("wasm")),first_only:true},RendererSourceIoMode::Modified=>{let mut paths=semio_framework_os_services_native_io::NativePathSet::new();paths.try_push(path).expect("one original modified path fits its fixed input owner");NativeIoRequest::Modified(paths)},RendererSourceIoMode::World=>NativeIoRequest::ReadBytes(path),RendererSourceIoMode::Resident=>unreachable!()};
  Ok((request,RetainedCloneProgress{copied_items:1,copied_bytes:length,retained_capacity_bytes:capacity,..Default::default()}))
 }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RendererSourceIoFailureKind{Source,NativeFault,Cancelled,Extent(NativeIoStreamRefusal),ChangedExtent{expected:u64,actual:u64},WrongResult,RecipientRefused,RegistryFull,Stale,WorkerShutdown}

#[derive(Clone,Copy)]
enum RendererSourceIoMode{World,Contiguous{maximum:u64},Directory,Resident,Modified}

/// 🚨️ A semantic refusal retains the actual registry/source and every complete cause until its consumer releases this owner.
pub struct RendererSourceIoError{pub kind:RendererSourceIoFailureKind,handle:Option<RendererSourceIoHandle>,launch:Option<ValueError>}
impl RendererSourceIoError{
 fn source(kind:RendererSourceIoFailureKind,handle:RendererSourceIoHandle)->Self{Self{kind,handle:Some(handle),launch:None}}
 fn launch(error:ValueError)->Self{Self{kind:RendererSourceIoFailureKind::Source,handle:None,launch:Some(error)}}
}
impl std::fmt::Debug for RendererSourceIoError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{std::fmt::Display::fmt(self,f)}}
impl std::fmt::Display for RendererSourceIoError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
 if let Some(error)=self.launch.as_ref(){return std::fmt::Display::fmt(error,f)}
 if let Some(handle)=self.handle.as_ref(){if let Some(result)=with_node(handle.slot,handle.generation,|node|{
  if let Some(error)=node.refusals.iter().flatten().next(){return write!(f,"{error}")}
  if let Some(error)=node.caller.original_failure(){return write!(f,"{error:?}")}
  if let Some(error)=node.caller.original_cleanup_failure(){return write!(f,"{error:?}")}
  if let Some(error)=node.root_failure.as_ref(){return write!(f,"{error:?}")}
  if let Some(invocation)=node.invocation.as_ref(){if let Ok(Some(JobOutcomeView::Fault{detail,..}))=invocation.borrow_outcome(){for index in 0..detail.page_count(){if let Some(bytes)=detail.page(index){match std::str::from_utf8(bytes){Ok(text)=>f.write_str(text)?,Err(_)=>write!(f,"{:?}",self.kind)?}}}return Ok(())}}
  write!(f,"{:?}",self.kind)
 }){return result}}
 write!(f,"{:?}",self.kind)
}}

struct RendererSourceIoSlot{state:AtomicU8,generation:AtomicU64,detached:AtomicBool,retry:AtomicBool,node:Mutex<Option<RendererSourceIoNode>>}
impl RendererSourceIoSlot{const fn empty()->Self{Self{state:AtomicU8::new(VACANT),generation:AtomicU64::new(0),detached:AtomicBool::new(false),retry:AtomicBool::new(false),node:Mutex::new(None)}}}
struct SlotBorrow<'a>{slot:&'a RendererSourceIoSlot,guard:Option<MutexGuard<'a,Option<RendererSourceIoNode>>>}
impl Drop for SlotBorrow<'_>{fn drop(&mut self){let state=if self.guard.as_ref().unwrap().is_some(){LIVE}else if self.slot.generation.load(Ordering::Relaxed)==u64::MAX{EXHAUSTED}else{VACANT};drop(self.guard.take());self.slot.state.store(state,Ordering::Release);if self.slot.retry.swap(false,Ordering::AcqRel){renderer_source_io_return_wake();}}}
fn with_node<T>(index:usize,generation:u64,visit:impl FnOnce(&mut RendererSourceIoNode)->T)->Option<T>{
 let slot=SLOTS.get(index)?;if slot.generation.load(Ordering::Acquire)!=generation{return None}if let Err(state)=slot.state.compare_exchange(LIVE,CHECKED,Ordering::Acquire,Ordering::Relaxed){if state==CHECKED{slot.retry.store(true,Ordering::Release);if slot.state.load(Ordering::Acquire)!=CHECKED&&slot.retry.swap(false,Ordering::AcqRel){renderer_source_io_return_wake();}}return None}
 let guard=match slot.node.try_lock(){Ok(guard)=>guard,Err(_)=>{slot.state.store(LIVE,Ordering::Release);return None}};let mut checked=SlotBorrow{slot,guard:Some(guard)};
 if slot.generation.load(Ordering::Relaxed)!=generation{return None}checked.guard.as_mut().unwrap().as_mut().map(visit)
}
fn generation_live(index:usize,generation:u64)->bool{SLOTS.get(index).is_some_and(|slot|slot.generation.load(Ordering::Acquire)==generation&&matches!(slot.state.load(Ordering::Acquire),LIVE|CHECKED))}

struct RendererSourceIoNode{
 caller:NativeFrameCaller,
 mode:RendererSourceIoMode,
 request:Option<NativeIoRequest>,
 request_committed:bool,
 invocation:Option<RendererOriginalIoNode<RETURN_SLOTS>>,
 invocation_committed:bool,
 orphan:Option<NativeIoJob>,
 page:Option<OriginalSuppliedJobPage>,
 page_committed:bool,
 page_requested:bool,
 contiguous:Option<OriginalContiguousJobBytes>,
 contiguous_extent:Option<u64>,
 contiguous_consumed:bool,
 bytes_reservation:Option<OriginalJobBytesReservation<INVOCATIONS>>,
 bytes_identity:Option<usize>,
 published_bytes:Option<OriginalSuppliedJobBytes>,
 path_reservation:Option<OriginalNativePathReservation<INVOCATIONS>>,
 path_identity:Option<usize>,
 published_path:Option<OriginalSuppliedNativePath>,
 metadata_consumed:bool,
 ready:bool,
 status:Option<NativeIoResultStatus>,
 failure:Option<RendererSourceIoFailureKind>,
 refusals:[Option<ValueError>;4],
 root_failure:Option<OriginalActorContextError>,
 closing:bool,
 closed:bool,
 waker:Option<Waker>,
}
impl RendererSourceIoNode{
 fn new(caller:NativeFrameCaller,mode:RendererSourceIoMode)->Self{Self{caller,mode,request:None,request_committed:false,invocation:None,invocation_committed:false,orphan:None,page:None,page_committed:false,page_requested:false,contiguous:None,contiguous_extent:None,contiguous_consumed:false,bytes_reservation:None,bytes_identity:None,published_bytes:None,path_reservation:None,path_identity:None,published_path:None,metadata_consumed:false,ready:false,status:None,failure:None,refusals:std::array::from_fn(|_|None),root_failure:None,closing:false,closed:false,waker:None}}
 fn room(&self)->bool{self.refusals.iter().any(Option::is_none)}
 fn retain(&mut self,error:ValueError){*self.refusals.iter_mut().find(|slot|slot.is_none()).expect("an original callback reserved a refusal slot")=Some(error);self.failure.get_or_insert(RendererSourceIoFailureKind::Source);}
 fn accept<T>(&mut self,result:Result<OriginalActorCall<T,OriginalActorEntryFailure>,ValueError>)->Option<T>{match result{Ok(call)=>match call.result{Ok(value)=>Some(value),Err(OriginalActorEntryFailure::Semantic)=>{self.failure.get_or_insert(RendererSourceIoFailureKind::Source);None},Err(OriginalActorEntryFailure::Authority(error))=>{self.retain(error);None}},Err(error)=>{self.retain(error);None}}}
 fn prepare(&mut self,path:RendererIoPath<'_>){
  if self.request.is_some()||!self.room()||self.failure.is_some()||!self.caller.ready(){return}
  if matches!(self.mode,RendererSourceIoMode::Resident){let request=&mut self.request;let call=self.caller.dispatch_step(|cx|admit_original_construction(cx,request,OriginalConstructionDemand{items:1,copy_bytes:size_of::<NativeIoRequest>(),capacity_bytes:0,release_bytes:0,depth:1},|_|Ok((NativeIoRequest::ProcessResidentBytes,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<NativeIoRequest>(),..Default::default()}))));if self.accept(call)==Some(true){self.request_committed=true;}return}
  let length=match path.extent(){Ok(length)=>length,Err(error)=>{self.retain(error);return}};
  let mode=self.mode;let request=&mut self.request;let call=self.caller.dispatch_step(|cx|admit_original_construction(cx,request,OriginalConstructionDemand{items:1,copy_bytes:length,capacity_bytes:length,release_bytes:0,depth:1},|_|path.receive(length,mode)));
  if self.accept(call)==Some(true){self.request_committed=true;}
 }
 fn needs_work(&self)->bool{!self.closed&&self.root_failure.is_none()&&(self.closing||self.failure.is_none()&&(!self.caller.ready()||self.request_committed&&!self.ready||self.page_requested&&!self.page_committed))}
 fn frontier(&self)->[usize;21]{[self.ready as usize,self.closed as usize,self.request_committed as usize,self.invocation_committed as usize,self.page_committed as usize,self.request.is_some()as usize,self.invocation.is_some()as usize,self.orphan.is_some()as usize,self.page.is_some()as usize,self.refusals.iter().flatten().count(),self.failure.is_some()as usize,self.caller.ready()as usize,self.contiguous.is_some()as usize,self.contiguous_consumed as usize,self.bytes_reservation.is_some()as usize,self.published_bytes.is_some()as usize,self.contiguous_extent.is_some()as usize,self.contiguous.as_ref().map_or(0,OriginalContiguousJobBytes::retained_length),self.path_reservation.is_some()as usize,self.published_path.is_some()as usize,self.metadata_consumed as usize]}
 fn pump(&mut self)->bool{
  if !self.room()||self.root_failure.is_some(){return false}
  let before=(self.caller.retained_spent(),self.caller.fuel_remaining(),self.frontier());
  if self.closing{self.close_one();}
  else if self.failure.is_some(){return false}
  else if !self.caller.ready(){match self.caller.admit_step(){Ok(call)=>if let Err(error)=call.result{self.retain(error)},Err(error)=>self.retain(error)}}
  else if self.request_committed&&!self.invocation_committed{
   let request=&mut self.request;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|RendererOriginalIoNode::admit(request,invocation,&RETURNS,cx));if self.accept(call)==Some(true){self.invocation_committed=true;}
  }else if self.invocation_committed&&!self.ready&&matches!(self.mode,RendererSourceIoMode::Contiguous{..}){self.pump_contiguous();
  }else if self.invocation_committed&&!self.ready&&matches!(self.mode,RendererSourceIoMode::Directory){self.pump_directory();
  }else if self.invocation_committed&&!self.ready{
   let fault=match self.invocation.as_ref().unwrap().borrow_outcome(){Ok(Some(JobOutcomeView::Fault{..}))=>Some(RendererSourceIoFailureKind::NativeFault),Ok(Some(JobOutcomeView::Cancelled{..}))=>Some(RendererSourceIoFailureKind::Cancelled),Ok(_)=>None,Err(error)=>{self.retain(error);return true}};
   if let Some(fault)=fault{self.failure=Some(fault);}else{let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().advance(cx));if self.accept(call)==Some(true){self.status=self.invocation.as_ref().unwrap().result_status();match (self.mode,self.status){(RendererSourceIoMode::World,Some(NativeIoResultStatus::Bytes{..}))|(RendererSourceIoMode::Resident,Some(NativeIoResultStatus::ResidentBytes(_)))|(RendererSourceIoMode::Modified,Some(NativeIoResultStatus::Modified{..}))=>self.ready=true,(_,Some(NativeIoResultStatus::StreamRefused(refusal)))=>self.failure=Some(RendererSourceIoFailureKind::Extent(refusal)),(_,Some(NativeIoResultStatus::Fault))=>self.failure=Some(RendererSourceIoFailureKind::NativeFault),_=>self.failure=Some(RendererSourceIoFailureKind::WrongResult)}}}
  }else if self.page_requested&&!self.page_committed{
   let page=&mut self.page;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().supply_next_page(page,cx));if self.accept(call)==Some(true){self.page_committed=true;self.status=self.invocation.as_ref().unwrap().result_status();}
  }
  before!=(self.caller.retained_spent(),self.caller.fuel_remaining(),self.frontier())
 }
 fn pump_directory(&mut self){
  if !self.invocation.as_ref().unwrap().outcome_is_acknowledged(){let failure=match self.invocation.as_ref().unwrap().borrow_outcome(){Ok(Some(JobOutcomeView::Fault{..}))=>Some(RendererSourceIoFailureKind::NativeFault),Ok(Some(JobOutcomeView::Cancelled{..}))=>Some(RendererSourceIoFailureKind::Cancelled),Ok(_)=>None,Err(error)=>{self.retain(error);return}};if let Some(failure)=failure{self.failure=Some(failure);return}let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().advance(cx));self.accept(call);return}
  self.status=self.invocation.as_ref().unwrap().result_status();let Some(NativeIoResultStatus::Paths{count})=self.status else{self.failure=Some(RendererSourceIoFailureKind::WrongResult);return};if count==0{self.ready=true;return}
  if self.path_reservation.is_none(){let reservation=&mut self.path_reservation;let identity=&mut self.path_identity;let call=self.caller.dispatch_step(|cx|{*identity=Some(cx.original_source_parent()?.source_identity()?);*reservation=PATH_RETURNS.reserve(cx)?;Ok(reservation.is_some())});self.accept(call);return}
  let reservation=&mut self.path_reservation;let output=&mut self.published_path;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().supply_selected_path(reservation.as_mut().unwrap(),output,cx));if self.accept(call)==Some(true){self.path_reservation.take();self.ready=true;}
 }
 fn pump_contiguous(&mut self){
  let maximum=match self.mode{RendererSourceIoMode::Contiguous{maximum}=>maximum,_=>unreachable!()};
  if let Some(actual)=self.invocation.as_ref().unwrap().known_file_length(){if actual>maximum{self.failure=Some(RendererSourceIoFailureKind::Extent(NativeIoStreamRefusal{length:actual,maximum_bytes:maximum}));return}if let Some(expected)=self.contiguous_extent{if expected!=actual{self.failure=Some(RendererSourceIoFailureKind::ChangedExtent{expected,actual});return}}else{self.contiguous_extent=Some(actual);}}
  if !self.invocation.as_ref().unwrap().outcome_is_acknowledged(){
   let failure=match self.invocation.as_ref().unwrap().borrow_outcome(){Ok(Some(JobOutcomeView::Fault{..}))=>Some(RendererSourceIoFailureKind::NativeFault),Ok(Some(JobOutcomeView::Cancelled{..}))=>Some(RendererSourceIoFailureKind::Cancelled),Ok(_)=>None,Err(error)=>{self.retain(error);return}};
   if let Some(failure)=failure{self.failure=Some(failure);return}let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().advance(cx));self.accept(call);return
  }
  let Some(NativeIoResultStatus::Page{length,pages,eof})=self.invocation.as_ref().unwrap().result_status()else{self.failure=Some(RendererSourceIoFailureKind::WrongResult);return};
  let Some(actual)=self.invocation.as_ref().unwrap().known_file_length()else{self.failure=Some(RendererSourceIoFailureKind::WrongResult);return};
  if actual>maximum{self.failure=Some(RendererSourceIoFailureKind::Extent(NativeIoStreamRefusal{length:actual,maximum_bytes:maximum}));return}
  if let Some(expected)=self.contiguous_extent{if expected!=actual{self.failure=Some(RendererSourceIoFailureKind::ChangedExtent{expected,actual});return}}else{self.contiguous_extent=Some(actual);}
  if !self.contiguous.as_ref().is_some_and(OriginalContiguousJobBytes::ready){let Ok(length)=usize::try_from(actual)else{self.failure=Some(RendererSourceIoFailureKind::Extent(NativeIoStreamRefusal{length:actual,maximum_bytes:usize::MAX as u64}));return};let output=&mut self.contiguous;let call=self.caller.dispatch_step(|cx|OriginalContiguousJobBytes::admit(length,output,cx));self.accept(call);return}
  if !self.contiguous_consumed{
   let copied=self.contiguous.as_ref().unwrap().retained_length();if (pages==0&&length!=0)||pages>1||copied.checked_add(length).is_none_or(|total|total as u64>actual)||length==0&&!eof{self.failure=Some(RendererSourceIoFailureKind::ChangedExtent{expected:actual,actual:copied.saturating_add(length)as u64});return}
   let page=if pages==0{&[][..]}else{let Some(page)=self.invocation.as_ref().unwrap().result_page(0)else{self.failure=Some(RendererSourceIoFailureKind::WrongResult);return};if page.len()!=length{self.failure=Some(RendererSourceIoFailureKind::WrongResult);return}page};let output=&mut self.contiguous;let call=self.caller.dispatch_step(|cx|output.as_mut().unwrap().append(page,cx));if self.accept(call)==Some(true){self.contiguous_consumed=true;}return
  }
  if eof{
   let length=self.contiguous.as_ref().unwrap().retained_length()as u64;if length!=actual{self.failure=Some(RendererSourceIoFailureKind::ChangedExtent{expected:actual,actual:length});return}
   if self.bytes_reservation.is_none(){let reservation=&mut self.bytes_reservation;let identity=&mut self.bytes_identity;let call=self.caller.dispatch_step(|cx|{*identity=Some(cx.original_source_parent()?.source_identity()?);*reservation=BYTE_RETURNS.reserve(cx)?;Ok(reservation.is_some())});self.accept(call);return}
   let reservation=&mut self.bytes_reservation;let bytes=&mut self.contiguous;let output=&mut self.published_bytes;let call=self.caller.dispatch_step(|cx|reservation.as_mut().unwrap().publish(bytes,output,cx));if self.accept(call)==Some(true){self.bytes_reservation.take();self.ready=true;}return
  }
  let offset=self.contiguous.as_ref().unwrap().retained_length()as u64;let invocation=&mut self.invocation;let call=self.caller.dispatch_step(|cx|invocation.as_mut().unwrap().restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx));if self.accept(call)==Some(true){self.contiguous_consumed=false;}
 }
 fn close_one(&mut self){
  use semio_framework_job::InteractiveJob;
  if self.published_path.is_some(){let path=&mut self.published_path;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<OriginalSuppliedNativePath>>(),depth:1,..Default::default()},|_|{path.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalSuppliedNativePath>>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if self.path_reservation.is_some(){let reservation=&mut self.path_reservation;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<OriginalNativePathReservation<INVOCATIONS>>>(),depth:1,..Default::default()},|_|{reservation.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalNativePathReservation<INVOCATIONS>>>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if self.published_bytes.is_some(){let bytes=&mut self.published_bytes;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<OriginalSuppliedJobBytes>>(),depth:1,..Default::default()},|_|{bytes.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalSuppliedJobBytes>>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if self.contiguous.is_some(){let bytes=&mut self.contiguous;let call=self.caller.dispatch_cleanup_step(|cx|bytes.as_mut().unwrap().retire_step(cx));if self.accept(call)==Some(true){self.contiguous.take();}return}
  if self.bytes_reservation.is_some(){let reservation=&mut self.bytes_reservation;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<OriginalJobBytesReservation<INVOCATIONS>>>(),depth:1,..Default::default()},|_|{reservation.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalJobBytesReservation<INVOCATIONS>>>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if self.page.is_some(){let page=&mut self.page;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<OriginalSuppliedJobPage>>(),depth:1,..Default::default()},|_|{page.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<OriginalSuppliedJobPage>>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if let Some(invocation)=self.invocation.as_mut(){invocation.begin_close();let call=self.caller.dispatch_cleanup_step(|cx|{if !invocation.terminal_is_empty(){invocation.close_step(cx)?;}Ok(invocation.terminal_is_empty())});if self.accept(call)==Some(true){let invocation=&mut self.invocation;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()},|_|{invocation.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()}))})?;Ok(())});self.accept(call);}return}
  if self.request.is_some(){let request=&mut self.request;let orphan=&mut self.orphan;let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<NativeIoRequest>(),depth:1,..Default::default()},|_|{*orphan=Some(NativeIoJob::new(request.take().unwrap()));orphan.as_mut().unwrap().begin_close();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<NativeIoRequest>(),..Default::default()}))})?;Ok(())});self.accept(call);return}
  if let Some(orphan)=self.orphan.as_mut(){let call=self.caller.dispatch_cleanup_step(|cx|{if !orphan.terminal_is_empty(){orphan.close_step(cx)?;}Ok(orphan.terminal_is_empty())});if self.accept(call)==Some(true){self.orphan.take();}return}
  if let Some(index)=self.refusals.iter().position(Option::is_some){
   if !self.caller.ready(){if matches!(self.refusals[index].as_ref().unwrap().message,std::borrow::Cow::Borrowed(_)){self.refusals[index].take();}else{return}}
   else{let refusal=&mut self.refusals[index];let release=match &refusal.as_ref().unwrap().message{std::borrow::Cow::Owned(text)=>text.capacity(),_=>0};let call=self.caller.dispatch_cleanup_step(|cx|{semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:size_of::<Option<ValueError>>(),release_bytes:release,depth:1,..Default::default()},|_|{refusal.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<ValueError>>(),released_bytes:release,..Default::default()}))})?;Ok(())});self.accept(call);}return
  }
  if self.bytes_identity.is_some_and(|identity|BYTE_RETURNS.retains_source(identity)){let call=self.caller.dispatch_cleanup_step(|cx|{BYTE_RETURNS.close_returned_step(cx)?;Ok(())});self.accept(call);return}
  if self.path_identity.is_some_and(|identity|PATH_RETURNS.retains_source(identity)){let call=self.caller.dispatch_cleanup_step(|cx|{PATH_RETURNS.close_returned_step(cx)?;Ok(())});self.accept(call);return}
  if !self.caller.original_loans_are_returned(){return}
  match self.caller.close_step(){Ok(call)=>match call.result{Ok(RetainedCloneStep::Complete(_))if self.caller.terminal_is_empty()=>self.closed=true,Ok(_)=>{},Err(error)=>{self.root_failure=Some(error);}},Err(error)=>self.retain(error)}
 }
 fn terminal_is_empty(&self)->bool{self.closed&&self.caller.terminal_is_empty()&&self.request.is_none()&&self.invocation.is_none()&&self.orphan.is_none()&&self.page.is_none()&&self.contiguous.is_none()&&self.bytes_reservation.is_none()&&self.published_bytes.is_none()&&self.path_reservation.is_none()&&self.published_path.is_none()&&self.refusals.iter().all(Option::is_none)&&self.root_failure.is_none()}
}
impl Drop for RendererSourceIoNode{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"fixed native I/O source cannot drop before its original page/job/failure/root close");}}

/// 🎫️ A non-clone handle observes one exact fixed-slot generation and releases it only through original asynchronous cleanup.
pub struct RendererSourceIoHandle{slot:usize,generation:u64}
impl RendererSourceIoHandle{
 pub fn next_page(&mut self)->RendererSourceIoPage<'_>{RendererSourceIoPage{handle:self,completed:false}}
 pub fn failure(self,kind:RendererSourceIoFailureKind)->RendererSourceIoError{RendererSourceIoError::source(kind,self)}
 pub fn prepare_modified(&mut self,path:&std::path::Path)->Result<bool,RendererSourceIoFailureKind>{let answer=with_node(self.slot,self.generation,|node|{if !matches!(node.mode,RendererSourceIoMode::Modified){return Err(RendererSourceIoFailureKind::WrongResult)}if let Some(failure)=node.failure{return Err(failure)}if !node.request_committed{node.prepare(RendererIoPath::from_path(path));}if let Some(failure)=node.failure{return Err(failure)}Ok(node.request_committed)});renderer_source_io_return_wake();answer.unwrap_or_else(||if generation_live(self.slot,self.generation){Ok(false)}else{Err(RendererSourceIoFailureKind::Stale)})}
 pub fn try_modified(&mut self,visit:impl FnOnce(Option<(&std::path::Path,std::time::SystemTime)>)->bool)->Option<Result<bool,RendererSourceIoFailureKind>>{with_node(self.slot,self.generation,|node|{if let Some(failure)=node.failure{return Some(Err(failure))}if !node.ready{return None}if node.metadata_consumed{return Some(Err(RendererSourceIoFailureKind::Stale))}let count=match node.status{Some(NativeIoResultStatus::Modified{count})if count<=1=>count,_=>return Some(Err(RendererSourceIoFailureKind::WrongResult))};let value=if count==0{None}else{let Some(value)=node.invocation.as_ref().unwrap().result_modified(0)else{return Some(Err(RendererSourceIoFailureKind::WrongResult))};Some(value)};let answer=visit(value);node.metadata_consumed=true;Some(Ok(answer))}).flatten().or_else(||(!generation_live(self.slot,self.generation)).then_some(Err(RendererSourceIoFailureKind::Stale)))}
}
impl Drop for RendererSourceIoHandle{fn drop(&mut self){if let Some(slot)=SLOTS.get(self.slot){if slot.generation.load(Ordering::Acquire)==self.generation{slot.detached.store(true,Ordering::Release);}}renderer_source_io_return_wake();}}
pub struct RendererSourceIoPage<'a>{handle:&'a mut RendererSourceIoHandle,completed:bool}
impl Future for RendererSourceIoPage<'_>{type Output=Result<Option<OriginalSuppliedJobPage>,RendererSourceIoFailureKind>;fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
 let this=self.get_mut();assert!(!this.completed,"a completed original native page future cannot issue another output");let handle=&this.handle;let result=with_node(handle.slot,handle.generation,|node|{
  if let Some(failure)=node.failure{return Some(Err(failure))}
  if node.page_committed{node.page_committed=false;node.page_requested=false;return Some(Ok(node.page.take()))}
  if matches!(node.status,Some(NativeIoResultStatus::Bytes{pages:0,..})){return Some(Ok(None))}
  node.page_requested=true;node.waker=Some(cx.waker().clone());None
 });renderer_source_io_return_wake();match result{Some(Some(answer))=>{this.completed=true;Poll::Ready(answer)},Some(None)=>Poll::Pending,None if !generation_live(handle.slot,handle.generation)=>{this.completed=true;Poll::Ready(Err(RendererSourceIoFailureKind::Stale))},None=>{cx.waker().wake_by_ref();Poll::Pending}}
}}

pub struct RendererSourceIoOpen<'a>{path:RendererIoPath<'a>,handle:Option<RendererSourceIoHandle>,mode:RendererSourceIoMode,startup:bool,completed:bool}
pub fn open_renderer_source_io(path:RendererIoPath<'_>)->RendererSourceIoOpen<'_>{RendererSourceIoOpen{path,handle:None,mode:RendererSourceIoMode::World,startup:false,completed:false}}
fn claim_renderer_source(mode:RendererSourceIoMode,startup:bool)->Result<RendererSourceIoHandle,RendererSourceIoError>{
  for(index,slot)in SLOTS.iter().enumerate(){
   if slot.state.compare_exchange(VACANT,CHECKED,Ordering::Acquire,Ordering::Relaxed).is_err(){continue}
   let guard=match slot.node.try_lock(){Ok(guard)=>guard,Err(_)=>{slot.state.store(VACANT,Ordering::Release);continue}};let mut checked=SlotBorrow{slot,guard:Some(guard)};
   let Some(generation)=slot.generation.load(Ordering::Relaxed).checked_add(1)else{slot.generation.store(u64::MAX,Ordering::Relaxed);continue};
   let caller=match crate::native_launch::native_launch_caller(startup){Ok(caller)=>caller,Err(error)=>{return Err(RendererSourceIoError::launch(error))}};
   slot.generation.store(generation,Ordering::Relaxed);slot.detached.store(false,Ordering::Relaxed);**checked.guard.as_mut().unwrap()=Some(RendererSourceIoNode::new(caller,mode));return Ok(RendererSourceIoHandle{slot:index,generation})
  }
 Err(RendererSourceIoError{kind:RendererSourceIoFailureKind::RegistryFull,handle:None,launch:None})
}
impl Future for RendererSourceIoOpen<'_>{type Output=Result<RendererSourceIoHandle,RendererSourceIoError>;fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
 let this=self.get_mut();assert!(!this.completed,"a completed native source future cannot issue another invocation");
 if this.handle.is_none(){match claim_renderer_source(this.mode,this.startup){Ok(handle)=>this.handle=Some(handle),Err(error)=>{this.completed=true;return Poll::Ready(Err(error))}}}
 if let Some(handle)=this.handle.as_ref(){let state=with_node(handle.slot,handle.generation,|node|{node.waker=Some(cx.waker().clone());if node.caller.ready()&&!node.request_committed&&!node.invocation_committed{node.prepare(this.path);}if let Some(failure)=node.failure{Some(Err(failure))}else if node.ready{Some(Ok(()))}else{None}});match state{Some(Some(Ok(())))=>{this.completed=true;return Poll::Ready(Ok(this.handle.take().unwrap()))},Some(Some(Err(failure)))=>{this.completed=true;return Poll::Ready(Err(RendererSourceIoError::source(failure,this.handle.take().unwrap())))},None if !generation_live(handle.slot,handle.generation)=>{this.completed=true;return Poll::Ready(Err(RendererSourceIoError::source(RendererSourceIoFailureKind::Stale,this.handle.take().unwrap())))},None=>cx.waker().wake_by_ref(),_=>{}}}
 renderer_source_io_return_wake();Poll::Pending
}}

/// 📚️ The original startup caller owns exact contiguous backing through compiler/JSON borrows and its final supplier return.
pub struct RendererSourceIoBytesOpen<'a>{open:RendererSourceIoOpen<'a>,handle:Option<RendererSourceIoHandle>,completed:bool}
pub fn read_renderer_source_bytes(path:&std::path::Path,maximum:u64)->RendererSourceIoBytesOpen<'_>{RendererSourceIoBytesOpen{open:RendererSourceIoOpen{path:RendererIoPath::from_path(path),handle:None,mode:RendererSourceIoMode::Contiguous{maximum},startup:true,completed:false},handle:None,completed:false}}
impl Future for RendererSourceIoBytesOpen<'_>{type Output=Result<OriginalSuppliedJobBytes,RendererSourceIoError>;fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
 let this=self.get_mut();assert!(!this.completed,"completed original contiguous future cannot birth another source");if this.handle.is_none(){match Pin::new(&mut this.open).poll(cx){Poll::Ready(Ok(handle))=>this.handle=Some(handle),Poll::Ready(Err(error))=>{this.completed=true;return Poll::Ready(Err(error))},Poll::Pending=>return Poll::Pending}}
 let handle=this.handle.as_ref().unwrap();match with_node(handle.slot,handle.generation,|node|node.published_bytes.take()){Some(Some(bytes))=>{this.completed=true;this.handle.take();Poll::Ready(Ok(bytes))},Some(None)=>{this.completed=true;Poll::Ready(Err(this.handle.take().unwrap().failure(RendererSourceIoFailureKind::WrongResult)))},None if !generation_live(handle.slot,handle.generation)=>{this.completed=true;Poll::Ready(Err(this.handle.take().unwrap().failure(RendererSourceIoFailureKind::Stale)))},None=>{cx.waker().wake_by_ref();Poll::Pending}}
}}

/// 🛎️ One actual timer observation claims its startup source before later paid borrowed-path intake.
pub fn start_renderer_modified()->Result<RendererSourceIoHandle,RendererSourceIoError>{let handle=claim_renderer_source(RendererSourceIoMode::Modified,true)?;renderer_source_io_return_wake();Ok(handle)}

/// 📊️ A resident-memory observation returns only a scalar; its original producer closes asynchronously on handle return.
pub async fn read_renderer_resident_bytes()->Result<Option<u64>,RendererSourceIoError>{
 let handle=RendererSourceIoOpen{path:RendererIoPath::new(["","",""]),handle:None,mode:RendererSourceIoMode::Resident,startup:true,completed:false}.await?;
 let result=std::future::poll_fn(|cx|match with_node(handle.slot,handle.generation,|node|match node.status{Some(NativeIoResultStatus::ResidentBytes(bytes))=>Ok(bytes),_=>Err(RendererSourceIoFailureKind::WrongResult)}){Some(result)=>Poll::Ready(result),None if !generation_live(handle.slot,handle.generation)=>Poll::Ready(Err(RendererSourceIoFailureKind::Stale)),None=>{cx.waker().wake_by_ref();Poll::Pending}}).await;
 match result{Ok(bytes)=>Ok(bytes),Err(kind)=>Err(handle.failure(kind))}
}

/// 🧭️ The exact selected path keeps its directory supplier through subsequent borrowed-path component reads.
pub async fn find_renderer_wasm_path(path:&std::path::Path)->Result<Option<OriginalSuppliedNativePath>,RendererSourceIoError>{
 let handle=RendererSourceIoOpen{path:RendererIoPath::from_path(path),handle:None,mode:RendererSourceIoMode::Directory,startup:true,completed:false}.await?;
 let result=std::future::poll_fn(|cx|match with_node(handle.slot,handle.generation,|node|node.published_path.take()){Some(path)=>Poll::Ready(Ok(path)),None if !generation_live(handle.slot,handle.generation)=>Poll::Ready(Err(RendererSourceIoFailureKind::Stale)),None=>{cx.waker().wake_by_ref();Poll::Pending}}).await;
 match result{Ok(path)=>Ok(path),Err(kind)=>Err(handle.failure(kind))}
}

/// 🔔️ The fixed pending bit survives saturation without retaining another callback allocation.
fn renderer_source_io_return_wake(){WORK_PENDING.store(true,Ordering::Release);RENDERER_IO_WAKE.store(true,Ordering::Release);let _=pump_renderer_source_io_sessions(1);}
pub fn renderer_source_io_pending()->bool{SLOTS.iter().any(|slot|matches!(slot.state.load(Ordering::Acquire),LIVE|CHECKED))}
pub fn pump_renderer_source_io_sessions(maximum:usize)->usize{
 if maximum==0||!WORK_PENDING.load(Ordering::Acquire)||WORKER_SCHEDULED.compare_exchange(false,true,Ordering::AcqRel,Ordering::Acquire).is_err(){return 0}
 WORK_PENDING.swap(false,Ordering::AcqRel);
 let callback:Box<dyn FnOnce()+Send+'static>=Box::new(renderer_source_io_worker);
 match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io,callback){Ok(())=>1,Err(error)=>{WORK_PENDING.store(true,Ordering::Release);let kind=error.kind;drop(error.job);WORKER_SCHEDULED.store(false,Ordering::Release);if matches!(kind,semio_framework_async::WorkerSubmitErrorKind::Shutdown|semio_framework_async::WorkerSubmitErrorKind::Poisoned){for(index,slot)in SLOTS.iter().enumerate(){let generation=slot.generation.load(Ordering::Acquire);if let Some(Some(waker))=with_node(index,generation,|node|{node.failure.get_or_insert(RendererSourceIoFailureKind::WorkerShutdown);node.waker.take()}){waker.wake();}}}0}}
}
fn renderer_source_io_worker(){
 let start=WORK_CURSOR.fetch_add(1,Ordering::Relaxed);let mut again=false;
 for offset in 0..INVOCATIONS{let index=start.wrapping_add(offset)%INVOCATIONS;let slot=&SLOTS[index];let generation=slot.generation.load(Ordering::Acquire);let progress=with_node(index,generation,|node|{if slot.detached.load(Ordering::Acquire){node.closing=true;}if !node.needs_work(){return(false,None)}let progress=node.pump()&&node.needs_work();(progress,node.waker.take())});if let Some((progress,waker))=progress{again|=progress;if let Some(waker)=waker{waker.wake();}}
  if slot.detached.load(Ordering::Acquire)&&slot.state.compare_exchange(LIVE,CHECKED,Ordering::Acquire,Ordering::Relaxed).is_ok(){if let Ok(guard)=slot.node.try_lock(){let mut checked=SlotBorrow{slot,guard:Some(guard)};if checked.guard.as_ref().unwrap().as_ref().is_some_and(RendererSourceIoNode::terminal_is_empty){checked.guard.as_mut().unwrap().take();slot.detached.store(false,Ordering::Relaxed);}}else{slot.state.store(LIVE,Ordering::Release);}}
 }
 if again{WORK_PENDING.store(true,Ordering::Release);}WORKER_SCHEDULED.store(false,Ordering::Release);if WORK_PENDING.load(Ordering::Acquire){let _=pump_renderer_source_io_sessions(1);}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
pub fn original_native_source_test_guard()->std::sync::MutexGuard<'static,()>{tests::fixture_guard()}
