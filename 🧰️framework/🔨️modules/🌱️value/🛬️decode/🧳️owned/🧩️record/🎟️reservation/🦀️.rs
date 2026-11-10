//! 🎟️ Sequence reservation retains paid selection and the actual original allocation callback.
pub use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
pub use semio_framework_value_retirement::{retirement,OriginalRetirementFailure,OriginalRetirementOutcome};
use retirement::RetireOwned;
pub use semio_framework_value_derive::RetireOwned;
use semio_framework_value_native_decoding::native_decoding::{self,NativeDecodeControl,OriginalReceivingPrimary};
use semio_framework_value_native_receiving_contract::{native_work_limit,work_sum,selected_native_work,NativeRecoveryLoan};
use semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;
fn invalid()->ValueError{ValueError::invariant("native reservation state is invalid")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
use native_decoding::construction::{NativeReceivingEffect,NativeReceivingEffectPhase};
use semio_framework_value_paged_list::PagedListReservationSelection;
pub trait NativeSequenceReservation:Default+RetireOwned{
 fn capture_plan(&self)->Result<Option<semio_framework_value_native_decoding::native_decoding::construction::NativeEffectCapturePlan>,ValueError>;
 fn has_pending_recovery(&self)->bool;
 fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>;
 fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>;
 fn close_demand(&self)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self)->Result<u64,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>;
 fn terminal_is_empty(&self)->bool;
}
impl NativeSequenceReservation for (){
 fn capture_plan(&self)->Result<Option<semio_framework_value_native_decoding::native_decoding::construction::NativeEffectCapturePlan>,ValueError>{Ok(None)}
 fn has_pending_recovery(&self)->bool{false}
 fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>{None}
 fn resume_recovery(&mut self,_:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"empty reservation has no original callback").into()),work:0}}
 fn close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(Default::default())}
 fn close_work_demand(&self)->Result<u64,ValueError>{Ok(0)}
 fn close_step(&mut self,_:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
 fn terminal_is_empty(&self)->bool{true}
}
#[derive(RetireOwned)]
#[value(crate="crate")]
pub struct NativePagedReservation{header:Option<semio_framework_value_paged_list::PagedListSelectionHeader>,selection:Option<PagedListReservationSelection>,context:Option<(usize,usize)>,finished:bool,complete:bool,effect:NativeReceivingEffect}
impl Default for NativePagedReservation{fn default()->Self{Self{header:None,selection:None,context:None,finished:false,complete:false,effect:Default::default()}}}
impl NativePagedReservation{
 fn header_demand()->RetirementDemand{fixed(2*size_of::<Option<semio_framework_value_paged_list::PagedListSelectionHeader>>()+size_of::<Option<(usize,usize)>>()+size_of::<bool>())}
 fn selection_demand()->RetirementDemand{fixed(2*size_of::<Option<semio_framework_value_paged_list::PagedListSelectionHeader>>()+2*size_of::<Option<PagedListReservationSelection>>()+size_of::<bool>())}
 fn clear_demand()->RetirementDemand{fixed(size_of::<Option<PagedListReservationSelection>>()+size_of::<bool>())}
 fn close_metadata_demand()->RetirementDemand{fixed(size_of::<Option<semio_framework_value_paged_list::PagedListSelectionHeader>>()+size_of::<Option<PagedListReservationSelection>>()+size_of::<Option<(usize,usize)>>()+2*size_of::<bool>())}
 fn validate<const N:usize>(&self,index:usize,limit:usize)->Result<(),ValueError>{if index>limit||limit>N||self.context.is_some_and(|context|context!=(index,limit))&&!self.complete{return Err(invalid())}Ok(())}
 pub fn demand<T,const N:usize>(&self,_storage:&semio_framework_value_paged_list::PagedList<T,N>,index:usize,limit:usize)->Result<Option<RetirementDemand>,ValueError>{
  self.validate::<N>(index,limit)?;
  if self.complete&&self.context==Some((index,limit)){return Ok(None)}
  if self.finished{return Ok(Some(Self::clear_demand()))}
  if let Some(selection)=self.selection.as_ref(){let mut d=selection.demand();d.copy_bytes=d.copy_bytes.checked_add(size_of::<bool>()).ok_or_else(invalid)?;return Ok(Some(d))}
  Ok(Some(if self.header.is_some(){Self::selection_demand()}else{Self::header_demand()}))
 }
 pub fn work_demand<T,const N:usize>(&self,storage:&semio_framework_value_paged_list::PagedList<T,N>,index:usize,limit:usize,maximum:u64)->Result<u64,ValueError>{
  if self.effect.has_pending_recovery(){return Err(ValueError::invariant("sequence reservation requires its original receiving recovery"))}
  if self.demand(storage,index,limit)?.is_none(){return Ok(0)}
  let work=if self.finished{2}else if let Some(selection)=self.selection.as_ref(){work_sum(selection.reserve_work_demand(),1)?}else if let Some(header)=self.header.as_ref(){work_sum(header.reservation_selection_work_demand(limit),4)?}else{4};Ok(selected_native_work(work,maximum))
 }
 pub fn advance<T,const N:usize>(&mut self,storage:&mut semio_framework_value_paged_list::PagedList<T,N>,index:usize,limit:usize,control:&mut NativeDecodeControl<'_>,maximum:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>{
  let mut work=0;let result=(||{let maximum=native_work_limit(control,maximum)?;let quoted=self.work_demand(storage,index,limit,maximum)?;if quoted==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let demand=self.demand(storage,index,limit)?.ok_or_else(invalid)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let Self{header,selection,context,finished,complete,effect}=self;
   let turn=if *finished{control.receive_allocation(demand,effect,false,quoted,|_,_|{*selection=None;*finished=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:2}})}
   else if let Some(selected)=selection.as_ref(){control.receive_allocation(demand,effect,true,quoted,|issued,actual|{
    *finished=true;let turn=storage.reserve_selected(selected,issued,maximum-1,actual.expect("original allocation frame supplies its physical marker"));let work=turn.as_ref().map(|p|p.work).unwrap_or_else(|e|e.work).checked_add(1).expect("quoted reservation work");let result=match turn{Ok(p)=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:p.placed_bytes+size_of::<bool>(),retained_capacity_bytes:p.allocated_bytes,released_bytes:p.released_allocation_bytes})),Err(e)=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason).with_retained_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),retained_capacity_bytes:e.allocated_bytes,..Default::default()}))};RetainedWorkOutcome{result,work:u128::from(work)}
   })}
   else if let Some(paid)=*header{control.receive_allocation(demand,effect,false,quoted,|_,_|{let turn=storage.select_reservation_from_header(&paid,limit,maximum-4);let work=turn.work+1;match turn.result{Ok(selected)=>{*complete=selected.is_none();*selection=selected;*header=None;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:work+3}},Err(e)=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason)),work}}})}
   else{control.receive_allocation(demand,effect,false,quoted,|_,_|{let turn=storage.inspect_selection_header(maximum-3);let work=turn.work;match turn.result{Ok(Some(paid))=>{*header=Some(paid);*context=Some((index,limit));*complete=false;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(demand))),work:work+3}},Ok(None)=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},Err(e)=>RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason)),work}}})};
work=turn.work;turn.result.map(RetainedCloneStep::Progress)
  })();RetainedWorkOutcome{result,work}
 }
}
impl NativeSequenceReservation for NativePagedReservation{
 fn capture_plan(&self)->Result<Option<semio_framework_value_native_decoding::native_decoding::construction::NativeEffectCapturePlan>,ValueError>{Ok(Some(self.effect.capture_plan()))}
 fn has_pending_recovery(&self)->bool{self.effect.has_pending_recovery()}
 fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>{self.effect.pending_recovery().map(NativeRecoveryLoan::Value)}
 fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{control.finish_receiving(&mut self.effect)}
 fn close_demand(&self)->Result<RetirementDemand,ValueError>{if self.effect.phase()!=NativeReceivingEffectPhase::Empty{return Err(ValueError::invariant("sequence reservation retains its original callbacks"))}Ok(if self.terminal_is_empty(){Default::default()}else{Self::close_metadata_demand()})}
 fn close_work_demand(&self)->Result<u64,ValueError>{self.close_demand()?;Ok(if self.terminal_is_empty(){0}else{5})}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{let result=(||{let d=self.close_demand()?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.header=None;self.selection=None;self.context=None;self.finished=false;self.complete=false;Ok(RetainedCloneStep::Complete(receipt(d)))})();let work=if matches!(&result,Ok(RetainedCloneStep::Complete(p))if *p!=Default::default()){5}else{0};RetainedWorkOutcome{result,work}}
 fn terminal_is_empty(&self)->bool{self.effect.phase()==NativeReceivingEffectPhase::Empty&&self.header.is_none()&&self.selection.is_none()&&self.context.is_none()&&!self.finished&&!self.complete}
}
