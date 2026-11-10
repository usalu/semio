//! 🧪️ Ordered Window sources preserve original callers and private owners across every admission and cancellation frontier.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,default_now_us};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_ownership::original_store_genesis::OriginalStoreBirth;
fn law()->serde_json::Value{serde_json::from_str(include_str!("🔣️.json")).unwrap()}
fn grant()->RetainedCloneGrant{let l=law();let g=&l["turn"];RetainedCloneGrant{maximum_items:g["items"].as_u64().unwrap()as usize,maximum_copy_bytes:g["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["depth"].as_u64().unwrap()as usize}}
fn caller(id:u64)->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(id),Generation(47),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},default_now_us,u64::MAX,1<<20)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
#[derive(Clone,Copy)]
struct Registry{identity:u64,values:[u8;2]}
impl Registry{fn new()->Self{Self{identity:19,values:[0;2]}}}
struct Owner<const SLOT:usize,const VALUE:u8>{binding:OriginalSourceBinding,birth:OriginalStoreBirth,unit:Option<()>,done:bool}
impl<const SLOT:usize,const VALUE:u8> OriginalWindowAdmission for Owner<SLOT,VALUE>{
 type Registry=Registry;
 fn registry_identity(registry:&Self::Registry)->u64{registry.identity}
 fn admit(_: &mut Self::Registry,cx:&mut StepContext<'_>,birth:&mut OriginalStoreBirth,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure>{let d=frame_birth::<Self>();let binding=OriginalSourceBinding::borrow(cx);birth.built(cx,output,construction(d),||Ok((Box::new(Self{binding,birth:OriginalStoreBirth::default(),unit:None,done:false}),receipt(d))))?;Ok(output.is_some())}
 fn advance(&mut self,registry:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx){return Err(invalid("fixture Window admission changed its source").into())}if self.done{return Ok(true)}let d=RetirementDemand{copy_bytes:2,depth:1,..Default::default()};let ready=self.birth.built(cx,&mut self.unit,construction(d),||Ok(((),receipt(d))))?;if ready{self.unit=None;registry.values[SLOT]=VALUE;self.done=true;}Ok(self.done)}
 fn close_step(&mut self,_:&mut Self::Registry,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !self.birth.terminal_is_empty(){self.birth.close_step(cx)?;return Ok(false)}let d=RetirementDemand{copy_bytes:1,depth:1,..Default::default()};close_original_retained_step(cx,d,|_|{self.done=true;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(receipt(d))),work:1}})?;Ok(self.done)}
 fn terminal_is_empty(&self)->bool{self.done&&self.birth.terminal_is_empty()}
 fn boxed_release_demands()->RetirementDemand{frame_release::<Self>()}
}
type Windows=OriginalWindowSequence<Owner<0,17>,OriginalWindowSequence<Owner<1,29>,OriginalNoWindows<Registry>>>;
#[test]
fn ordered_windows_keep_original_grants_and_close_every_partial_admission(){
 let law=law();let maximum=law["maximumTurns"].as_u64().unwrap()as usize;
 for cut in std::iter::once(usize::MAX).chain(law["cancelAfter"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize)){
  let mut caller=caller(7121);let mut decoder=OriginalJobDecodeOwner::empty();let mut source=None;let mut registry=Registry::new();let mut birth=OriginalStoreBirth::default();
  for _ in 0..8{if turn(&mut caller,&mut decoder,grant(),|cx|Windows::admit(&mut registry,cx,&mut birth,&mut source)){break;}}assert!(source.is_some());
  for _ in 0..cut.min(maximum){if turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().advance(&mut registry,cx)){break;}}
  if cut==usize::MAX{assert!(source.as_ref().unwrap().ready());assert_eq!(registry.values,[law["owners"][0].as_u64().unwrap()as u8,law["owners"][1].as_u64().unwrap()as u8]);}
  assert!(registry.values==[0,0]||registry.values==[17,0]||registry.values==[17,29]);
  let before=registry.values;
  caller.cancel().cancel_now();
  for _ in 0..maximum{if source.as_ref().unwrap().terminal_is_empty(){break;}turn(&mut caller,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|source.as_mut().unwrap().close_step(if cut==usize::MAX{None}else{Some(&mut registry)},cx)));}
  assert!(source.as_ref().unwrap().terminal_is_empty());assert_eq!(registry.values,before);
  turn(&mut caller,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|release_frame(&mut source,cx,Windows::boxed_release_demands())));
  assert!(source.is_none());for _ in 0..128{if birth.terminal_is_empty(){break}turn(&mut caller,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|birth.close_step(cx)));}assert!(decoder.terminal_is_empty());caller.retire(grant());
 }
}
#[test]
fn ordered_window_birth_refuses_each_currency_and_a_foreign_source(){
 for axis in law()["deniedAxes"].as_array().unwrap(){let mut c=caller(7122);let mut decoder=OriginalJobDecodeOwner::empty();let mut source=None;let mut registry=Registry::new();let mut birth=OriginalStoreBirth::default();let mut g=grant();match axis.as_str().unwrap(){"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes=0,"capacity"=>g.maximum_capacity_bytes=0,_=>g.maximum_depth=0}assert!(!turn(&mut c,&mut decoder,g,|cx|Windows::admit(&mut registry,cx,&mut birth,&mut source)));assert!(source.is_none());assert_eq!(c.receipt(),RetainedCloneProgress::default());c.retire(grant());}
 let mut c=caller(7123);let mut decoder=OriginalJobDecodeOwner::empty();let mut source=None;let mut registry=Registry::new();let mut birth=OriginalStoreBirth::default();for _ in 0..8{if turn(&mut c,&mut decoder,grant(),|cx|Windows::admit(&mut registry,cx,&mut birth,&mut source)){break;}}
 let original_identity=registry.identity;registry.identity+=1;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||c.turn_with_decoder(grant(),&mut decoder,|cx|source.as_mut().unwrap().advance(&mut registry,cx)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));registry.identity=original_identity;
 let mut foreign=caller(7124);let mut foreign_decoder=OriginalJobDecodeOwner::empty();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn_with_decoder(grant(),&mut foreign_decoder,|cx|source.as_mut().unwrap().advance(&mut registry,cx)));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registry.values,[0,0]);foreign.retire(grant());
 for _ in 0..128{if source.as_ref().unwrap().terminal_is_empty(){break;}turn(&mut c,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|source.as_mut().unwrap().close_step(Some(&mut registry),cx)));}
 turn(&mut c,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|release_frame(&mut source,cx,Windows::boxed_release_demands())));for _ in 0..128{if birth.terminal_is_empty(){break}turn(&mut c,&mut decoder,grant(),|cx|cx.with_original_retirement(|cx|birth.close_step(cx)));}c.retire(grant());
}
