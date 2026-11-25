//! 🧸️ Native installed Toy producer advances its real transaction and borrowed pages before granted closure.
#[cfg(test)]
use semio_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use semio_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use pack::snapshot::ArtifactPack;
use store::{ArtifactStoreInitializationAuthority, Mutation, MutationDiff};
use semio_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use store::os_store::backbone::Backbone;
use semio_ui_contract::{Buildable, HasBase, HasChildren, *};
use semio_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework::{FaultFrom, *};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_plugin_window_kits::{LabelAxes, WindowKit};
use protocol::{OpBinary, OpText};
use semio_plugin_tests_06::tool_run_support_1::original_toy_emission::OriginalToyEmission;
use semio_plugin_tests_06::tool_run_support_1::original_toy_unit::OriginalToyUnit;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_plugin_program::PluginProgram;
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunTick, ToolRunTickWriter};
use semio_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_plugin_tests_06::tool_run_support_1::{ToyRunJob, ToyRunJobOriginal, toy_fixture_caller_policy};
use semio_plugin_window_config::WindowConfigOwner;
use semio_plugin_window_transient::window_transient::WindowTransientOwner;
#[test]
fn original_installed_toy_job_preserves_transaction_wire_borrowed_outcomes_and_physical_receipts(){
 use semio_framework_job::{InteractiveJob, JobOutcomeView, OperationId, Generation, OriginalJobFixtureCaller, OriginalJobOutcome, OriginalJobReturn, OriginalJobReturnOrigin, OriginalTurnFailure};
 macro_rules! observe{($body:expr)=>{semio_framework_trace::observe_heap_allocations_on_this_thread($body)}}
 let delta=|before:semio_framework_value::RetainedCloneProgress,after:semio_framework_value::RetainedCloneProgress|semio_framework_value::RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};
 let grant=toy_fixture_caller_policy().0.preparation;let bank=RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(98117),Generation(11),bank,semio_framework_job::default_now_us,u64::MAX,1<<20);
 let mut origin=OriginalJobReturnOrigin::empty();let(mut born,mut released)=(0,0);let mut turns=0;
 loop{let(admitted,heap)=observe!(||caller.turn(grant,|cx|origin.admit(cx)).unwrap());born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<10000);if admitted{break}}
 let(mut job,heap)=observe!(||ToyRunJob{unit:OriginalToyUnit::new(),emission:OriginalToyEmission::new(),phase:0,batch_units:0,scan:0,retraction:None,progress_source:None,counter_source:None,trace_source:None,payload_source:[0;4],payload_source_ready:false,payload_position:0,successor:None,writer_retirement:None,progress_retirement:None,purpose:ToolRunJobPurpose::Run,original:std::mem::ManuallyDrop::new(ToyRunJobOriginal{writer:ToolRunTickWriter::new(ToolRunIdentity::new(ToolRunId{app_instance_id:7,run:11},[17;32])),provisional_counts:Vec::new()}),original_live:true,original_close:None,base_count:0,target:3,done:0,resumed_from:0,checkpoint_due:false,finished:false,closing:false});
 assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let owner=std::ptr::addr_of!(job);let(mut previews,mut checkpoints)=(0,0);let mut complete=false;let mut pending:Option<OriginalJobOutcome>=None;
 while !complete{
  if pending.is_none()&&origin.outcome().is_none(){for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let((claimed,receipt),heap)=observe!(||caller.turn(denied,|cx|{let before=cx.retained_progress();let claimed=origin.enter_body(cx)?.is_some();Ok((claimed,delta(before,cx.retained_progress())))}).unwrap());assert!(!claimed);assert_eq!(receipt,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(std::ptr::addr_of!(job),owner);}}
  let((returned,receipt),heap)=observe!(||caller.turn(grant,|cx|{let before=cx.retained_progress();let mut returned=None;if pending.is_some(){origin.receive_outcome(cx,&mut pending)?;}else if origin.outcome().is_none(){if let Some(mut claim)=origin.enter_body(cx)?{let step=job.step(cx,&mut claim);if let OriginalJobReturn::Outcome(outcome)=claim.finish(cx,step,None){returned=Some(outcome.into_descriptor())}}}Ok((returned,delta(before,cx.retained_progress())))}).unwrap());
  assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<10000);
  if returned.is_some(){pending=returned}
  if pending.is_none()&&origin.outcome().is_some(){
   let(view,heap)=observe!(||job.borrow_outcome(origin.outcome().unwrap()).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   match view{JobOutcomeView::PreviewReady{payload,..}=>{let bytes:Vec<u8>=(0..payload.page_count()).flat_map(|page|payload.page(page).unwrap().iter().copied()).collect();let tick=ToolRunTick::decode(&bytes).unwrap();assert_eq!(tick.append_ops.len(),6);assert_eq!(tick.append_entities,[1,2,3]);previews+=1;},JobOutcomeView::CheckpointReady{state,applied_progress,..}=>{assert_eq!(applied_progress,3);assert_eq!(state.page(0).unwrap(),3u32.to_le_bytes());checkpoints+=1;},JobOutcomeView::Complete{state:None,output:None,..}=>complete=true,_=>panic!("unexpected original Toy descriptor")}
   loop{let(acknowledged,heap)=observe!(||caller.turn(grant,|cx|origin.acknowledge_outcome(cx)).unwrap());born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<10000);if acknowledged{break}}
  }
 }
 assert_eq!((previews,checkpoints),(1,1));assert_eq!(job.done,3);job.begin_close();
 while !job.terminal_is_empty(){let(step,heap)=observe!(||caller.turn(grant,|cx|{let before=cx.retained_progress();let step=job.close_step(cx)?;assert_eq!(delta(before,cx.retained_progress()),step.progress());Ok(step)}).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<20000)}
 while !origin.terminal_is_empty(){let(step,heap)=observe!(||caller.turn(grant,|cx|origin.close_step(cx)).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<30000)}
 assert_eq!(born,released);let((),heap)=observe!(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(bank);
}
