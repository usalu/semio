//! 📦️ One actual original page and its existing operation ledger move without copying backing.
use crate::{RetainedJobPayload,JobPayloadPage,JobPayloadOperationLedger,JobPayloadStream,OriginalPayloadSourceIdentity,RetainedCloneGrant,RetainedCloneProgress as P,RetainedCloneStep as S,RetainedWorkOutcome as O,RetirementDemand as D,ValueError,ValueRefusalKind};
use std::sync::Arc;
fn metadata()->P{P{copied_items:1,..Default::default()}}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn admitted(g:RetainedCloneGrant,d:D)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
/// 🪪️ The genuine detached page owns its exact backing and retained operation ledger until paid close.
pub struct RetainedJobPageTransfer{page:Option<JobPayloadPage>,ledger:Option<Arc<JobPayloadOperationLedger>>,stream:JobPayloadStream,index:usize,source:OriginalPayloadSourceIdentity}
impl RetainedJobPageTransfer{
 pub fn is_ready(&self)->bool{self.page.is_some()}
 pub fn original_index(&self)->usize{self.index}
 pub fn page_identity(&self)->Option<*const u8>{self.page.as_ref().map(|page|page.bytes().as_ptr())}
 pub fn ledger_identity(&self)->Option<OriginalPayloadSourceIdentity>{self.ledger.as_ref().map(|ledger|OriginalPayloadSourceIdentity(Arc::as_ptr(ledger)as usize))}
 pub fn bytes(&self)->Option<&[u8]>{self.page.as_ref().map(JobPayloadPage::bytes)}
 pub fn terminal_is_empty(&self)->bool{self.page.is_none()&&self.ledger.is_none()}
 pub fn retirement_demands(&self)->D{if let Some(page)=&self.page{D{release_bytes:page.source.allocated_capacity_bytes(),depth:1,..Default::default()}}else{crate::ledger_demands(&self.ledger)}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->O{
  if self.terminal_is_empty(){return O{result:Ok(S::Complete(Default::default())),work:0}}
  let d=self.retirement_demands();if !admitted(grant,d){return O{result:Ok(S::Progress(Default::default())),work:0}}
  if self.page.is_some(){let page=self.page.take().unwrap();self.ledger.as_ref().expect("detached backing retains its original operation ledger").release(self.stream,d.release_bytes);drop(page);return O{result:Ok(S::Progress(P{released_bytes:d.release_bytes,..metadata()})),work:1}}
  crate::close_ledger(&mut self.ledger,grant)
 }
}
impl Drop for RetainedJobPageTransfer{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"detached original page requires paid close");}}
impl RetainedJobPayload{
 pub fn original_page_transfer_demand(&self,output:&Option<RetainedJobPageTransfer>)->Result<D,ValueError>{
  if let Some(owner)=output{
   if owner.stream!=self.stream||self.payload_authority_identity().is_some_and(|identity|identity!=owner.source){return Err(refused("original page transfer changed its source ledger or stream"))}
   if owner.is_ready(){return Ok(Default::default())}
   if self.page_count!=owner.index+1{return Err(refused("original page transfer changed its pending page frontier"))}
  }
  if self.page_count==0{return Ok(Default::default())}
  Ok(D{depth:1,..Default::default()})
 }
 pub fn transfer_last_original_page(&mut self,output:&mut Option<RetainedJobPageTransfer>,grant:RetainedCloneGrant)->O{
  let d=match self.original_page_transfer_demand(output){Ok(d)=>d,Err(error)=>return O{result:Err(error),work:0}};
  if output.as_ref().is_some_and(RetainedJobPageTransfer::is_ready)||self.page_count==0{return O{result:Ok(S::Complete(Default::default())),work:0}}
  if !admitted(grant,d){return O{result:Ok(S::Progress(Default::default())),work:0}}
  let index=self.page_count-1;
  if output.is_none(){
   let Some(source)=self.payload_authority_identity()else{return O{result:Err(refused("original page transfer has no live operation ledger")),work:0}};
   if self.page_count>1{*output=Some(RetainedJobPageTransfer{page:None,ledger:Some(Arc::clone(self.ledger.as_ref().unwrap())),stream:self.stream,index,source});return O{result:Ok(S::Progress(metadata())),work:1}}
   let page=self.pop_page().expect("original payload page frontier");
   *output=Some(RetainedJobPageTransfer{page:Some(page),ledger:self.ledger.take(),stream:self.stream,index,source});
  }else{
   let page=self.pop_page().expect("retained pending original page");output.as_mut().unwrap().page=Some(page);
  }
  O{result:Ok(S::Complete(metadata())),work:1}
 }
}
