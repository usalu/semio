//! 📤️ Registered native JSON conversion retains original funding through interior cancellation and same-store history.
extern crate semio_os_kernel as store;
use semio_s_artifact_puzzle_2d_text_inverse as puzzle;
use puzzle::{Puzzle2dSnapshot,Puzzle2dMutation};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobReturnOrigin,OriginalJobReturn,OriginalTurnFailure,StepContext,OperationId,Generation,InteractiveStage,JobOutcomeView,USER_VISIBLE_LANE_FUEL,USER_VISIBLE_LANE_WALL_US};
use semio_framework_value::{DslValue,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_os_kernel::io::io_mechanism::{OwnedSerializerRequest,OwnedSerializerJob,io_register,io_begin_owned_serialization};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::Value;
use std::mem::size_of_val;

#[path="../../../🧪️tests/🧱️material/🦀️.rs"]
mod oracle;
type Store=store::ArtifactStore<Puzzle2dSnapshot,Puzzle2dMutation>;
const GRANT:RetainedCloneGrant=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance;

fn clock()->Option<u64>{semio_framework_job::default_now_us()}
fn ready<T>(future:impl std::future::Future<Output=T>)->T{semio_framework_async::poll::resolve_ready(future)}
fn delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{
 RetainedCloneProgress{copied_items:after.copied_items.checked_sub(before.copied_items).unwrap(),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).unwrap(),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).unwrap(),released_bytes:after.released_bytes.checked_sub(before.released_bytes).unwrap()}
}
fn dialect(value:&Value)->ArtifactDialect{
 ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()}
}
fn document(native:&Value)->Puzzle2dSnapshot{
 semio_framework_pack_json::from_json_str(&serde_json::to_string(native).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).expect("authored independent native specimen")
}
fn make_store(snapshot:Puzzle2dSnapshot)->Store{
 let envelope=store::os_store::document_materialize::create_document_envelope::<Puzzle2dSnapshot,Puzzle2dMutation>("puzzle.puzzle2d","owned-json-acceptance",snapshot,None,&store::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
 let mut store=ready(Store::new::<store::os_store::ArtifactSnapshotPublication>(envelope,store::os_spr::ActorId(store::os_spr::LOCAL_ACTOR_ID.into()))).unwrap();
 store.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners().expect("actual bounded store owner catalog"));store
}
struct Runner{caller:OriginalJobFixtureCaller,turns:usize,maximum:usize,epoch:u64,law:Value}
impl Runner{
 fn new(law:Value,generation:u64)->Self{
  let source=&law["operationSource"];let stock=RetainedCloneGrant{maximum_items:source["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:source["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:source["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:source["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:source["maximumDepth"].as_u64().unwrap()as usize};
  let limit=&law["limits"];let request=limit["requestBytes"].as_u64().unwrap()as usize;let result=limit["resultBytes"].as_u64().unwrap()as usize;let native=limit["nativeValueAllowanceBytes"].as_u64().unwrap()as usize;let vm=limit["vmBytes"].as_u64().unwrap()as usize;let scaffold=limit["scaffoldingBytes"].as_u64().unwrap()as usize;
  assert_eq!(native,law["material"]["nodes"].as_u64().unwrap()as usize*4096);assert_eq!(stock.maximum_items,GRANT.maximum_items*law["execution"]["userVisibleTurns"].as_u64().unwrap()as usize);assert_eq!(stock.maximum_copy_bytes,vm+2*(2*request+2*result+2*native));assert_eq!(stock.maximum_capacity_bytes,2*vm+2*(2*request+3*result+2*native)+scaffold);assert_eq!(stock.maximum_release_bytes,stock.maximum_capacity_bytes);assert_eq!(stock.maximum_depth,GRANT.maximum_depth);
  let deadline=clock().unwrap().checked_add(law["execution"]["maximumMilliseconds"].as_u64().unwrap()*1000).unwrap();let fuel=USER_VISIBLE_LANE_FUEL.checked_mul(law["execution"]["userVisibleTurns"].as_u64().unwrap()).unwrap();
  Self{caller:OriginalJobFixtureCaller::admit(OperationId(772),Generation(generation),stock,clock,deadline,fuel),turns:0,maximum:law["execution"]["maximumTurns"].as_u64().unwrap()as usize,epoch:0,law}
 }
 fn count(&mut self){assert!(self.turns<self.maximum,"one finite operation protocol-turn stock");self.turns+=1;self.epoch+=1;}
 fn turn<T>(&mut self,cleanup:bool,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{
  self.count();let epoch=self.epoch;let mut output=None;
  self.caller.turn(GRANT,|cx|{
   cx.bind_original_source_epoch(epoch)?;let deadline=clock().unwrap().checked_add(USER_VISIBLE_LANE_WALL_US).unwrap().min(cx.original_absolute_deadline_us());
   let mut operation=|cx:&mut StepContext<'_>|cx.with_original_work_loan(USER_VISIBLE_LANE_FUEL,deadline,|cx|{
    let before=cx.retained_progress();let(result,heap)=observe(||run(cx));let spent=delta(before,cx.retained_progress());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(spent.retained_capacity_bytes,spent.released_bytes),"actual operation backing equals its original receipt");output=Some(result?);Ok(())
   });
   if cleanup{cx.with_original_retirement(operation)}else{operation(cx)}
  }).expect("actual original bounded native turn");output.unwrap()
 }
 fn retire<T:RetireOwned>(&mut self,value:T){
  let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native test owner has actual paid retirement"));
  while !owner.terminal_is_empty(){self.turn(true,|cx|{
   let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;
   semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;Ok(())
  });}
 }
 fn request(&self,issuer:&Store)->OwnedSerializerRequest{
  OwnedSerializerRequest{source:issuer.snapshot_read().unwrap().into_erased(),route:(dialect(&self.law["route"]["from"]),dialect(&self.law["route"]["into"])),generation:issuer.generation().0,revision:issuer.content_revision_now(),options:DslValue::Null,children:None}
 }
 fn admit(&mut self,request:OwnedSerializerRequest)->Result<Box<dyn OwnedSerializerJob>,String>{
  let mut returned=None;self.turn(false,|cx|{
   let result=io_begin_owned_serialization(request,cx.retained_grant());let progress=match &result{Ok(admission)=>admission.progress,Err(refusal)=>refusal.progress};cx.consume_retained(progress)?;cx.consume_fuel(1);returned=Some(result);Ok(())
  });
  match returned.unwrap(){Ok(admission)=>Ok(admission.job),Err(refusal)=>{
   let message=refusal.error.to_string();let OwnedSerializerRequest{source,route:(from,into),options,children,..}=refusal.request;assert!(children.is_none());self.retire(source);self.retire((from.artifact_kind,from.standard,from.subset,into.artifact_kind,into.standard,into.subset,options));self.retire(refusal.error);Err(message)
  }}
 }
 fn close_store(&mut self,issuer:&mut Store){
  while !issuer.close_owned_terminal_is_empty(){self.turn(true,|cx|{
   let demand=issuer.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;
   if !cx.retained_work_can_enter(demand){return Ok(())}
   let grant=cx.retained_grant();let result=match cx.with_original_effect(grant,|_,effect|issuer.close_owned_step(effect))?{Some((semio_framework_value::RetainedWorkPoll::Complete(turn),_))=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),_=>Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))};let spent=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_retained(spent)?;cx.consume_fuel(1);result?;Ok(())
  });}
 }
 fn finish(mut self){
  while !self.caller.terminal_is_empty(){self.count();self.caller.retire_step(GRANT).expect("same finite original source closes");}
 }
}
#[derive(serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
struct Progress{phase:Phase,completed_bytes:usize,total_bytes:Option<usize>}
#[derive(serde::Deserialize,PartialEq,Eq)]
#[serde(rename_all="camelCase")]
enum Phase{Measure,Allocate,Encode,SourceClose,Handoff,Complete,Closing}
fn progress(bytes:&[u8])->Progress{
 let value:Value=serde_json::from_slice(bytes).expect("independent progress JSON");
 if let Some(total)=value.get("totalBytes"){assert!(total.as_u64().is_some(),"present totalBytes obeys its closed integer schema");}
 serde_json::from_value(value).expect("closed typed progress payload")
}
struct Outcome{bytes:[u8;512],length:usize,complete:bool,cancelled:bool}
fn step(runner:&mut Runner,job:&mut dyn OwnedSerializerJob,origin:&mut OriginalJobReturnOrigin)->Outcome{
 let mut observation=Outcome{bytes:[0;512],length:0,complete:false,cancelled:false};
 runner.turn(false,|cx|{
  if let Some(descriptor)=origin.outcome(){
   if !descriptor.is_acknowledged(){match job.borrow_outcome(descriptor)?{
     JobOutcomeView::PreviewReady{payload,..}=>{
      assert!(payload.len()<=observation.bytes.len(),"closed native progress envelope");let demand=RetirementDemand{copy_bytes:payload.len(),depth:1,..Default::default()};if !cx.retained_work_can_enter(demand){return Ok(())}let mut offset=0;
      for index in 0..payload.page_count(){let page=payload.page(index).unwrap();observation.bytes[offset..offset+page.len()].copy_from_slice(page);offset+=page.len();}
      observation.length=offset;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:offset,..Default::default()})?;cx.consume_fuel(1);
     },
     JobOutcomeView::Complete{..}=>observation.complete=true,
     JobOutcomeView::Cancelled{..}=>observation.cancelled=true,
     JobOutcomeView::Fault{..}=>panic!("ordinary native fault cannot satisfy cancellation or completion"),
     JobOutcomeView::CheckpointReady{..}=>panic!("format progress is a real preview, not fabricated checkpoint state"),
     JobOutcomeView::Yield{..}=>{}
    }}

   origin.acknowledge_outcome(cx)?;return Ok(())
  }
  if !origin.admit(cx)?{return Ok(())}
  let returned=semio_framework_job::drive_step(job,cx,origin.receiver()?,"puzzle.owned-json",InteractiveStage::InteractiveStep,&mut None)?;
  match returned{
   OriginalJobReturn::Preparing|OriginalJobReturn::Pending=>{},
   OriginalJobReturn::Continuation(value)=>{let mut value=Some(value);origin.receive_continuation(cx,&mut value)?;assert!(value.is_none());panic!("actual serializer preserves its primary failure in original continuation")},
   OriginalJobReturn::Outcome(reply)=>{
    let mut value=Some(reply.into_descriptor());assert!(origin.receive_outcome(cx,&mut value)?);assert!(value.is_none());
   }
  }Ok(())
 });observation
}
fn close_job(runner:&mut Runner,mut job:Box<dyn OwnedSerializerJob>,origin:&mut OriginalJobReturnOrigin,cancelled:bool){
 if cancelled{assert!(job.output().is_none());let mut refused=None;runner.turn(true,|cx|{match job.take_output(cx.retained_grant()){Err(error)=>refused=Some(error),Ok(_)=>panic!("canceled native export must reject output handoff")}Ok(())});runner.retire(refused.unwrap());}
 job.begin_close();let mut observed_cancel=false;
 while !job.terminal_is_empty(){
  runner.turn(true,|cx|{job.close_step(cx)?;Ok(())});
  if let Some(fault)=job.fault(){observed_cancel|=fault.code.0=="canceled"&&fault.origin==semio_framework_diagnostic::FaultOrigin::Framework&&fault.severity==semio_framework_diagnostic::Severity::Error&&!fault.retryable;}
  if cancelled{assert!(job.output().is_none());}
 }
 while !origin.terminal_is_empty(){runner.turn(true,|cx|{origin.close_step(cx)?;Ok(())});}
 let bytes=job.terminal_frame_release_bytes().expect("actual completed frame release");assert_eq!(bytes,size_of_val(job.as_ref()));
 runner.turn(true,|cx|{
  semio_framework_job::close_original_retained_step(cx,RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},|_|{drop(job);RetainedWorkOutcome{result:Ok::<_,semio_framework_value::ValueError>(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})),work:1}})?;Ok(())
 });
 if cancelled{assert!(observed_cancel,"real cancellation must preserve its exact typed canceled fault during paid cleanup");}
}
fn exported(runner:&mut Runner,issuer:&Store,expected:&Value,require_interior:bool)->Result<(),String>{
 let mut job=runner.admit(runner.request(issuer))?;let mut origin=OriginalJobReturnOrigin::empty();let mut completed=false;let mut saw_encode=false;
 while !completed{
  let observation=step(runner,job.as_mut(),&mut origin);assert!(!observation.cancelled);
  if observation.length!=0{let progress:Progress=progress(&observation.bytes[..observation.length]);if progress.phase==Phase::Encode{let total=progress.total_bytes.expect("measured total retained before encoding");assert!(progress.completed_bytes<=total);saw_encode|=progress.completed_bytes>0&&progress.completed_bytes<total;}}
  completed=observation.complete;
 }
 assert!(!require_interior||saw_encode,"actual large encoding exposes positive incomplete conversion");
 let mut handoff=None;runner.turn(false,|cx|{let value=job.take_output(cx.retained_grant())?;if let Some(value)=value{cx.consume_retained(value.progress)?;cx.consume_fuel(1);handoff=Some(value.output);}Ok(())});
 let output=handoff.expect("actual completed paged text output");assert!(output.contiguous.is_empty(),"owned conversion must not flatten its whole file");
 let bytes=(0..output.page_count()).flat_map(|index|{let page=output.page(index).unwrap();assert!(page.len()<=runner.law["limits"]["pageBytes"].as_u64().unwrap()as usize);page.iter().copied()}).collect::<Vec<_>>();
 assert!(bytes.len()<=runner.law["limits"]["resultBytes"].as_u64().unwrap()as usize);let actual:Value=serde_json::from_slice(&bytes).expect("independent third-party complete JSON output");assert_eq!(&actual,expected);
 let read=issuer.snapshot_read().unwrap();let leaf=puzzle::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(read.get()).unwrap();assert_eq!(bytes,leaf,"owned file preserves exact normative pretty bytes");runner.retire(read.into_erased());
 close_job(runner,job,&mut origin,false);runner.retire(output);Ok(())
}
fn node_count(runner:&mut Runner,issuer:&Store)->usize{let read=issuer.snapshot_read().unwrap();let count=read.get().nodes.len();runner.retire(read.into_erased());count}
#[semio_framework_async_macros::async_test]
async fn registered_owned_json_converts_cancels_and_keeps_the_same_store_live(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let schema:Value=serde_json::from_str(include_str!("🧫️fixtures/📐️.json")).unwrap();assert_eq!(schema["additionalProperties"],false);for(key,value)in law.as_object().unwrap(){assert_eq!(&schema["properties"][key]["const"],value);}
 io_register(puzzle::standards::v1::subsets::any::io::routes::entries()).unwrap();
 let(native,expected)=oracle::material(&law["material"]);let mut issuer=make_store(document(&native));let mut runner=Runner::new(law,issuer.generation().0);
 if let Err(error)=exported(&mut runner,&issuer,&expected,true){runner.close_store(&mut issuer);runner.finish();panic!("canonical registered JSON owned factory required: {error}");}
 let seed=oracle::seed();
 for word in std::iter::once(None).chain(seed["binary64Words"].as_array().unwrap().iter().map(|word|Some(u64::from_str_radix(word.as_str().unwrap(),16).unwrap()))){
  let expected=oracle::declared(seed["snapshot"].clone(),word,false);
  let snapshot=puzzle::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&serde_json::to_vec(&expected).unwrap()).expect("actual declared import prepares authored full word specimen");
  let mut specimen=make_store(snapshot);
  if let Err(error)=exported(&mut runner,&specimen,&expected,false){runner.close_store(&mut specimen);runner.close_store(&mut issuer);runner.finish();panic!("registered complete word export refused: {error}");}
  runner.close_store(&mut specimen);
 }
 let mut job=runner.admit(runner.request(&issuer)).unwrap_or_else(|error|panic!("{error}"));let mut origin=OriginalJobReturnOrigin::empty();let mut interior=false;
 while !interior{let observation=step(&mut runner,job.as_mut(),&mut origin);assert!(!observation.complete&&!observation.cancelled,"cancel must occur inside actual conversion");if observation.length!=0{let progress:Progress=progress(&observation.bytes[..observation.length]);interior=progress.phase==Phase::Encode&&progress.completed_bytes>0&&progress.completed_bytes<progress.total_bytes.expect("actual encode total");}}
 runner.caller.cancel().cancel_now();close_job(&mut runner,job,&mut origin,true);
 let before=node_count(&mut runner,&issuer);let mut node=native["nodes"][0].clone();node["id"]=Value::String("AfterExportCancel".into());
 let node=semio_framework_pack_json::from_json_str::<puzzle::standards::v1::subsets::any::schema::records::Puzzle2dNode>(&serde_json::to_string(&node).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let mutation:Puzzle2dMutation=puzzle::standards::v1::subsets::any::schema::mutations::create_node::create_node(node,None);
 let receipt=semio_plugin::with_authoring_identity!(|identity|issuer.dispatch(store::os_store::store_schema::ArtifactCommand::Apply{mutations:vec![mutation],transaction:None},&mut identity).await).unwrap();runner.retire((receipt.edit_ids,receipt.messages));assert_eq!(node_count(&mut runner,&issuer),before+1);
 let receipt=semio_plugin::with_authoring_identity!(|identity|issuer.dispatch(store::os_store::store_schema::ArtifactCommand::Undo,&mut identity).await).unwrap();runner.retire((receipt.edit_ids,receipt.messages));assert_eq!(node_count(&mut runner,&issuer),before);
 let receipt=semio_plugin::with_authoring_identity!(|identity|issuer.dispatch(store::os_store::store_schema::ArtifactCommand::Redo,&mut identity).await).unwrap();runner.retire((receipt.edit_ids,receipt.messages));assert_eq!(node_count(&mut runner,&issuer),before+1);
 runner.close_store(&mut issuer);runner.finish();
}
