//! 🧪️ Every original Flow error field survives its paid domain frame and cancellation without formatting.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,StepContext,OriginalTurnFailure,OriginalDomainReservation,admit_original_domain_failure,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};
#[global_allocator]static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn make(row:&serde_json::Value)->FlowCoreError{
 let text=|key:&str|row[key].as_str().unwrap().to_owned();
 match row["variant"].as_str().unwrap(){"Json"=>FlowCoreError::Json(text("value")),"WidgetIdExists"=>FlowCoreError::WidgetIdExists(text("value")),"UnknownWidget"=>FlowCoreError::UnknownWidget(text("value")),"UnknownNeuronWidget"=>FlowCoreError::UnknownNeuronWidget(text("value")),"NotVariadicInput"=>FlowCoreError::NotVariadicInput(text("value")),"NotVariadicOutput"=>FlowCoreError::NotVariadicOutput(text("value")),"NotNeuron"=>FlowCoreError::NotNeuron(text("value")),"WidgetNotNeuron"=>FlowCoreError::WidgetNotNeuron(text("value")),"MaxInputPortsReached"=>FlowCoreError::MaxInputPortsReached(text("value")),"MaxOutputPortsReached"=>FlowCoreError::MaxOutputPortsReached(text("value")),"UnknownInputPort"=>FlowCoreError::UnknownInputPort(text("value")),"UnknownOutputPort"=>FlowCoreError::UnknownOutputPort(text("value")),"NoOutputPort"=>FlowCoreError::NoOutputPort(text("value")),"NoInputPort"=>FlowCoreError::NoInputPort(text("value")),"UnknownSynapse"=>FlowCoreError::UnknownSynapse(text("value")),"UnknownWidgetLayout"=>FlowCoreError::UnknownWidgetLayout(text("value")),"UnknownCluster"=>FlowCoreError::UnknownCluster(text("value")),"WidgetNotCluster"=>FlowCoreError::WidgetNotCluster(text("value")),"UnknownSharedPayload"=>FlowCoreError::UnknownSharedPayload(text("value")),"HistoryRefused"=>FlowCoreError::HistoryRefused(text("value")),"SelfConnection"=>FlowCoreError::SelfConnection,"SelfInsertion"=>FlowCoreError::SelfInsertion,"CycleWouldBeCreated"=>FlowCoreError::CycleWouldBeCreated,"ConnectionAlreadyExists"=>FlowCoreError::ConnectionAlreadyExists,"CollapseNeedsTwoWidgets"=>FlowCoreError::CollapseNeedsTwoWidgets,"CollapseUnknownWidgets"=>FlowCoreError::CollapseUnknownWidgets,"CollapseContainsClusters"=>FlowCoreError::CollapseContainsClusters,
 "MinInputPorts"=>FlowCoreError::MinInputPorts{widget:text("widget"),min:usize::try_from(row["min"].as_u64().unwrap()).unwrap()},
 "MinOutputPorts"=>FlowCoreError::MinOutputPorts{widget:text("widget"),min:usize::try_from(row["min"].as_u64().unwrap()).unwrap()},
 "IncompatiblePortTypes"=>FlowCoreError::IncompatiblePortTypes{source:text("source"),source_type:text("sourceType"),target:text("target"),target_type:text("targetType")},
 "Dag"=>FlowCoreError::Dag(match row["dagVariant"].as_str().unwrap(){"SnapshotRootNotObject"=>DagError::SnapshotRootNotObject,"SchemaMismatch"=>DagError::SchemaMismatch,"NodesMissing"=>DagError::NodesMissing,"UnknownAlignMode"=>DagError::UnknownAlignMode(text("value")),"UnknownWidget"=>DagError::UnknownWidget(text("value")),"CanvasTheme"=>DagError::CanvasTheme(text("value")),"GridFactorOutOfRange"=>DagError::GridFactorOutOfRange,"Json"=>DagError::Json(text("value")),_=>panic!("declared DAG fixture variant")}),_=>panic!("declared Flow fixture variant")
 }
}
fn json(source:&dyn OriginalDomainProjection,path:&mut Vec<usize>)->serde_json::Value{match source.projection_view(path).unwrap(){
 View::IntrinsicObject(length)=>{let mut object=serde_json::Map::new();for index in 0..length{let key=source.projection_key(path,index).unwrap().to_owned();path.push(index);let value=json(source,path);path.pop();assert!(object.insert(key,value).is_none())}serde_json::Value::Object(object)},
 View::IntrinsicText(value)=>serde_json::Value::String(value.to_owned()),
 View::IntrinsicNumber(Number::UInt(value))=>serde_json::json!(value),
 _=>panic!("Flow error projection uses its declared object/text/unsigned scalar vocabulary")
}}
fn close_cold<T:RetireOwned>(source:T){let mut owner=ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original Flow test owner retirement"));for _ in 0..10000{if owner.terminal_is_empty(){return}let d=owner.selected_demand(64).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}panic!("original Flow test owner close")}
#[test]
fn native_flow_core_error_projection_preserves_all_38_declared_payloads(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(corpus["cases"].as_array().unwrap().len(),38);
 for row in corpus["cases"].as_array().unwrap(){let source=make(row);assert_eq!(json(&source,&mut Vec::new()),row["expected"]);assert_eq!(source.domain_code(),row["code"].as_str().unwrap());assert_eq!(source.domain_retained_progress(),Default::default());
  let((view,code,message),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(source.projection_view(&[]).unwrap(),source.domain_code(),source.domain_message()));assert!(matches!(view,View::IntrinsicObject(_)));assert!(!code.is_empty());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(text)=source.original_text(){assert_eq!(message.as_ptr(),text.as_ptr())}
  for path in [&[usize::MAX][..],&[1,0,0][..]]{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.projection_view(path));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}close_cold(source);
 }
}
#[test]
fn native_flow_core_error_keeps_the_complete_native_usize(){
 let source=FlowCoreError::MinOutputPorts{widget:"maximum native minimum".into(),min:usize::MAX};let value=source.projection_view(&[2]).unwrap();let View::IntrinsicNumber(Number::UInt(value))=value else{panic!("unsigned minimum")};assert_eq!(serde_json::json!(value),serde_json::json!(u64::try_from(usize::MAX).unwrap()));close_cold(source);
}
fn clock()->Option<u64>{Some(1000)}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:b.copied_items-a.copied_items,copied_bytes:b.copied_bytes-a.copied_bytes,retained_capacity_bytes:b.retained_capacity_bytes-a.retained_capacity_bytes,released_bytes:b.released_bytes-a.released_bytes}}
struct Family{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner,loan:RetainedCloneGrant,calls:usize,born:usize,released:usize}
impl Family{
 fn turn<R>(&mut self,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{self.calls+=1;assert!(self.calls<=200);let mut actual=(0,0);let result=self.caller.turn_with_decoder(self.loan,&mut self.decoder,|cx|{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(cx));assert!(!heap.overflowed);let p=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));actual=(heap.requested_bytes,heap.released_bytes);result}).unwrap();self.born+=actual.0;self.released+=actual.1;result}
 fn close<T:RetireOwned>(&mut self,value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original domain family close"));while !owner.terminal_is_empty(){self.turn(|cx|cx.with_original_retirement(|cx|{for _ in 0..512{if owner.terminal_is_empty()||cx.should_yield(){break}let d=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|{let result=owner.step(g);semio_framework_value::RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;if step.progress()==Default::default(){break}}Ok(())}));}}
}
#[test]
fn original_flow_core_failures_keep_one_caller_stock_and_fuel_through_publication_and_cancel(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 for row in corpus["cases"].as_array().unwrap(){for items in [1,512]{for cut in [None,Some(0usize),Some(1),Some(2),Some(3)]{
  let(error,seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||make(row));assert!(!seed.overflowed);assert_eq!(seed.released_bytes,0);
  let caller=OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let roots=caller.born_bytes();let mut family=Family{caller,decoder:OriginalJobDecodeOwner::empty(),loan:RetainedCloneGrant{maximum_items:items,..stock},calls:0,born:0,released:0};
  let mut reservation=OriginalDomainReservation::<FlowCoreError>::new();let mut input=Some(error);let mut output=None;let mut stage=0;
  loop{
   if cut==Some(stage){let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||family.caller.cancel().cancel_now());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));break}
   match stage{
    0=>{if family.turn(|cx|reservation.prepare(cx)){stage=1}},
    1=>{let ready=family.turn(|cx|reservation.with_body(cx,|cx,slot|{let d=RetirementDemand{copy_bytes:2*std::mem::size_of::<FlowCoreError>(),depth:1,..Default::default()};if !cx.retained_work_can_enter(d){return Ok(false)}*slot=input.take();cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;Ok(true)}));if ready==Some(true){assert_eq!(json(reservation.pending_projection().unwrap(),&mut Vec::new()),row["expected"]);stage=2}},
    2=>{output=family.turn(|cx|admit_original_domain_failure(&mut reservation,cx));if output.is_some(){stage=3}},
    _=>break
   }
  }
  if cut.is_none(){assert!(output.is_some());let source=output.as_ref().unwrap();let semio_framework_job::OriginalTurnPrimary::Domain(domain)=&source.primary else{panic!("original typed domain primary")};assert_eq!(json(domain.projection(),&mut Vec::new()),row["expected"]);assert_eq!(domain.code(),row["code"].as_str().unwrap());}
  family.close((reservation,input,output));
  while !family.decoder.terminal_is_empty(){family.calls+=1;assert!(family.calls<=200);let mut actual=(0,0);family.caller.turn(family.loan,|cx|cx.with_original_retirement(|cx|{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let d=family.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|family.decoder.close_step(g))?;Ok(())});let p=delta(before,cx.retained_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));actual=(heap.requested_bytes,heap.released_bytes);result})).unwrap();family.born+=actual.0;family.released+=actual.1;}
  assert_eq!(family.caller.stock(),stock);assert!(family.caller.cumulative_receipt().fits(stock));let mut roots_released=0;while !family.caller.terminal_is_empty(){family.calls+=1;assert!(family.calls<=200);family.caller.retire_step(family.loan).unwrap();roots_released+=family.caller.receipt().released_bytes;}assert_eq!(roots_released,roots);assert_eq!(seed.requested_bytes+roots+family.born,family.released+roots_released);
 }}}
}
