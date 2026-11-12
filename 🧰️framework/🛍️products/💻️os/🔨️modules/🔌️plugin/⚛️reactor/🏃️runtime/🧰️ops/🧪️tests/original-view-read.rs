//! 🧪️ Original registry pins retain real view buffers without cloning and restore every canceled prefix.
use semio_plugin_plugin_program::surface_context::{OriginalSurfaceViewRead,OriginalSurfaceViewCapture,SurfaceContexts};
use semio_framework::{ViewModel};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalTurnFailure,StepContext,OperationId,Generation,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand};
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_actor::retained_context::{OriginalActorContextSlot,OriginalActorCallError};
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{OriginalSourceBinding,StepBudget,receive_original_retained_work};
fn clock()->Option<u64>{Some(1000)}
struct Rig{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner,stock:RetainedCloneGrant,calls:usize,born:usize,released:usize,seed:usize}
impl Rig{
 fn new(policy:&serde_json::Value,seed:usize)->Self{
  let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
  let(caller,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap()));
  assert!(!heap.overflowed);Self{caller,decoder:OriginalJobDecodeOwner::empty(),stock,calls:0,born:heap.requested_bytes,released:heap.released_bytes,seed}
 }
 fn event<R>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
  self.calls+=1;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn_with_decoder(grant,&mut self.decoder,run));
  assert!(!heap.overflowed);let p=self.caller.receipt();assert!(p.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
  self.born+=heap.requested_bytes;self.released+=heap.released_bytes;result.unwrap()
 }
 fn turn<R>(&mut self,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{self.event(self.stock,run)}
 fn close<T:RetireOwned>(&mut self,value:T){
  let mut value=Some(value);let mut close=RetainedCloneClose::default();
  while value.is_some()||!close.is_empty(){self.turn(|cx|cx.with_original_retirement(|cx|{
   for _ in 0..512{if cx.retirement_should_yield(){break}let d=if !close.is_empty(){close.demands(cx.retained_grant().maximum_copy_bytes)?}else{close.next_owner_demands::<T>(true,&None,cx.retained_grant().maximum_copy_bytes)?};
    let body=cx.retained_grant().maximum_copy_bytes;let step=semio_framework_job::close_original_effect_step(cx,d,|effect|if !close.is_empty(){close.step_granted(body,effect)}else{semio_plugin_job_receiving::original_steps::begin(&mut close,&mut value,effect)})?;
    if value.is_none()&&close.is_empty()||step.progress()==Default::default(){break}
   }Ok(())
  }));}
 }
 fn registry(&mut self,registry:&mut SurfaceContexts){while !registry.original_terminal_is_empty(){self.turn(|cx|cx.with_original_retirement(|cx|{for _ in 0..512{if registry.original_terminal_is_empty()||cx.retirement_should_yield(){break}let before=cx.retained_progress();registry.close_original_step(cx)?;if before==cx.retained_progress(){break}}Ok(())}));}}
 fn foreign_decoder<R>(&mut self,decoder:&mut OriginalJobDecodeOwner,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
  self.calls+=1;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn_with_decoder(self.stock,decoder,run));assert!(!heap.overflowed);let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));self.born+=heap.requested_bytes;self.released+=heap.released_bytes;result.unwrap()
 }
 fn close_foreign_decoder(&mut self,decoder:&mut OriginalJobDecodeOwner){
  while !decoder.terminal_is_empty(){self.calls+=1;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(self.stock,|cx|cx.with_original_retirement(|cx|{
   for _ in 0..512{if decoder.terminal_is_empty()||cx.retirement_should_yield(){break}let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let before=cx.retained_progress();let done=decoder.close_on_source(cx)?;if done||cx.retained_progress()==before{break}}Ok(())
  })));assert!(!heap.overflowed);let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));self.born+=heap.requested_bytes;self.released+=heap.released_bytes;result.unwrap();}
 }
 fn finish(mut self){
  while !self.decoder.terminal_is_empty(){
   self.calls+=1;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(self.stock,|cx|cx.with_original_retirement(|cx|{
    for _ in 0..512{if self.decoder.terminal_is_empty()||cx.retirement_should_yield(){break}let d=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;let before=cx.retained_progress();let done=self.decoder.close_on_source(cx)?;if done||cx.retained_progress()==before{break}}Ok(())
   })));
   assert!(!heap.overflowed);let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));self.born+=heap.requested_bytes;self.released+=heap.released_bytes;result.unwrap()
  }
  assert_eq!(self.caller.stock(),self.stock);assert!(self.caller.cumulative_receipt().fits(self.stock));
  while !self.caller.terminal_is_empty(){self.calls+=1;let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.retire_step(self.stock).unwrap());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));self.born+=heap.requested_bytes;self.released+=heap.released_bytes;}
  assert_eq!(self.born+self.seed,self.released,"all source, pin, decoder and caller allocations close");assert!(self.calls<=200,"original opportunity exhausted: {}",self.calls);
 }
}
fn pointers(view:&ViewModel)->(usize,usize,Option<usize>,Option<usize>){(view.panel_json.as_ref().map_or(0,|text|text.as_ptr()as usize),view.window_instances.as_ptr()as usize,view.active_utility_by_window_id.original_root_identity(),view.tool_run_trace_cursor_by_window_id.original_root_identity())}
fn selected<'a>(registry:&'a SurfaceContexts,surface:Option<&str>)->Option<&'a ViewModel>{match surface{Some(surface)=>registry.borrow(surface).map(|view|view.view_state),None=>registry.view()}}
fn release_read(read:&mut Option<Box<OriginalSurfaceViewRead>>,registry:&mut SurfaceContexts,rig:&mut Rig){
 while !read.as_ref().unwrap().terminal_is_empty(){rig.turn(|cx|{for _ in 0..512{if read.as_ref().unwrap().terminal_is_empty()||cx.retirement_should_yield(){break}let before=cx.retained_progress();let fuel=cx.fuel_remaining();cx.with_original_work_loan(fuel,1000,|cx|read.as_mut().unwrap().close_step(registry,cx))?;if before==cx.retained_progress(){break}}Ok(())});}
 rig.turn(|cx|cx.with_original_retirement(|cx|{let d=OriginalSurfaceViewRead::release_demand();close_original_retained_step(cx,d,|_|{read.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));
}
#[test]
fn original_surface_read_pins_actual_registry_without_view_clone_and_restores_all_prefixes(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️command/📥️custody/🛂️manifest/metadata.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for surface in [None,Some("selected-original-surface")]{let mut birth=None;for size in [0,8197]{for cut in [None,Some(0),Some(1),Some(2),Some(3),Some(4),Some(5),Some(6),Some(7),Some(8),Some(9),Some(10),Some(11)]{
  let mut wire=fixture["view"].clone();wire["panelJson"]=serde_json::Value::String("α🙂".repeat(size));
  let((mut registry,original),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let view:ViewModel=serde_json::from_value(wire.clone()).unwrap();let registry=match surface{Some(surface)=>SurfaceContexts::original_surface_read_fixture(surface,&view),None=>SurfaceContexts::original_read_fixture(&view)};(registry,view)
  });assert!(!seed.overflowed);
  let original_pointers=pointers(selected(&registry,surface).unwrap());let original_address=selected(&registry,surface).unwrap()as*const ViewModel as usize;
  let mut rig=Rig::new(&policy,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());let mut read=None;let mut read_admission=Default::default();
  let denied=RetainedCloneGrant{maximum_items:0,..rig.stock};assert!(!rig.event(denied,|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read)));assert!(read.is_none());
  let before=rig.born;rig.turn(|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read));let mut turns=0;
  loop{
   if cut==Some(turns){rig.caller.cancel().cancel_now();break}
   let ready=rig.turn(|cx|match surface{Some(surface)=>read.as_mut().unwrap().advance_surface(&mut registry,surface,cx),None=>read.as_mut().unwrap().advance(&mut registry,cx)});turns+=1;if ready{break}assert!(turns<1000)
  }
  if cut.is_none(){
   let admission_born=rig.born-before;if let Some(expected)=birth{assert_eq!(admission_born,expected,"pin birth is independent of original view text size")}else{birth=Some(admission_born)}
   for _ in 0..3{rig.turn(|cx|{let source=read.as_ref().unwrap().borrow(&registry,cx)?.unwrap();let actual=source.get();assert_eq!(pointers(actual),original_pointers);assert_eq!(actual as*const ViewModel,selected(&registry,surface).unwrap()as*const ViewModel);Ok(())});}
   let mut busy=None;let mut busy_admission=Default::default();rig.turn(|cx|OriginalSurfaceViewRead::admit(cx,&mut busy_admission,&mut busy));assert!(!rig.turn(|cx|busy.as_mut().unwrap().advance(&mut registry,cx)));release_read(&mut busy,&mut registry,&mut rig);
   assert_eq!(pointers(selected(&registry,surface).unwrap()),original_pointers);
   let copied_surface=surface.map(str::to_owned);
   if let Some(copied)=copied_surface.as_deref(){let failure=rig.turn(|cx|Ok(read.as_mut().unwrap().advance_surface(&mut registry,copied,cx).err().expect("foreign selector allocation")));rig.close(failure)}
   let mut decoder=OriginalJobDecodeOwner::empty();let failure=rig.foreign_decoder(&mut decoder,|cx|Ok(read.as_ref().unwrap().borrow(&registry,cx).err().expect("original decoder recipient changed")));rig.close(failure);rig.close_foreign_decoder(&mut decoder);
   let mut foreign=Rig::new(&policy,0);let failure=foreign.turn(|cx|Ok(read.as_ref().unwrap().borrow(&registry,cx).err().expect("same labels on another genuine caller")));foreign.close(failure);foreign.finish();
   let other=SurfaceContexts::default();let failure=rig.turn(|cx|Ok(read.as_ref().unwrap().borrow(&other,cx).err().expect("foreign registry")));rig.close(failure);
  }
  release_read(&mut read,&mut registry,&mut rig);
  assert_eq!(pointers(selected(&registry,surface).unwrap()),original_pointers);assert_eq!(selected(&registry,surface).unwrap()as*const ViewModel as usize,original_address);
  rig.registry(&mut registry);rig.close(original);rig.finish();
 }}}
}
#[test]
fn original_surface_read_pending_is_not_absence_and_denials_leave_the_owner_unchanged(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️command/📥️custody/🛂️manifest/metadata.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let((mut registry,original),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();(SurfaceContexts::original_read_fixture(&view),view)});assert!(!seed.overflowed);
 let mut rig=Rig::new(&policy,seed.requested_bytes-seed.released_bytes);let mut read=None;let mut read_admission=Default::default();
 let d=OriginalSurfaceViewRead::birth_demand();
 for axis in 0..4{let mut denied=rig.stock;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=d.copy_bytes-1,2=>denied.maximum_capacity_bytes=d.capacity_bytes-1,_=>denied.maximum_depth=d.depth-1}assert!(!rig.event(denied,|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read)));assert!(read.is_none());}
 rig.turn(|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read));
 let failure=rig.turn(|cx|Ok(read.as_ref().unwrap().borrow(&registry,cx).err().expect("pending original is not business absence")));rig.close(failure);
 for _ in 0..100{if read.as_ref().unwrap().ready(){break}let denied=RetainedCloneGrant{maximum_items:0,..rig.stock};assert!(!rig.event(denied,|cx|read.as_mut().unwrap().advance(&mut registry,cx)));rig.turn(|cx|read.as_mut().unwrap().advance(&mut registry,cx));}
 assert!(read.as_ref().unwrap().ready());
 while !read.as_ref().unwrap().terminal_is_empty(){let d=read.as_ref().unwrap().close_demands(&registry).unwrap();for axis in 0..5{let mut denied=rig.stock;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}assert!(!rig.event(denied,|cx|read.as_mut().unwrap().close_step(&mut registry,cx)));assert_eq!(read.as_ref().unwrap().close_demands(&registry).unwrap(),d);}rig.turn(|cx|read.as_mut().unwrap().close_step(&mut registry,cx));}
 release_read(&mut read,&mut registry,&mut rig);rig.registry(&mut registry);rig.close(original);rig.finish();
 let mut registry=SurfaceContexts::default();let mut rig=Rig::new(&policy,0);let mut read=None;let mut read_admission=Default::default();rig.turn(|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read));while !read.as_ref().unwrap().ready(){rig.turn(|cx|read.as_mut().unwrap().advance(&mut registry,cx));}
 assert!(rig.turn(|cx|Ok(read.as_ref().unwrap().borrow(&registry,cx)?.is_none())));release_read(&mut read,&mut registry,&mut rig);rig.registry(&mut registry);rig.finish();
}
#[test]
fn original_surface_clone_capture_still_materializes_its_typed_output_and_restores_the_same_source(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️command/📥️custody/🛂️manifest/metadata.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for cut in [None,Some(0),Some(1),Some(7),Some(32)]{
  let((mut registry,original),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();(SurfaceContexts::original_read_fixture(&view),view)});assert!(!seed.overflowed);
  let source_pointers=pointers(registry.view().unwrap());let mut rig=Rig::new(&policy,seed.requested_bytes-seed.released_bytes);let mut capture=None;let mut capture_admission=Default::default();rig.turn(|cx|OriginalSurfaceViewCapture::admit(cx,&mut capture_admission,&mut capture));let mut turns=0;let mut output=None;
  loop{if cut==Some(turns){rig.caller.cancel().cancel_now();break}let ready=rig.turn(|cx|{for _ in 0..512{if cx.should_yield(){return Ok(false)}let before=cx.retained_progress();if capture.as_mut().unwrap().advance(&mut registry,cx)?{return Ok(true)}if before==cx.retained_progress(){break}}Ok(false)});turns+=1;if ready{rig.turn(|cx|capture.as_mut().unwrap().take_into(&mut output,cx));break}assert!(turns<1000)}
  let matched=output.as_ref().is_none_or(|output|serde_json::to_value(output).unwrap()==serde_json::to_value(&original).unwrap());
  while !capture.as_ref().unwrap().terminal_is_empty(){rig.turn(|cx|{for _ in 0..512{if capture.as_ref().unwrap().terminal_is_empty()||cx.retirement_should_yield(){break}let before=cx.retained_progress();capture.as_mut().unwrap().close_step(&mut registry,cx)?;if before==cx.retained_progress(){break}}Ok(())});}
  rig.turn(|cx|cx.with_original_retirement(|cx|{let d=OriginalSurfaceViewCapture::release_demand();close_original_retained_step(cx,d,|_|{capture.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())}));
  assert_eq!(pointers(registry.view().unwrap()),source_pointers);rig.close(output);rig.registry(&mut registry);rig.close(original);rig.finish();assert!(matched);if cut.is_none(){assert!(turns>0)}
 }
}


struct ActorEpoch{slot:OriginalActorContextSlot,epoch:u64}
impl ActorEpoch{
 fn new()->Self{Self{slot:OriginalActorContextSlot::empty(),epoch:0}}
 fn input(&mut self,host:&mut StepContext<'_>)->Result<RetainedTurnInput,OriginalTurnFailure>{self.epoch=self.epoch.checked_add(1).unwrap();host.bind_original_source_epoch(self.epoch)?;Ok(RetainedTurnInput::borrow_original(host)?)}
 fn post(host:&mut StepContext<'_>,progress:RetainedCloneProgress,work:u64)->Result<(),OriginalTurnFailure>{receive_original_retained_work(host,RetainedWorkOutcome::<_,semio_framework_value::ValueError>{result:Ok(RetainedCloneStep::Progress(progress)),work:u128::from(work)})?;Ok(())}
 fn admit(&mut self,rig:&mut Rig){
  for _ in 0..16{
   let ready=rig.turn(|host|{let input=self.input(host)?;let call=self.slot.admit_step(input,StepBudget::new(host.fuel_remaining().min(512),host.deadline_us(),host.retained_grant()),clock);Self::post(host,call.receipt_spent,call.work_units_spent)?;Ok(call.result?.ready)});
   if ready{return}
  }
  panic!("actual actor context admission did not settle")
 }
 fn dispatch<R>(&mut self,host:&mut StepContext<'_>,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{
  let input=self.input(host)?;let budget=StepBudget::new(host.fuel_remaining().min(512),host.deadline_us(),host.retained_grant());let mut recipient=RetainedCloneProgress::default();
  let call=self.slot.dispatch_step(input,budget,&mut recipient,clock,run);
  Self::post(host,call.receipt_spent,call.work_units_spent)?;
  assert_eq!(recipient,call.receipt_spent);
  match call.result{Ok(value)=>Ok(value),Err(OriginalActorCallError::Authority(error))=>Err(error.into()),Err(OriginalActorCallError::Semantic(error))=>Err(error)}
 }
 fn turn<R>(&mut self,rig:&mut Rig,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{rig.turn(|host|self.dispatch(host,run))}
 fn retire_turn<R>(&mut self,rig:&mut Rig,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{rig.turn(|host|host.with_original_retirement(|host|self.dispatch(host,|cx|cx.with_original_retirement(run))))}
 fn close_value<T:RetireOwned>(&mut self,rig:&mut Rig,value:T){
  let mut value=Some(value);let mut close=RetainedCloneClose::default();
  for _ in 0..4096{
   if value.is_none()&&close.is_empty(){return}
   self.retire_turn(rig,|cx|{
    for _ in 0..512{
     if cx.retirement_should_yield()||value.is_none()&&close.is_empty(){break}
     let before=cx.retained_progress();let fuel=cx.fuel_remaining();
     let d=if !close.is_empty(){close.demands(cx.retained_grant().maximum_copy_bytes)?}else{close.next_owner_demands::<T>(true,&None,cx.retained_grant().maximum_copy_bytes)?};
     let body=cx.retained_grant().maximum_copy_bytes;semio_framework_job::close_original_effect_step(cx,d,|effect|if !close.is_empty(){close.step_granted(body,effect)}else{semio_plugin_job_receiving::original_steps::begin(&mut close,&mut value,effect)})?;
     if before==cx.retained_progress()&&fuel==cx.fuel_remaining(){break}
    }Ok(())
   });
  }
  panic!("actual actor value retirement did not settle")
 }
 fn close_read(&mut self,rig:&mut Rig,read:&mut Option<Box<OriginalSurfaceViewRead>>,registry:&mut SurfaceContexts){
  for _ in 0..4096{
   if read.as_ref().unwrap().terminal_is_empty(){break}
   self.retire_turn(rig,|cx|{for _ in 0..512{if read.as_ref().unwrap().terminal_is_empty()||cx.retirement_should_yield(){break}let before=cx.retained_progress();let fuel=cx.fuel_remaining();read.as_mut().unwrap().close_step(registry,cx)?;if before==cx.retained_progress()&&fuel==cx.fuel_remaining(){break}}Ok(())});
  }
  self.retire_turn(rig,|cx|{let d=OriginalSurfaceViewRead::release_demand();close_original_retained_step(cx,d,|_|{drop(read.take());RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(())});
 }
 fn close_registry(&mut self,rig:&mut Rig,registry:&mut SurfaceContexts){
  for _ in 0..4096{
   if registry.original_terminal_is_empty(){return}
   self.retire_turn(rig,|cx|{for _ in 0..512{if registry.original_terminal_is_empty()||cx.retirement_should_yield(){break}let before=cx.retained_progress();let fuel=cx.fuel_remaining();registry.close_original_step(cx)?;if before==cx.retained_progress()&&fuel==cx.fuel_remaining(){break}}Ok(())});
  }
  panic!("actual actor registry retirement did not settle")
 }
 fn finish(&mut self,rig:&mut Rig){
  for _ in 0..4096{
   if self.slot.terminal_is_empty(){return}
   rig.turn(|host|host.with_original_retirement(|host|{let input=self.input(host)?;let call=self.slot.close_step(input,StepBudget::new(host.fuel_remaining().min(512),host.deadline_us(),host.retained_grant()));Self::post(host,call.receipt_spent,call.work_units_spent)?;call.result.expect("actual actor context closes its original source and decoder");Ok(())}));
  }
  panic!("actual actor context retirement did not settle")
 }
}
impl Drop for ActorEpoch{fn drop(&mut self){assert!(self.slot.terminal_is_empty(),"actual actor epoch law requires both physical contexts closed")}}

#[test]
fn original_surface_read_resumes_across_genuine_actor_epochs_and_refuses_another_physical_actor(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🪟️surfaces/🧪️tests/read-pin/cases.json")).unwrap();
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️command/📥️custody/🛂️manifest/metadata.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for case in corpus.as_array().unwrap().iter().filter(|case|case.get("actorEpochs").is_some()){
  let selected_surface=case["selected"].as_bool().unwrap().then_some("original-actor-selected-view");
  let((mut registry,original),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();let registry=match selected_surface{Some(surface)=>SurfaceContexts::original_surface_read_fixture(surface,&view),None=>SurfaceContexts::original_read_fixture(&view)};(registry,view)});
  assert!(!seed.overflowed);
  let source_pointers=pointers(selected(&registry,selected_surface).unwrap());let source_address=selected(&registry,selected_surface).unwrap()as*const ViewModel as usize;
  let mut rig=Rig::new(&policy,seed.requested_bytes-seed.released_bytes);
  let mut actor=ActorEpoch::new();actor.admit(&mut rig);let mut read=None;let mut read_admission=Default::default();
  actor.turn(&mut rig,|cx|OriginalSurfaceViewRead::admit(cx,&mut read_admission,&mut read));
  for _ in 0..4096{if read.as_ref().unwrap().ready(){break}actor.turn(&mut rig,|cx|match selected_surface{Some(surface)=>read.as_mut().unwrap().advance_surface(&mut registry,surface,cx),None=>read.as_mut().unwrap().advance(&mut registry,cx)});}
  let job_aliases_while_pinned=actor.slot.original_loans_are_returned();
  let mut first=None;let mut last_epoch=0;let mut matched=true;let mut borrowed=0;
  for _ in 0..case["actorEpochs"].as_u64().unwrap(){
   let current=actor.turn(&mut rig,|cx|{let source=read.as_ref().unwrap().borrow(&registry,cx)?.unwrap();let view=source.get();Ok((OriginalSourceBinding::borrow(cx),cx.native_receiving_identity(),cx.original_source_parent()?.source_identity()?,view as*const ViewModel as usize,pointers(view),cx.original_clock_binding(),cx.original_absolute_deadline_us()))});
   matched&=current.4==source_pointers&&actor.epoch>last_epoch&&current.1.is_some()&&current.6==policy["deadlineUs"].as_u64().unwrap();
   if let Some(expected)=first{matched&=current==expected}else{first=Some(current)}
   last_epoch=actor.epoch;borrowed+=1;
  }
  let first=first.unwrap();let mut foreign=ActorEpoch::new();foreign.admit(&mut rig);let mut foreign_read=None;let mut foreign_read_admission=Default::default();
  foreign.turn(&mut rig,|cx|OriginalSurfaceViewRead::admit(cx,&mut foreign_read_admission,&mut foreign_read));
  let(failure,other)=foreign.turn(&mut rig,|cx|Ok((read.as_ref().unwrap().borrow(&registry,cx).err(),(OriginalSourceBinding::borrow(cx),cx.native_receiving_identity(),cx.original_source_parent()?.source_identity()?,cx.original_clock_binding()))));
  let refused=failure.is_some();
  matched&=other.0.operation()==first.0.operation()&&other.0.generation()==first.0.generation()&&other.3==first.5&&other.0!=first.0&&other.2!=first.2&&other.1.is_some()&&other.1!=first.1;
  if let Some(failure)=failure{foreign.close_value(&mut rig,failure)}
  foreign.close_read(&mut rig,&mut foreign_read,&mut registry);foreign.finish(&mut rig);
  let resumed=actor.turn(&mut rig,|cx|{let source=read.as_ref().unwrap().borrow(&registry,cx)?.unwrap();Ok((OriginalSourceBinding::borrow(cx),cx.native_receiving_identity(),cx.original_source_parent()?.source_identity()?,source.get()as*const ViewModel as usize,pointers(source.get())))});
  matched&=resumed.0==first.0&&resumed.1==first.1&&resumed.2==first.2&&resumed.3==first.3&&resumed.4==first.4;
  actor.close_read(&mut rig,&mut read,&mut registry);
  let restored=pointers(selected(&registry,selected_surface).unwrap())==source_pointers&&selected(&registry,selected_surface).unwrap()as*const ViewModel as usize==source_address;
  actor.close_registry(&mut rig,&mut registry);actor.close_value(&mut rig,original);actor.finish(&mut rig);
  rig.finish();
  assert!(matched,"same genuine caller, decoder, clock and original view survive every issued epoch");
  assert_eq!(borrowed,case["expected"]["borrowed"].as_u64().unwrap());assert_eq!(usize::from(refused),case["expected"]["refused"].as_u64().unwrap()as usize);assert_eq!(restored,case["expected"]["restored"].as_bool().unwrap());
 }
}
#[test]
fn removed_surface_view_retires_within_the_next_funded_turns(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🎞️command/📥️custody/🛂️manifest/metadata.json")).unwrap();
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 let((mut registry,original),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
  let view:ViewModel=serde_json::from_value(fixture["view"].clone()).unwrap();let mut registry=SurfaceContexts::default();
  registry.insert("removed-surface".into(),"body".into(),view.clone()).unwrap();registry.remove("removed-surface");(registry,view)
 });assert!(!seed.overflowed);
 assert!(registry.borrow("removed-surface").is_none());assert!(!registry.original_terminal_is_empty());
 let mut rig=Rig::new(&policy,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());
 while !rig.turn(|cx|registry.retire_displaced_step(cx)){assert!(rig.calls<200,"displaced view retires within the funded turns")}
 assert!(registry.original_terminal_is_empty());
 rig.registry(&mut registry);rig.close(original);rig.finish();
}
