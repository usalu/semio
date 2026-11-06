//! 🧮️ Declared guest reservations remain sealed by the actual held semantic loan until producer retirement.
use semio_framework_job::{OriginalSourceBinding,StepContext};
use semio_plugin_host_guest_runtime::OriginalOperationReceiving;
use semio_plugin_host_wake::{GuestRelayGuestSubloan,GuestRelayGuestPurpose};
use semio_framework_value::{ValueError,ValueRefusalKind,native_encoding::{NativeEncodeProgress,NativeAllocationRequest}};
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"codec receiving ledger differs from its original held semantic source")}
struct Reservations{capacity:usize,reserved:usize,phase_owned:usize,maximum:usize,returned:usize,output:usize,active:bool,finished:bool}
impl Reservations{
 fn new(capacity:usize)->Self{Self{capacity,reserved:0,phase_owned:0,maximum:0,returned:0,output:0,active:false,finished:false}}
 fn reserve(&mut self,bytes:usize)->Result<(),ValueError>{let next=self.reserved.checked_add(bytes).filter(|next|*next<=self.capacity).ok_or_else(refused)?;self.reserved=next;Ok(())}
 fn begin(&mut self)->Result<usize,ValueError>{if self.active{return Err(refused())}self.maximum=self.capacity.checked_sub(self.reserved).ok_or_else(refused)?;self.phase_owned=0;self.active=true;self.finished=false;Ok(self.maximum)}
 fn allocation(&mut self,next:NativeAllocationRequest)->Result<(),ValueError>{if !self.active||next.maximum_bytes>self.maximum||next.owned_bytes!=self.phase_owned||self.phase_owned.checked_add(next.bytes)!=Some(next.next_owned_bytes)||next.next_owned_bytes>next.maximum_bytes{return Err(refused())}self.reserve(next.bytes)?;self.phase_owned=next.next_owned_bytes;Ok(())}
 fn progress(&self,next:NativeEncodeProgress)->Result<(),ValueError>{if !self.active||next.owned_bytes!=self.phase_owned||(next.total!=0&&next.completed>next.total){return Err(refused())}Ok(())}
 fn finish(&mut self,owned:usize)->Result<(),ValueError>{if !self.active||owned!=self.phase_owned{return Err(refused())}self.active=false;self.finished=true;Ok(())}
 fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>{if !self.active{return Err(refused())}let next=self.returned.checked_add(bytes).ok_or_else(refused)?;self.reserve(bytes)?;self.returned=next;Ok(())}
 fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>{if !self.finished{return Err(refused())}let next=self.output.checked_add(bytes).ok_or_else(refused)?;self.reserve(bytes)?;self.output=next;Ok(())}
 fn ensure_finished(&self)->Result<(),ValueError>{if self.active{Err(refused())}else{Ok(())}}
}
pub(crate) struct GuestCodecReceivingLedger{loan:GuestRelayGuestSubloan,binding:OriginalSourceBinding,epoch:Option<u64>,reservations:Reservations}
impl GuestCodecReceivingLedger{
 pub(crate) fn new(loan:GuestRelayGuestSubloan,cx:&StepContext<'_>)->Result<Self,ValueError>{cx.original_source_parent()?.validate_borrowed_context(cx)?;let budget=loan.budget();if loan.purpose()!=GuestRelayGuestPurpose::Semantic||(budget.input.operation,budget.input.generation)!=(cx.operation().0,cx.generation().0)||Some(budget.input.work.clock_binding)!=cx.original_clock_binding()||budget.input.work.absolute_deadline_us!=cx.original_absolute_deadline_us(){return Err(refused())}Ok(Self{loan,binding:OriginalSourceBinding::borrow(cx),epoch:None,reservations:Reservations::new(budget.retained_policy.maximum_capacity_bytes)})}
 pub(crate) fn bind(&mut self,cx:&StepContext<'_>)->Result<GuestCodecReceiving<'_>,ValueError>{cx.original_source_parent()?.validate_borrowed_context(cx)?;let epoch=semio_framework_actor::RetainedTurnInput::borrow_original(cx)?.epoch;if !self.binding.matches(cx)||self.epoch.is_some_and(|prior|epoch<=prior){return Err(refused())}self.epoch=Some(epoch);Ok(GuestCodecReceiving{reservations:&mut self.reservations})}
 pub(crate) fn held_loan(&self)->GuestRelayGuestSubloan{self.loan}
}
pub(crate) struct GuestCodecReceiving<'a>{reservations:&'a mut Reservations}
impl OriginalOperationReceiving for GuestCodecReceiving<'_>{
 fn begin(&mut self)->Result<usize,ValueError>{self.reservations.begin()}
 fn progress(&mut self,next:NativeEncodeProgress)->Result<(),ValueError>{self.reservations.progress(next)}
 fn allocation(&mut self,next:NativeAllocationRequest)->Result<(),ValueError>{self.reservations.allocation(next)}
 fn finish(&mut self,owned:usize)->Result<(),ValueError>{self.reservations.finish(owned)}
 fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>{self.reservations.reserve_return(bytes)}
 fn reserved_return_bytes(&self)->usize{self.reservations.returned}
 fn verify_reserved_return(&self,bytes:usize)->Result<(),ValueError>{self.reservations.ensure_finished()?;if self.reservations.returned==bytes{Ok(())}else{Err(refused())}}
 fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>{self.reservations.receive_output(bytes)}
 fn ensure_finished(&self)->Result<(),ValueError>{self.reservations.ensure_finished()}
}
#[cfg(test)]
mod tests{
use super::*;
#[test]
fn codec_declared_reservations_do_not_turn_gauges_into_source_spend(){let fixture:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let mut state=Reservations::new(fixture["capacity"].as_u64().unwrap()as usize);for event in fixture["events"].as_array().unwrap(){let amount=event["amount"].as_u64().unwrap()as usize;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||match event["operation"].as_str().unwrap(){"begin"=>state.begin().map(|_|()),"allocate"=>state.allocation(NativeAllocationRequest{bytes:amount,owned_bytes:state.phase_owned,next_owned_bytes:state.phase_owned+amount,maximum_bytes:state.maximum}),"return"=>state.reserve_return(amount),"finish"=>state.finish(amount),"output"=>state.receive_output(amount),"progress"=>state.progress(NativeEncodeProgress{completed:amount,total:amount,owned_bytes:state.phase_owned}),_=>unreachable!()});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(result.is_ok(),event["accepted"].as_bool().unwrap());assert_eq!(state.reserved,event["reserved"].as_u64().unwrap()as usize);}assert!(state.ensure_finished().is_ok());}
}
