//! ♻️ An actual original caller funds cursor birth and keeps a failed partial release closeable.
use super::*;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
struct Probe{bytes:Vec<u8>,work:u64}
impl RetireOwned for Probe{
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Cursor>())}
 fn controlled_retirement_supported()->bool{true}
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(Cursor{bytes:self.bytes,work:self.work})}
}
struct Cursor{bytes:Vec<u8>,work:u64}
impl RetirementCursor for Cursor{
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(std::mem::size_of::<Vec<u8>>())}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.bytes.capacity())}
 fn terminal_release_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
 fn terminal_is_empty(&self)->bool{self.bytes.capacity()==0}
 fn close_step(&mut self,_:RetainedCloneGrant)->RetirementStep{if self.bytes.capacity()==0{return RetirementStep::Complete(0)}let bytes=self.bytes.capacity();drop(std::mem::take(&mut self.bytes));let p=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Vec<u8>>(),released_bytes:bytes,..Default::default()};RetirementStep::Failure(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"funded partial release probe").with_retained_progress(p),self.work)}
}
#[test]
fn work_closing_keeps_original_birth_denial_and_actual_failed_child_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/♻️retirement/🔣️.json")).unwrap();let mut caller=OriginalWorkFixture::new();let stock=OriginalWorkFixture::stock();let bytes=law["payloadBytes"].as_u64().unwrap()as usize;let mut value=None;
 caller.turn(stock,|cx|{semio_framework_job::admit_original_construction(cx,&mut value,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes+2*std::mem::size_of::<Probe>(),capacity_bytes:bytes,release_bytes:0,depth:1},|_|{let data=vec![19;bytes];let p=RetainedCloneProgress{copied_items:1,copied_bytes:bytes+2*std::mem::size_of::<Probe>(),retained_capacity_bytes:data.capacity(),..Default::default()};RetainedWorkOutcome{result:Ok((Probe{bytes:data,work:law["childWork"].as_u64().unwrap()},p)),work:1}})?;Ok(())}).unwrap();let value=value.unwrap();let pointer=value.bytes.as_ptr();let mut closing=WorkClosing::default();closing.stage(value);
 caller.turn(stock,|cx|{let before=(cx.retained_progress(),cx.fuel_remaining());let denied=RetainedCloneGrant{maximum_capacity_bytes:law["deniedCapacity"].as_u64().unwrap()as usize,..cx.retained_grant()};cx.with_retained_loan(denied,|cx|{let step=cx.with_original_retirement(|cx|closing.close_step(cx))?;assert_eq!(step.progress(),Default::default());Ok(())})?;assert_eq!(cx.retained_progress(),before.0);assert_eq!(cx.fuel_remaining(),before.1);Ok(())}).unwrap();assert_eq!(closing.staged.as_ref().unwrap().bytes.as_ptr(),pointer);
 let mut failed=false;for _ in 0..1024{if closing.is_empty(){break}let mut delta=(0,0);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{let before=(cx.retained_progress(),cx.fuel_remaining());let result=cx.with_original_retirement(|cx|closing.close_step(cx));delta=(cx.retained_progress().released_bytes-before.0.released_bytes,before.1-cx.fuel_remaining());result}));assert_eq!(delta.0,heap.released_bytes);if let Err(error)=result{assert!(!failed);failed=true;assert_eq!(delta,(law["failedReleaseBytes"].as_u64().unwrap()as usize,law["childWork"].as_u64().unwrap()));assert_eq!(error.effects.released_bytes,bytes);assert!(!closing.is_empty());caller.retire_owned(error,stock);}}assert!(failed);assert!(closing.is_empty());caller.finish();
}
