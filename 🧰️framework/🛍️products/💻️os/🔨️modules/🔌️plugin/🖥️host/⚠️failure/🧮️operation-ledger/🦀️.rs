//! 🧮️ Guest operation imports cross one reservation ledger sized by the issued semantic capacity.
use semio_framework_value::{ValueError,ValueRefusalKind,native_encoding::{NativeAllocationRequest,NativeEncodeProgress}};
use semio_plugin_host_guest_runtime::OriginalOperationReceiving;
fn refused()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest operation ledger differs from its issued semantic capacity")}
/// 🪙️ Declared allocations stay sealed by the issued semantic capacity.
pub struct GuestOperationLedger{capacity:usize,reserved:usize,phase_owned:usize,maximum:usize,returned:usize,output:usize,active:bool,finished:bool}
impl GuestOperationLedger{
 pub fn new(capacity:usize)->Self{Self{capacity,reserved:0,phase_owned:0,maximum:0,returned:0,output:0,active:false,finished:false}}
 fn reserve(&mut self,bytes:usize)->Result<(),ValueError>{self.reserved=self.reserved.checked_add(bytes).filter(|next|*next<=self.capacity).ok_or_else(refused)?;Ok(())}
}
impl OriginalOperationReceiving for GuestOperationLedger{
 fn begin(&mut self)->Result<usize,ValueError>{if self.active{return Err(refused())}self.maximum=self.capacity.checked_sub(self.reserved).ok_or_else(refused)?;self.phase_owned=0;self.active=true;self.finished=false;Ok(self.maximum)}
 fn progress(&mut self,next:NativeEncodeProgress)->Result<(),ValueError>{if !self.active||next.owned_bytes!=self.phase_owned||(next.total!=0&&next.completed>next.total){return Err(refused())}Ok(())}
 fn allocation(&mut self,next:NativeAllocationRequest)->Result<(),ValueError>{if !self.active||next.maximum_bytes>self.maximum||next.owned_bytes!=self.phase_owned||self.phase_owned.checked_add(next.bytes)!=Some(next.next_owned_bytes)||next.next_owned_bytes>next.maximum_bytes{return Err(refused())}self.reserve(next.bytes)?;self.phase_owned=next.next_owned_bytes;Ok(())}
 fn finish(&mut self,owned:usize)->Result<(),ValueError>{if !self.active||owned!=self.phase_owned{return Err(refused())}self.active=false;self.finished=true;Ok(())}
 fn reserve_return(&mut self,bytes:usize)->Result<(),ValueError>{if !self.active{return Err(refused())}let next=self.returned.checked_add(bytes).ok_or_else(refused)?;self.reserve(bytes)?;self.returned=next;Ok(())}
 fn reserved_return_bytes(&self)->usize{self.returned}
 fn verify_reserved_return(&self,bytes:usize)->Result<(),ValueError>{self.ensure_finished()?;if self.returned==bytes{Ok(())}else{Err(refused())}}
 fn receive_output(&mut self,bytes:usize)->Result<(),ValueError>{if !self.finished{return Err(refused())}let next=self.output.checked_add(bytes).ok_or_else(refused)?;self.reserve(bytes)?;self.output=next;Ok(())}
 fn ensure_finished(&self)->Result<(),ValueError>{if self.active{Err(refused())}else{Ok(())}}
}
#[cfg(test)]
mod tests{
 use super::*;
 /// 🧪️ One export reserves, allocates and finishes inside its capacity and an allocation past it is refused.
 #[test]
 fn the_ledger_admits_one_phase_inside_its_capacity_and_refuses_the_excess(){
  let mut ledger=GuestOperationLedger::new(1024);
  assert_eq!(ledger.begin().unwrap(),1024);
  ledger.allocation(NativeAllocationRequest{bytes:600,owned_bytes:0,next_owned_bytes:600,maximum_bytes:1024}).unwrap();
  assert!(ledger.allocation(NativeAllocationRequest{bytes:600,owned_bytes:600,next_owned_bytes:1200,maximum_bytes:1024}).is_err());
  ledger.progress(NativeEncodeProgress{completed:1,total:2,owned_bytes:600}).unwrap();
  ledger.finish(600).unwrap();
  ledger.ensure_finished().unwrap();
  assert!(ledger.begin().is_ok());
 }
 /// 🧪️ Reserved return bytes verify exactly once the phase finished.
 #[test]
 fn reserved_returns_verify_only_after_the_phase_finished(){
  let mut ledger=GuestOperationLedger::new(64);
  ledger.begin().unwrap();
  ledger.reserve_return(16).unwrap();
  assert!(ledger.verify_reserved_return(16).is_err());
  ledger.finish(0).unwrap();
  ledger.verify_reserved_return(16).unwrap();
  assert!(ledger.verify_reserved_return(8).is_err());
  ledger.receive_output(16).unwrap();
  assert_eq!(ledger.reserved_return_bytes(),16);
 }
}
