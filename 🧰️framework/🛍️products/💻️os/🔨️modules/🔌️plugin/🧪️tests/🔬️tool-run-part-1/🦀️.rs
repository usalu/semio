//! ⏯️ Part 1 of 2 of the runtime laws of the tool run ledger and driver (cut from `🔬️tool-run`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
use semio_framework_plugin_decl_views::NoDraftMutation;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_job::StepContext;
use semio_framework_plugin_tests_06::tool_run_support_1::{TOY_REQUEST_WINDOWS, ToyApp, ToyFixtureCaller, ToyPortJobCustody, ToyRunApp, ToyRunJob, ToyRunJobOriginal, ToyWorldWindowConfig, advance_toy, close, encoded, fixture, number, pump_until, run_action, set_target, start, text, tool_run_action, toy_app, toy_fixture_caller_policy, toy_manifest, toy_meta, toy_target};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_os_kernel::io;
use semio_framework_plugin_interaction as local_interaction;
use semio_framework_plugin_window_config as window_config;
use semio_framework_ui_scene::scene_lane_hash;
#[cfg(test)]
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture;
use semio_framework::*;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(test)]
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, ToolOperationKey};
use pack::snapshot::ArtifactPack;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactRef;
use store::{ArtifactStore, Mutation, MutationDiff};
use semio_framework_value::DslValue;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value_derive::{FromValue, ToValue};
use protocol::{OpBinary, OpText};
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(test)]
use semio_framework::ViewModel;
#[cfg(test)]
use semio_framework_plugin_retained_command::typed_operation::test_blank_mounted_operation;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework::RequestId;
use semio_framework::ViewWindowInstance;
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceSubject, ToolRunVerdict};
use semio_framework_ui_scene::{Board2dScene, World3dScene};
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
use semio_framework_plugin_tests_06::tool_run_support_1::original_toy_unit::OriginalToyUnit;
use semio_framework_plugin_tests_06::tool_run_support_1::original_toy_emission::OriginalToyEmission;

#[path="../🔬️tool-run/🚦️unit/🧪️tests/🦀️.rs"]
mod original_toy_job_tests;

#[test]
fn original_toy_run_job_closes_same_writer_counts_under_fixed_full_grant(){
    use semio_framework_job::InteractiveJob;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
    let source=toy_fixture_caller_policy().0.preparation;for count in[0,1,8192]{let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(99110),semio_framework_job::Generation(1),source,semio_framework_job::default_now_us,u64::MAX,1);let(job,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let identity=ToolRunIdentity::new(ToolRunId{app_instance_id:7,run:11},[17;32]);let mut writer=ToolRunTickWriter::new(identity);writer.payload(vec![b'x';8192]);ToyRunJob{unit:OriginalToyUnit::new(),emission:OriginalToyEmission::new(),phase:0,batch_units:0,scan:0,retraction:None,progress_source:None,counter_source:None,trace_source:None,payload_source:[0;4],payload_source_ready:false,payload_position:0,successor:None,writer_retirement:None,progress_retirement:None,purpose:ToolRunJobPurpose::Run,original:std::mem::ManuallyDrop::new(ToyRunJobOriginal{writer,provisional_counts:(0..count).collect()}),original_live:true,original_close:None,base_count:0,target:0,done:0,resumed_from:0,checkpoint_due:false,finished:false,closing:false}});let mut job=job;let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let pointer=job.original.provisional_counts.as_ptr();assert_eq!(serde_json::to_value(&job.original.provisional_counts).unwrap(),serde_json::json!((0..count).collect::<Vec<_>>()));job.begin_close();let mut turns=0;
     while !job.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.original_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}for grant in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if job.original_live{assert_eq!(job.original.provisional_counts.as_ptr(),pointer);}}
      let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);
     }assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(source);
    }
}

async fn render_text(app: &mut ToyApp, body_key: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> String {
    let view=ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){if let Some(tree)=app.render(body_key,None,&view,caller.context).await.unwrap_or_else(|fault|panic!("render {body_key}: {fault:?}")){return artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error|panic!("project {body_key}: {error}"))}}
    panic!("render {body_key} did not settle under the original fixture caller")
}

/// 🆔️ A fixture panel id template scoped to run `run` (`{run}` substituted).
fn panel_id(template: &Value, run: u64) -> String {
    text(template).replace("{run}", &run.to_string())
}

#[doc(hidden)]
pub fn find_node<'a>(node: &'a Value, key: &str) -> Option<&'a Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| find_node(child, key))
}

async fn attach_probe(app: &mut ToyApp, channel: &str) -> MemoryBackbone {
    let (backbone, probe) = MemoryBackbone::pair(channel, channel).await;
    app.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone)).await.expect("attach probe backbone");
    probe
}

/// 📮️ Drains what the app sent: `(mutations batches, genesis announcements)`; a member lane's batch is
/// a mutations batch of that document, and acks are not document traffic.
#[doc(hidden)]
pub async fn drain(probe: &mut MemoryBackbone) -> (usize, usize) {
    probe.receive().await.expect("probe receive").into_iter().fold((0, 0), |(mutations, genesis), message| match message {
        BackboneMessage::Mutations { .. } | BackboneMessage::Member { .. } => (mutations + 1, genesis),
        BackboneMessage::Genesis { .. } => (mutations, genesis + 1),
        BackboneMessage::Ack { .. } | BackboneMessage::Retract { .. } => (mutations, genesis),
    })
}

#[semio_framework_async_macros::async_test]
async fn tool_run_ticks_render_the_overlay_while_the_committed_document_stays_untouched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["overlay"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let generation = app.stores.store.generation();
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "overlay run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.stores.store.generation(), generation, "ticks never touch the store generation");
    assert_eq!(app.snapshot().expect("committed snapshot").count, number(&expected["committedCount"]) as i32);
    assert_eq!(app.tool_runs.provisional().len() as u64, number(&expected["provisionalOps"]));
    let entities: Vec<u64> = expected["entities"].as_array().expect("entities").iter().map(number).collect();
    assert_eq!(app.tool_runs.provisional_entities().expect("entities").iter().copied().collect::<Vec<_>>(), entities);
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]));
    let body = render_text(&mut app, "main", &mut caller).await;
    assert!(body.contains(text(&expected["mainBodyText"])), "the renderer reads committed ⊕ provisional: {body}");
    let panel_expected = &fixture["panel"];
    let run = app.tool_runs.slot().expect("slot").run;
    let panel: Value = serde_json::from_str(&render_text(&mut app, text(&panel_expected["bodyKey"]), &mut caller).await).expect("panel projection parses");
    assert_eq!(panel["key"], panel_expected["rootId"]);
    let status = find_node(&panel, &panel_id(&panel_expected["status"]["id"], run)).expect("status line");
    assert_eq!(status["accessibility"]["live"], panel_expected["status"]["live"]);
    let bar = find_node(&panel, &panel_id(&panel_expected["progress"]["id"], run)).expect("progress bar");
    assert_eq!(bar["component"]["type"], panel_expected["progress"]["type"]);
    assert_eq!(bar["component"]["completed"].as_f64(), Some(number(&panel_expected["progress"]["completed"]) as f64));
    assert_eq!(bar["component"]["total"].as_f64(), Some(number(&panel_expected["progress"]["total"]) as f64));
    let finalize = find_node(&panel, &panel_id(&panel_expected["finalize"]["id"], run)).expect("finalize button");
    assert_eq!(finalize["component"]["type"], "button");
    assert_eq!(finalize["accessibility"]["shortcut"], panel_expected["finalize"]["shortcut"]);
    assert!(finalize["bindings"].as_array().is_some_and(|bindings| !bindings.is_empty()), "finalize is a real bound button");
    let presence = app.tool_run_presence().expect("presence summary");
    assert_eq!((presence.completed, presence.total), (number(&expected["units"]), Some(number(&expected["units"]))));
    assert!(app.tool_run_trace_delta(None).is_some_and(|delta| !delta.is_empty() && !delta.contains(['+', '/', '='])), "trace delta is base64url");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted), &mut caller).await;
    assert!(render_text(&mut app, "main", &mut caller).await.contains(text(&expected["committedBodyText"])), "an aborted run renders the committed document again");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_abort_leaves_store_generation_edits_command_log_and_outbox_untouched() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["abort"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-abort").await;
    drain(&mut probe).await;
    app.render_ctx().refresh_cache().await.expect("backfill the command log before the invariant capture");
    let (generation, edits, commands) = (app.stores.store.generation(), app.stores.store.envelope().vcs.edits.len(), app.history.command_log.len());
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "first provisional unit", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeAbort"]) * number(&fixture["opsPerUnit"]), &mut caller).await;
    let output = run_action(&mut app, "toolRunAbort", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"));
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.tool_runs.state().map(ToolRunState::as_str), Some(text(&expected["state"])));
    assert_eq!(app.stores.store.generation(), generation, "abort: store generation");
    assert_eq!(app.stores.store.envelope().vcs.edits.len(), edits, "abort: envelope.vcs.edits length");
    assert_eq!(app.history.command_log.len(), commands, "abort: command log");
    assert_eq!(drain(&mut probe).await, (0, 0), "abort: memory-backbone outbox");
    assert!(app.tool_runs.provisional().is_empty(), "abort retires every provisional op");
    assert_eq!(app.snapshot().expect("committed").count, 0);
    drop(probe);
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a run that has not completed finalizes its PARTIAL result — the run job stops at its tick boundary and the one
/// grouped edit carries exactly the provisional ops computed until the finalize, never an op the job would have computed
/// after it; the run ends `finalized` and the document holds exactly that many units.
#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_while_running_publishes_exactly_the_partial_result_computed_so_far() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["partialFinalize"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    pump_until(&mut app, "the partial result", |app| app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeFinalize"]) * ops_per_unit, &mut caller).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "the run is still computing");
    let computed = app.tool_runs.provisional().len();
    let edits = app.stores.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "partial finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!((app.stores.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), computed, "the edit carries exactly the ops computed until the finalize");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))));
    assert_eq!(app.snapshot().expect("committed").count as u64, computed as u64 / ops_per_unit, "the document holds exactly the computed units");
    assert!(app.tool_runs.provisional().is_empty());
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_publishes_one_grouped_edit_one_mutations_batch_and_undo_removes_all() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["finalize"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-finalize").await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    drain(&mut probe).await;
    let (generation, edits, commands) = (app.stores.store.generation(), app.stores.store.envelope().vcs.edits.len(), app.history.command_log.len());
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    pump_until(&mut app, "finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.stores.store.generation() - generation, number(&expected["storeGenerationAdded"]));
    assert_eq!((app.stores.store.envelope().vcs.edits.len() - edits) as u64, number(&expected["editsAdded"]));
    assert_eq!((app.history.command_log.len() - commands) as u64, number(&expected["commandRowsAdded"]));
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    assert_eq!(edit.forwards.len(), number(&fixture["overlay"]["provisionalOps"]) as usize, "every provisional op lands in the one edit");
    assert!(edit.mutation_meta.iter().all(|meta| meta.group_id.as_deref() == Some(text(&expected["groupId"]))), "the edit is stamped with the run's group id");
    assert_eq!(drain(&mut probe).await, (number(&expected["mutationsMessages"]) as usize, number(&expected["genesisMessages"]) as usize), "one Mutations batch, no genesis re-announcement");
    let committed = app.snapshot().expect("committed after finalize");
    assert_eq!((committed.count, committed.label.as_str()), (number(&expected["countAfterFinalize"]) as i32, text(&expected["labelAfterFinalize"])));
    assert!(app.tool_runs.provisional().is_empty());
    app.stores.store.dispatch(ArtifactCommand::Undo, &mut caller.identity).await.expect("undo the finalized run");
    let undone = app.snapshot().expect("committed after undo");
    assert_eq!((undone.count, undone.label.as_str()), (number(&expected["countAfterUndo"]) as i32, text(&expected["labelAfterUndo"])), "one undo removes the whole run");
    drop(probe);
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a finalized run is ONE tool transaction — every op of its one edit carries the same `TransactionRef`
/// (`tx-<hex16>`, tool `<appId>#<toolId>`), the edit keeps no single-locale description, and its history row is the
/// tool's own declared label in every locale.
#[semio_framework_async_macros::async_test]
async fn tool_run_finalize_is_one_transaction_labelled_by_its_tool_in_every_locale() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(number(&fixture["finalize"]["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    run_action(&mut app, "toolRunFinalize", &mut caller).await;
    pump_until(&mut app, "finalize publishes", |app| app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let edit = app.stores.store.envelope().vcs.edits.last().expect("finalized edit");
    let edit_id = edit.id.clone();
    let transaction = edit.mutation_meta.first().and_then(|meta| meta.transaction.clone()).expect("the finalized edit is a tool transaction");
    assert!(edit.mutation_meta.iter().all(|meta| meta.transaction.as_ref() == Some(&transaction)), "every op of the run carries the one ref");
    assert_eq!(transaction.tool, format!("{}#{}", ToyRunApp::APP_ID, text(&fixture["toolId"])));
    assert!(transaction.id.starts_with("tx-") && transaction.id.len() == "tx-".len() + 16, "a minted transaction id: {}", transaction.id);
    let history = app.render_ref().build_history_view(None).await;
    let row = history.commands.iter().find(|row| row.edit_id.as_deref() == Some(edit_id.as_str())).expect("the run's history row");
    assert_eq!(row.transaction.as_ref(), Some(&transaction));
    assert_eq!((row.label.resolve(Terminology::Native, Locale::En).to_string(), row.label.resolve(Terminology::Native, Locale::De).to_string()), ("Toy fill".to_string(), "Spielfüllung".to_string()));
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a large finalize never piles up returned document roots — after every driver turn the document Store
/// holds at most one returned-but-unreclaimed snapshot read, so the publication's memory is bounded by the staged
/// root and the base its current op reads instead of one whole document per op.
#[semio_framework_async_macros::async_test]
async fn tool_run_large_finalize_reclaims_each_folded_root_before_the_next_op() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let law = &fixture["largeFinalize"];
    let units = number(&law["units"]);
    let maximum = number(&law["maximumReturnedReads"]) as usize;
    let mut app = toy_app(units, &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let edits = app.stores.store.envelope().vcs.edits.len();
    let output = run_action(&mut app, "toolRunFinalize", &mut caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"));
    let (mut turns, mut peak) = (0usize, 0usize);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while !(app.tool_runs.state() == Some(ToolRunState::Finalized) && !app.tool_runs.has_pending_work()) {
        assert!(std::time::Instant::now() < deadline, "the large finalize never settled; state {:?}", app.tool_runs.state());
        advance_toy(&mut app, &mut caller).await.unwrap_or_else(|fault| panic!("large finalize driver turn faulted: {fault:?}"));
        turns += 1;
        peak = peak.max(app.stores.store.returned_snapshot_read_count());
        assert!(peak <= maximum, "turn {turns}: {peak} returned document roots are still held (at most {maximum})");
    }
    assert_eq!(app.stores.store.envelope().vcs.edits.len() - edits, 1, "still exactly one grouped edit");
    assert_eq!(app.stores.store.envelope().vcs.edits.last().expect("finalized edit").forwards.len() as u64, units * number(&fixture["opsPerUnit"]), "every provisional op landed");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_remote_ingest_rebases_and_a_revalidation_conflict_returns_to_complete_with_the_next_generation() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["conflict"];
    let mut app = toy_app(number(&fixture["overlay"]["units"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-conflict").await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId("remote".into()), caller.policy, &mut caller.identity).await;
    let mut remote_probe = attach_probe(&mut remote, "tool-run-conflict-remote").await;
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: number(&expected["remoteCount"]) as i32 }.into()], transaction: None }, &mut caller.identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut caller.identity).await.expect("ingest remote edit");
    assert_eq!(app.stores.store.generation(), generation + 1, "the remote edit is ingested");
    pump_until(&mut app, "rebase refold settles", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"])) && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Complete));
    let store_generation = app.stores.store.generation();
    run_action(&mut app, "toolRunFinalize", &mut caller).await;
    pump_until(&mut app, "revalidation returns", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!(slot.state.as_str(), text(&expected["state"]));
    assert_eq!(u64::from(slot.generation), number(&expected["generationAfterConflict"]));
    assert_eq!(u64::from(app.tool_runs.conflicts()), number(&expected["retracted"]));
    assert_eq!(app.stores.store.generation(), store_generation, "a conflict return never publishes");
    assert!(app.tool_runs.provisional().is_empty(), "conflicting ops are retracted");
    assert!(app.tool_runs.steps().expect("steps").iter().any(|step| step.reason == semio_framework_tool_run::TOOL_RUN_REASON_CONFLICT), "a danger conflict step is shown");
    drop((probe, remote_probe));
    close(&mut app, &mut caller);
    close(&mut remote, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪪️ Law: the runtime mints the document, config, draft and interaction stores each with its canonical operation wire, and each authors a row through it.
/// The fixture is deliberately leaked: its close ladder is the registered-fixture close law, not this one.
#[semio_framework_async_macros::async_test]
async fn the_runtime_installs_its_canonical_operation_wire_on_every_store_it_mints_and_each_authors_through_it() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let mut app = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId(text(&fixture()["actor"]).into()), caller.policy, &mut caller.identity).await;
    assert!(app.stores.store.operation_wire().is_some(), "the document store mints with its semantic wire");
    assert!(app.stores.config_store.operation_wire().is_some(), "the config store mints with its canonical wire");
    assert!(app.stores.draft_store.operation_wire().is_some(), "the draft store mints with its canonical wire");
    assert!(app.stores.interaction_store.operation_wire().is_some(), "the interaction store mints with its canonical wire");
    let config_mutation: <ToyRunApp as ArtifactApp>::ConfigMutation = ChangeTestConfigSelection { selected: Some("7".to_string()) }.into();
    let rows = app.stores.config_store.authored_operation_bytes(std::slice::from_ref(&config_mutation), &mut caller.identity).expect("the config mutation authors through the runtime's wire");
    assert_eq!(rows.len(), 1);
    assert!(rows[0].starts_with(b"op"), "{:?}", String::from_utf8_lossy(&rows[0]));
    let interaction = <semio_framework_plugin_interaction::mutations::InteractionConfigMutation>::set_state(protocol::InteractionState::default());
    let interaction_rows = app.stores.interaction_store.authored_operation_bytes(std::slice::from_ref(&interaction), &mut caller.identity).expect("the interaction mutation authors through the runtime's wire");
    let oracle = format!("op{}", serde_json::json!({ "setInteractionState": { "activeGranularity": {}, "activeMode": {}, "hover": {}, "selection": {} } }));
    assert_eq!(String::from_utf8(interaction_rows[0].clone()).expect("canonical JSON is text"), oracle, "the interaction wire is the independent serde_json document");
    std::mem::forget(app);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_stale_generation_and_run_actions_are_silent_no_ops_and_an_illegal_action_publishes_nothing() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run is running", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))], &mut caller).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    let wrong_run = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(number(&fixture["stale"]["wrongRun"]).to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))], &mut caller).await;
    assert_eq!(wrong_run.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])));
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running), "stale actions change nothing");
    let generation = app.stores.store.generation();
    let illegal = run_action(&mut app, "toolRunResume", &mut caller).await;
    assert_eq!(illegal.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["illegal"]["code"])));
    assert_eq!(app.stores.store.generation(), generation, "an illegal action publishes nothing");
    let busy = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))], &mut caller).await;
    assert_eq!(busy.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["busy"]["code"])));
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY, &mut caller).await).expect("panel parses");
    let finalize = find_node(&panel, &panel_id(&fixture["panel"]["finalize"]["id"], slot.run)).expect("finalize button");
    assert_eq!((finalize["disabled"].as_bool().unwrap_or(false), finalize["accessibility"].get("description")), (false, None), "a running run offers finalize for the partial result it holds");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_pause_then_step_drives_exactly_one_unit_per_step() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["pauseStep"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    assert_eq!(run_action(&mut app, "toolRunPause", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("paused turn");
    }
    let mut provisional = app.tool_runs.provisional().len() as u64;
    for _ in 0..number(&expected["steps"]) {
        assert_eq!(run_action(&mut app, "toolRunStep", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("driveOneUnit"));
        pump_until(&mut app, "single step settles", |app| !app.tool_runs.has_pending_work(), &mut caller).await;
        let after = app.tool_runs.provisional().len() as u64;
        assert_eq!(after - provisional, number(&expected["opsPerStep"]), "one step is exactly one algorithm unit");
        provisional = after;
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("paused turn");
        }
        assert_eq!(app.tool_runs.provisional().len() as u64, provisional, "a paused run schedules nothing");
    }
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Paused));
    assert_eq!(run_action(&mut app, "toolRunPause", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "the shared mod+alt+enter chord binds toolRunPause, which resumes a paused run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resumes_from_its_checkpoint_and_a_lowered_target_retracts() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["reconfigure"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "initial target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit, &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])), "reconfigure keeps the run id and bumps the generation");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the job resumed from its checkpoint");
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["raisedTarget"]))));
    set_target(&mut app, number(&expected["loweredTarget"]), &mut caller).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.snapshot().expect("committed").count, 0, "reconfigure never commits");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⏱️ Bench-style law: a Nakagin-sized fill (771 ticks, two ops each) appends to the overlay in O(k).
/// Machine-load note: tests share the host with concurrent cargo builds, so up to
/// `allowedPreemptedTicks` single ticks may exceed the budget through preemption alone; a real O(n) fold
/// would exceed it on every late tick.
#[semio_framework_async_macros::async_test]
async fn tool_run_overlay_append_per_tick_stays_below_two_milliseconds_for_nakagin_sized_ticks() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["bench"];
    let mut app = toy_app(number(&fixture["defaultTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    let identity = app.tool_runs.identity().expect("identity");
    let label = "x".repeat(number(&expected["labelBytes"]) as usize);
    let ticks = number(&expected["ticks"]);
    let budget = std::time::Duration::from_micros(number(&expected["budgetUs"]));
    let mut over_budget = 0;
    let mut worst = std::time::Duration::ZERO;
    for index in 0..ticks {
        let tick = ToolRunTick {
            identity,
            sequence: index,
            progress: None,
            steps: Vec::new(),
            trace: Vec::new(),
            append_ops: vec![encoded(SetCount { value: index as i32 + 1 }.into()), encoded(SetLabel { value: label.clone() }.into())],
            append_entities: vec![index + 1],
            retract_to: None,
            payload: None,
        };
        let started = std::time::Instant::now();
        let receipt = app.tool_runs.apply_tick(tick).expect("tick applies");
        let elapsed = started.elapsed();
        assert_eq!(receipt.appended, 2);
        worst = worst.max(elapsed);
        if elapsed >= budget {
            over_budget += 1;
        }
    }
    assert!(over_budget <= number(&expected["allowedPreemptedTicks"]), "{over_budget} of {ticks} overlay appends exceeded {budget:?} (worst {worst:?})");
    assert_eq!(app.tool_runs.provisional().len() as u64, ticks * 2);
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={ticks}")));
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

//#region 🔌️IntegrationSeams
/// 🪟️ The per-window view state a renderer sends: both toy windows live, `window_id` targeted with its echoed cursor.
fn window_view(window_id: &str, cursor: Option<ToolRunTraceCursor>) -> ViewModel {
    let lane = &fixture()["traceLane"];
    let (world, other) = (text(&lane["windowId"]).to_string(), text(&lane["otherWindowId"]).to_string());
    ViewModel {
        window_id: Some(window_id.to_string()),
        window_instances: vec![ViewWindowInstance { id: world.clone(), window_kind_id: world }, ViewWindowInstance { id: other.clone(), window_kind_id: other }],
        tool_run_trace_cursor_by_window_id: cursor.map(|cursor| (window_id.to_string(), cursor)).into_iter().collect(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    }
}

/// 🎬️ Renders the `world` body for `cursor` and returns its assembled scene plus the decoded trace delta.
async fn render_world(app: &mut ToyApp, cursor: Option<ToolRunTraceCursor>, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> (World3dScene, Option<ToolRunTraceDelta>) {
    let lane = &fixture()["traceLane"];
    let view=window_view(text(&lane["windowId"]),cursor);let mut tree=None;
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){tree=app.render(text(&lane["bodyKey"]),None,&view,caller.context).await.unwrap_or_else(|fault|panic!("render world: {fault:?}"));if tree.is_some(){break}}
    let tree=tree.expect("world render did not settle under the original fixture caller");
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&lane["surfaceId"])).expect("the world body carries its scene surface");
        artifact_app_laws::built_surface_scene::<World3dScene>(surface).expect("the scene surface assembles")
    });
    let delta = scene.tool_run_trace.as_deref().map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("the lane is base64url")).expect("the lane is one ToolRunTraceDelta"));
    (scene, delta)
}

fn cursor_after(delta: &ToolRunTraceDelta) -> ToolRunTraceCursor {
    ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next }
}

async fn abort_and_close(app: &mut ToyApp, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    run_action(app, "toolRunAbort", caller).await;
    pump_until(app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), caller).await;
    close(app, caller);
}

#[semio_framework_async_macros::async_test]
async fn tool_run_scene_render_carries_the_trace_lane_and_honours_the_echoed_cursor() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    let (idle, idle_delta) = render_world(&mut app, None, &mut caller).await;
    assert!(idle_delta.is_none() && idle.lanes.iter().all(|lane| lane.lane != text(&expected["laneName"])), "no run, no lane");
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let store_pages = app.tool_runs.trace().expect("trace").next_page();
    assert!(store_pages > number(&expected["resumePage"]) as u32, "the run logged several trace pages ({store_pages})");
    let (scene, delta) = render_world(&mut app, None, &mut caller).await;
    let delta = delta.expect("a scene rendered during a run carries the toolRunTrace lane");
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((delta.identity.id.run, delta.identity.generation, delta.clear, delta.next), (slot.run, slot.generation, true, store_pages), "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]), "every tested unit reaches the renderer");
    let reference = scene.lanes.iter().find(|lane| lane.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    let lane_text = scene.tool_run_trace.as_deref().expect("lane text");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane_text.len(), semio_framework_ui_scene::scene_lane_hash(lane_text).as_str()), "the lane ref matches its carrier exactly as split_lanes would emit it");
    let (_, caught_up) = render_world(&mut app, Some(cursor_after(&delta)), &mut caller).await;
    assert!(caught_up.is_none(), "a caught-up cursor costs nothing");
    let resume = number(&expected["resumePage"]) as u32;
    let (_, resumed) = render_world(&mut app, Some(ToolRunTraceCursor { page: resume, ..cursor_after(&delta) }), &mut caller).await;
    let resumed = resumed.expect("a lagging cursor gets the pages after it");
    assert!(!resumed.clear && resumed.pages.first().map(|page| page.page) == Some(resume) && resumed.next == store_pages, "resume continues from the echoed page");
    let (_, rebound) = render_world(&mut app, Some(ToolRunTraceCursor { generation: slot.generation + 1, page: resume, ..cursor_after(&delta) }), &mut caller).await;
    let rebound = rebound.expect("a generation mismatch resends");
    assert!(rebound.clear && rebound.pages.first().map(|page| page.page) == Some(0), "a run/generation mismatch sends clear and resends from page 0");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(run_action(&mut app, "toolRunDismiss", &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none(), "a dismissed run leaves no slot");
    let (_, cleared) = render_world(&mut app, Some(cursor_after(&delta)), &mut caller).await;
    let cleared = cleared.expect("a dismissed run clears a renderer still holding pages");
    assert!(cleared.clear && cleared.pages.is_empty() && cleared.next == 0);
    let (_, empty) = render_world(&mut app, Some(cursor_after(&cleared)), &mut caller).await;
    assert!(empty.is_none(), "a cleared renderer at page 0 gets nothing");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_trace_backlog_keeps_the_scene_dirty_until_the_echoed_cursor_stalls() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "trace run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    while app.take_typed_operation_ui_scope().is_some() {}
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let (window, body) = (text(&expected["windowId"]), text(&expected["bodyKey"]));
    let budget = number(&expected["pageByteBudget"]) as usize;
    let pages = app.tool_runs.trace().expect("trace").next_page();
    let mut cursor = None;
    for delivered in 1..=pages {
        let lane = app.tool_runs.trace_lane(window, body, cursor, budget).expect("one page per refresh under a one-byte budget");
        let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta");
        assert_eq!((delta.pages.len(), delta.next), (1, delivered), "the byte budget still delivers one page per refresh");
        assert_eq!(app.tool_runs.is_ui_dirty(), delivered < pages, "a backlog keeps the window dirty; the last page does not");
        app.tool_run_ctx().flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
        cursor = Some(cursor_after(&delta));
    }
    let lagging = Some(ToolRunTraceCursor { page: 0, ..cursor.expect("cursor") });
    let mut dirty_refreshes = 0;
    for _ in 0..number(&expected["stallRefreshes"]) * 2 {
        app.tool_runs.trace_lane(window, body, lagging, budget).expect("the backlog is resent");
        dirty_refreshes += u64::from(app.tool_runs.is_ui_dirty());
        app.tool_run_ctx().flush_tool_run_ui_dirty();
        while app.take_typed_operation_ui_scope().is_some() {}
    }
    assert_eq!(dirty_refreshes, number(&expected["stallRefreshes"]), "a renderer that never echoes costs a bounded number of refreshes");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_tick_dirty_scope_is_the_panel_plus_the_scene_windows_and_excludes_unrelated_windows() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["dirtyScope"];
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    render_world(&mut app, None, &mut caller).await;
    let other = text(&fixture["traceLane"]["otherWindowId"]);
    let tree = app.render(text(&expected["excludedWindowBody"]), None, &window_view(other, None)).await.expect("render the unrelated window");
    artifact_app_laws::project_and_retire_fixture_tree(tree).expect("project the unrelated window");
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before, &mut caller).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(
        scope,
        UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        "never the whole UI per tick"
    );
    let UiDirtyScope::Partial { window_bodies, .. } = &scope else { unreachable!() };
    assert!(!window_bodies.iter().any(|body| body == text(&expected["excludedWindowBody"])), "a window without a scene surface is not dirtied by a tick");
    let pause = run_action(&mut app, "toolRunPause", &mut caller).await;
    assert_eq!(pause.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_presence_and_the_ephemeral_snapshot_follow_the_run_with_completed_never_above_total() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["presence"];
    let reconfigure = &fixture["reconfigure"];
    let mut app = toy_app(number(&fixture["abort"]["target"]), &mut caller).await;
    assert!(app.tool_run_presence().is_none() && app.ephemeral_snapshot().await.tool_run.is_none(), "no run, no presence summary");
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.progress().is_some_and(|progress| progress.completed > 0), &mut caller).await;
    let running = app.tool_run_presence().expect("running presence");
    assert_eq!((running.tool_id.as_str(), running.state.wire_name()), (text(&fixture["toolId"]), text(&expected["running"])));
    assert!(running.total.is_none_or(|total| running.completed <= total));
    assert_eq!(app.ephemeral_snapshot().await.tool_run, Some(running), "the ephemeral snapshot carries the summary the heartbeat publishes");
    run_action(&mut app, "toolRunAbort", &mut caller).await;
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    assert_eq!(app.tool_run_presence().map(|presence| presence.state.wire_name()), Some(text(&expected["aborted"])));
    close(&mut app, &mut caller);
    let mut app = toy_app(number(&reconfigure["raisedTarget"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "raised run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    set_target(&mut app, number(&reconfigure["loweredTarget"]), &mut caller).await;
    let mut clamped = true;
    pump_until(&mut app, "lowered run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&reconfigure["loweredTarget"]) * number(&fixture["opsPerUnit"]), &mut caller).await;
    for _ in 0..4 {
        let presence = app.tool_run_presence().expect("presence");
        clamped &= presence.total.is_none_or(|total| presence.completed <= total);
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    let presence = app.tool_run_presence().expect("presence");
    assert!(clamped, "completed never exceeds total");
    assert_eq!((presence.state.wire_name(), presence.total), (text(&expected["complete"]), Some(number(&expected["loweredTotal"]))));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

#[semio_framework_async_macros::async_test]
async fn tool_run_provisional_entities_ride_the_instance_records_the_producer_stamps() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["provisional"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let (scene, _) = render_world(&mut app, None, &mut caller).await;
    let instances: Vec<Value> = serde_json::from_str(&scene.instances_json).expect("instances json");
    let flagged: Vec<&str> = instances.iter().filter(|instance| instance["provisional"] == Value::Bool(true)).map(|instance| text(&instance["id"])).collect();
    assert_eq!(flagged, expected["flaggedInstances"].as_array().expect("flagged").iter().map(text).collect::<Vec<_>>(), "ArtifactView::tool_run().provisional_entities drives the provisional instance flag");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
#[semio_framework_async_macros::async_test]
async fn tool_run_job_port_hands_effects_to_the_host_and_a_waiting_job_keeps_nothing_runnable_until_woken() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["port"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    for hop in 1..=number(&expected["hops"]) {
        pump_until(&mut app, "the hop is handed to the host", |app| {
            app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && !app.tool_runs.port().is_some_and(ToolRunJobPort::has_effects) && app.tool_runs.trace().is_some_and(|trace| trace.len() as u64 == hop - 1)
        }, &mut caller)
        .await;
        let mut effects = Vec::new();
        while let Some(effect) = app.take_typed_operation_effect() {
            effects.push(effect);
        }
        assert!(matches!(effects.as_slice(), [Effect::DispatchAction { req: RequestId(req), action, .. }] if *req == hop && action == text(&expected["hopAction"])), "hop {hop}: exactly the job's effect reaches the host outbox: {effects:?}");
        for _ in 0..2 {
            while app.take_typed_operation_ui_scope().is_some() {}
            app.tool_run_ctx().flush_tool_run_ui_dirty();
        }
        assert!(!app.tool_runs.has_pending_work() && !app.tool_run_ref().tool_run_has_pending_work(), "hop {hop}: a waiting job keeps its instance idle");
        advance_toy(&mut app, &mut caller).await.expect("an idle turn");
        assert!(app.take_typed_operation_effect().is_none(), "hop {hop}: an unwoken job is never stepped again");
        app.tool_runs.port().expect("the run's port").wake();
        assert!(app.tool_runs.has_pending_work(), "hop {hop}: the wake makes the run runnable again");
    }
    pump_until(&mut app, "every hop answered", |app| app.tool_runs.state().map(ToolRunState::as_str) == Some(text(&expected["state"])), &mut caller).await;
    assert_eq!(app.tool_runs.trace().expect("trace").len() as u64, number(&expected["traceRecords"]), "one trace record per answered hop");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🔌️IntegrationSeams

//#region 🔢️ActionArgCarriers
/// 🔢️ The wire value a fixture row names, in the carrier it names — the shapes a §2.5 action argument
/// really arrives in: a guest's `DslValue::uint`, the shell's `UiValue::Number` float, and the text a
/// targeted action's `runId` travels as.
fn carrier_value(row: &Value) -> DslValue {
    match text(&row["carrier"]) {
        "uint" => DslValue::uint(row["value"].as_u64().expect("an unsigned fixture carrier")),
        "float" => DslValue::float(row["value"].as_f64().expect("a float fixture carrier")),
        "string" => DslValue::String(text(&row["value"]).to_string()),
        other => panic!("the fixture named an unknown arg carrier {other}"),
    }
}

/// ⚖️ LAW: every `actionArgCarriers` row — a §2.5 action argument that is an exact non-negative integer
/// reads as that integer whichever carrier minted it, and nothing else names an identity at all.
///
/// 🪪️ `generation` crosses as `Number(Float(1.0))` from the shell's own action arguments and from a
/// guest's `DslValue::uint` alike; a reader that took only an exact `u64` carrier or a numeric string
/// answered `None`, and `(_, None)` is `ToolRunRejection::Stale`. Every `toolRunFinalize` of every app
/// was refused, so runs reached `Complete` and stayed there — and because a start against a live run is
/// `ToolRunRejection::Busy`, no tool could run a second time in a session. Measured on 6018 as a 3d
/// preview that never re-evaluated after an inspector edit
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[test]
fn every_action_arg_carrier_of_an_exact_integer_reads_the_same_identity() {
    for row in fixture()["actionArgCarriers"]["rows"].as_array().expect("carrier rows") {
        let id = text(&row["id"]);
        let args = DslValue::Object(vec![("generation".to_string(), carrier_value(row)), ("runId".to_string(), carrier_value(row))]);
        let read = semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&args), "generation");
        let expected = row["reads"].as_u64();
        println!("[STATS] actionArgCarriers {id}: carrier={} read={read:?}", text(&row["carrier"]));
        assert_eq!(read, expected, "{id}: generation");
        assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&args), "runId"), expected, "{id}: runId — one reader, both identity arguments");
    }
    assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(Some(&DslValue::Object(Vec::new())), "generation"), None, "an absent argument names no identity");
    assert_eq!(semio_framework_plugin_tool_run::tool_run_arg_u64(None, "generation"), None, "an action with no arguments at all names no identity");
}

/// ⚖️ LAW: a `toolRunFinalize` whose `generation` arrives in the fixture's declared wire carrier really
/// finalizes the complete run it names — the end-to-end reading of the law above, through the same
/// `handle_action` door the shell dispatches through.
#[semio_framework_async_macros::async_test]
async fn a_finalize_in_the_wire_carrier_finalizes_the_run_it_names() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let carrier = text(&fixture["actionArgCarriers"]["finalizeCarrier"]);
    let mut app = toy_app(number(&fixture["finalize"]["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    let slot = app.tool_runs.slot().expect("a complete run has a slot");
    let row = serde_json::json!({ "carrier": carrier, "value": f64::from(slot.generation) });
    let arguments = vec![("runId".to_string(), DslValue::String(slot.run.to_string())), ("generation".to_string(), carrier_value(&row))];
    let output = tool_run_action(&mut app, "toolRunFinalize", arguments, &mut caller).await;
    println!("[STATS] finalize carrier={carrier} output={output:?}");
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "a finalize naming the run's own generation is never stale, whatever carrier the number arrived in");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("beginFinalize"), "it begins the finalize");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Finalizing), "and the run leaves complete");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🔢️ActionArgCarriers

/// ⚖️ LAW: a READ-ONLY run's completion is its finalization — the driver turn that completes its job
/// leaves `complete` on its own, and the tool can be started again without any finalize action ever
/// being dispatched.
///
/// 🪪️ `complete` means "the job is done and the run awaits the finalize that publishes its provisional
/// edits". A `mutating: false` run authors none, so there is nothing to publish and nothing to review;
/// parking it in `complete` only holds the tool's single run slot, and a `toolRunStart` against a
/// non-terminal run is `toolRun.busy`. Generation3d's read-only `previewEval` therefore ran exactly ONCE
/// per session and its 3d preview stopped re-evaluating after the first evaluation
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
///
/// ▶️ And the slot is free: the next start is admitted, with no finalize action anywhere in this test.
#[semio_framework_async_macros::async_test]
async fn a_read_only_run_finalizes_itself_and_frees_its_slot_for_the_next_start() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readOnlyRun"];
    let tool_id = text(&expected["toolId"]);
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, tool_id, &mut caller).await;
    pump_until(&mut app, "the read-only run leaves complete on its own", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Running && state != ToolRunState::Starting && state != ToolRunState::Complete), &mut caller).await;
    let state = app.tool_runs.state().expect("a state");
    println!("[STATS] readOnlyRun state={} after its job completed, with no finalize action dispatched", state.as_str());
    assert_ne!(state, ToolRunState::Complete, "a read-only run never parks in complete waiting for a finalize nobody owes it");
    pump_until(&mut app, "the read-only run settles", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let output = tool_run_action(&mut app, "toolRunStart", vec![("toolId".into(), DslValue::String(tool_id.into()))], &mut caller).await;
    assert_eq!(output.get("rejected").and_then(DslValue::as_str), None, "the finished read-only run never refuses the next start as busy");
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "and the next start really spawns a job");
    assert_eq!(expected["startsAgainWithoutAnyFinalizeAction"].as_bool(), Some(true));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

//#region 🎯️RetargetAndSettings
/// 📡️ A remote peer sets the count to `count`; `probe` forwards its mutation batch and `app` ingests it.
async fn ingest_remote_count(app: &mut ToyApp, probe: &mut MemoryBackbone, channel: &str, count: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let mut remote = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId("remote".into()), caller.policy, &mut caller.identity).await;
    let mut remote_probe = attach_probe(&mut remote, channel).await;
    assert_eq!(remote.stores.store.local_actor_id(), &protocol::ActorId("remote".into()));
    remote.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value: count as i32 }.into()], transaction: None }, &mut caller.identity).await.expect("remote edit");
    for message in remote_probe.receive().await.expect("remote outbox").into_iter().filter(|message| matches!(message, BackboneMessage::Mutations { .. })) {
        probe.send(message).await.expect("forward remote edit");
    }
    let generation = app.stores.store.generation();
    app.tick_backbone(&mut caller.identity).await.expect("ingest remote edit");
    assert_eq!(app.stores.store.generation(), generation + 1, "the remote edit is ingested");
    drop(remote_probe);
    close(&mut remote, caller);
}

/// ⚖️ LAW: a base change during a run hands every later tick the new identity — a plain job is rebuilt under it
/// from its checkpoint, a retargetable job is rebound in place — so the run keeps appending after the rebase
/// instead of producing ticks the ledger drops as stale.
#[semio_framework_async_macros::async_test]
async fn tool_run_base_change_rebinds_the_running_job_so_ticks_never_carry_a_stale_identity() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["rebind"];
    for (tool_id, rebuilt) in [(text(&fixture["toolId"]), true), (text(&fixture["retarget"]["toolId"]), false)] {
        let mut app = toy_app(number(&expected["target"]), &mut caller).await;
        let mut probe = attach_probe(&mut app, &format!("tool-run-rebind-{tool_id}")).await;
        start(&mut app, tool_id, &mut caller).await;
        pump_until(&mut app, "first checkpointed unit", |app| {
            app.tool_runs.provisional().len() as u64 >= number(&expected["unitsBeforeRebase"]) * number(&fixture["opsPerUnit"]) && app.tool_runs.checkpoint().is_some() && app.tool_runs.state() == Some(ToolRunState::Running)
        }, &mut caller)
        .await;
        ingest_remote_count(&mut app, &mut probe, &format!("tool-run-rebind-remote-{tool_id}"), number(&expected["remoteCount"]), &mut caller).await;
        pump_until(&mut app, "rebase observed", |app| app.tool_runs.slot().is_some_and(|slot| u64::from(slot.generation) == number(&expected["generationAfterRebase"])), &mut caller).await;
        let identity = app.tool_runs.identity().expect("identity");
        let after_rebase = app.tool_runs.provisional().len();
        pump_until(&mut app, "the run appends after the rebase", |app| app.tool_runs.provisional().len() > after_rebase + 2 * number(&fixture["opsPerUnit"]) as usize, &mut caller).await;
        assert_eq!(app.tool_runs.identity(), Some(identity), "{tool_id}: appending never moved the identity again");
        let resumed_from = app.tool_runs.progress().expect("progress").counters.first().map_or(0, |counter| counter.value);
        println!("[STATS] rebind {tool_id}: provisional {after_rebase} -> {} resumedFrom={resumed_from}", app.tool_runs.provisional().len());
        assert_eq!(resumed_from > 0, rebuilt, "{tool_id}: a plain job is rebuilt from its checkpoint, a retargetable job keeps running");
        drop(probe);
        abort_and_close(&mut app, &mut caller).await;
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a §2.5 action that names no identity (a keyboard chord) targets the instance's live run at its
/// generation of the dispatch; an identity argument that is stale still no-ops.
#[semio_framework_async_macros::async_test]
async fn tool_run_chord_actions_without_an_identity_resolve_the_live_run_and_stale_identities_still_no_op() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["chord"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "running", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    let output = app.handle_action("toolRunPause", None, &toy_meta(), &mut caller.identity).await.expect("chord pause").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("stopScheduling"), "a chord without arguments pauses the live run");
    let window_only = vec![("windowId".to_string(), DslValue::String(text(&expected["windowArgument"]).into()))];
    assert_eq!(tool_run_action(&mut app, "toolRunResume", window_only, &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("schedule"), "arguments that name no identity resolve the live run too");
    let slot = app.tool_runs.slot().expect("slot");
    let stale = tool_run_action(&mut app, "toolRunPause", vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(number(&fixture["stale"]["wrongGeneration"]).to_string()))], &mut caller).await;
    assert_eq!(stale.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a stale identity still no-ops");
    let generation_only = tool_run_action(&mut app, "toolRunPause", vec![("generation".into(), DslValue::String(slot.generation.to_string()))], &mut caller).await;
    assert_eq!(generation_only.get("rejected").and_then(DslValue::as_str), Some(text(&fixture["stale"]["code"])), "a partial identity is checked as named, never completed from the live run");
    assert_eq!(app.tool_runs.state(), Some(ToolRunState::Running));
    let output = app.handle_action("toolRunAbort", None, &toy_meta(), &mut caller.identity).await.expect("chord abort").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "a chord abort aborts the live run");
    pump_until(&mut app, "abort settles", |app| app.tool_runs.state() == Some(ToolRunState::Aborted) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let output = app.handle_action("toolRunDismiss", None, &toy_meta(), &mut caller.identity).await.expect("chord dismiss").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("clearTrace"));
    assert!(app.tool_runs.slot().is_none());
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: `settingsChanged` fires only when a value behind a declared settings pointer changes. Republishing the
/// same target and a window-config publication (a camera move) never reconfigure — the run keeps its generation
/// and its job — and a run that declares no settings reads is never reconfigured at all.
#[semio_framework_async_macros::async_test]
async fn tool_run_settings_changed_fires_only_for_the_declared_settings_reads() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["settingsReads"];
    for tool_id in [text(&fixture["toolId"]), text(&expected["undeclaredToolId"])] {
        let declared = tool_id == text(&fixture["toolId"]);
        let mut app = toy_app(number(&expected["target"]), &mut caller).await;
        start(&mut app, tool_id, &mut caller).await;
        pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty(), &mut caller).await;
        let config_generation = app.stores.config_store.generation();
        set_target(&mut app, number(&expected["target"]), &mut caller).await;
        assert!(app.stores.config_store.generation() > config_generation, "{tool_id}: the republished target is a real config publication");
        app.tool_runs.note_window_config_published();
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("turn");
        }
        assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterUnrelatedPublications"]), "{tool_id}: unrelated publications never reconfigure");
        assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(0), "{tool_id}: the job was never rebuilt");
        set_target(&mut app, number(&expected["changedTarget"]), &mut caller).await;
        for _ in 0..8 {
            advance_toy(&mut app, &mut caller).await.expect("turn");
        }
        let generation = u64::from(app.tool_runs.slot().expect("slot").generation);
        println!("[STATS] settingsReads {tool_id}: declared={declared} generation after the changed target={generation}");
        assert_eq!(generation, if declared { number(&expected["generationAfterChangedTarget"]) } else { 0 }, "{tool_id}: only a declared read reconfigures");
        abort_and_close(&mut app, &mut caller).await;
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a retargetable job stays resident while its run is complete and `reconfigure: resume` retargets it in
/// place — a raise continues and a lower retracts without any rebuild — and a base change rebinds it in place.
#[semio_framework_async_macros::async_test]
async fn tool_run_reconfigure_resume_retargets_a_retargetable_job_in_place() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["retarget"];
    let ops_per_unit = number(&fixture["opsPerUnit"]);
    let mut app = toy_app(number(&expected["initialTarget"]), &mut caller).await;
    let mut probe = attach_probe(&mut app, "tool-run-retarget").await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "initial target completes and its resident job is no work", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let run = app.tool_runs.slot().expect("slot").run;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "raised target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit, &mut caller).await;
    let slot = app.tool_runs.slot().expect("slot");
    assert_eq!((slot.run, u64::from(slot.generation)), (run, number(&expected["generationAfterRaise"])));
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the raise retargeted the resident job, no rebuild replayed a checkpoint");
    set_target(&mut app, number(&expected["loweredTarget"]), &mut caller).await;
    pump_until(&mut app, "lowered target completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["loweredTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterLower"]));
    assert!(render_text(&mut app, "main", &mut caller).await.contains(&format!("count={}", number(&expected["loweredTarget"]))), "the retracted tail leaves the overlay");
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])));
    ingest_remote_count(&mut app, &mut probe, "tool-run-retarget-remote", number(&expected["remoteCount"]), &mut caller).await;
    set_target(&mut app, number(&expected["raisedTarget"]), &mut caller).await;
    pump_until(&mut app, "rebased and raised", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && app.tool_runs.provisional().len() as u64 == number(&expected["raisedTarget"]) * ops_per_unit && !app.tool_runs.is_refolding(), &mut caller).await;
    assert_eq!(app.tool_runs.progress().expect("progress").counters.first().map(|counter| counter.value), Some(number(&expected["resumedFrom"])), "the rebased run was rebound and retargeted in place");
    assert_eq!(app.snapshot().expect("committed").count, number(&expected["remoteCount"]) as i32, "reconfigure never commits");
    drop(probe);
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a retract followed by re-appends spread over several ticks keeps the previous overlay rendered until the
/// job reaches its checkpoint; only then does the refolded overlay replace it.
#[semio_framework_async_macros::async_test]
async fn tool_run_retract_keeps_the_previous_overlay_until_the_job_reaches_its_checkpoint() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["compact"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    let counts: Vec<u64> = expected["compactCounts"].as_array().expect("counts").iter().map(number).collect();
    for (index, _) in counts.iter().enumerate() {
        pump_until(&mut app, "a compaction page landed and was folded", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting) && app.tool_runs.provisional().len() == index + 1 && !app.tool_runs.has_pending_work(), &mut caller).await;
        let body = render_text(&mut app, "main", &mut caller).await;
        println!("[STATS] compact page {index}: rendered {body}");
        assert!(app.tool_runs.is_refolding(), "page {index}: the retract refold waits for its boundary");
        assert!(body.contains(text(&expected["renderedWhileCompacting"])), "page {index}: the previous overlay stays rendered: {body}");
        app.tool_runs.port().expect("the run's port").wake();
    }
    pump_until(&mut app, "the compaction checkpoint swaps the overlay", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let body = render_text(&mut app, "main", &mut caller).await;
    assert!(body.contains(text(&expected["renderedAfterCheckpoint"])), "the refolded overlay replaces the previous one at the boundary: {body}");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: the run's trace key allocator hands out keys above every key its ticks upserted and every key allocated
/// before, the request carries the entity marks, and the request names the window the run was started from.
#[semio_framework_async_macros::async_test]
async fn tool_run_requests_carry_trace_key_allocation_entity_marks_and_the_starting_window() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["traceKeys"];
    let window = &fixture["startWindow"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(text(&window["windowId"]), None));
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&fixture["toolId"]).into()))])), &meta, &mut caller.identity).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete), &mut caller).await;
    assert_eq!(app.tool_runs.window(), Some((text(&window["windowId"]), text(&window["windowKindId"]))));
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&fixture["toolId"]) && window_id.as_deref() == Some(text(&window["windowId"])) && !window_config), "the job request names the starting window: {requests:?}");
    let keys = app.tool_runs.trace_keys().expect("trace keys");
    assert_eq!(keys.next(), number(&expected["nextAfterRun"]));
    let allocated = keys.allocate(number(&expected["allocate"]));
    assert_eq!([allocated.start, allocated.end], [number(&expected["allocated"][0]), number(&expected["allocated"][1])]);
    assert_eq!(keys.next(), number(&expected["nextAfterAllocation"]));
    let marks = app.tool_runs.entity_marks();
    assert_eq!(marks.iter().map(|(_, entity)| *entity).collect::<Vec<_>>(), (1..=number(&expected["units"])).collect::<Vec<_>>());
    assert!(marks.iter().all(|(end, _)| *end as usize <= app.tool_runs.provisional().len()));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a board-2d scene surface carries the `toolRunTrace` lane exactly like a world-3d one — the spine names
/// the lane with its byte length and hash, and the carrier holds the whole trace delta of the run.
#[semio_framework_async_macros::async_test]
async fn tool_run_board_scene_render_carries_the_trace_lane() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["boardLane"];
    let mut app = toy_app(number(&expected["units"]), &mut caller).await;
    start(&mut app, text(&fixture["toolId"]), &mut caller).await;
    pump_until(&mut app, "run completes", |app| app.tool_runs.state() == Some(ToolRunState::Complete) && !app.tool_runs.is_refolding(), &mut caller).await;
    let mut view = window_view(text(&expected["windowId"]), None);
    view.window_instances.push(ViewWindowInstance { id: text(&expected["windowId"]).to_string(), window_kind_id: text(&expected["windowId"]).to_string() });
    let tree = app.render(text(&expected["bodyKey"]), None, &view).await.unwrap_or_else(|fault| panic!("render board: {fault:?}"));
    let scene = artifact_app_laws::observe_and_retire_fixture_tree(tree, |root| {
        let surface = root.children.iter().find(|child| child.key.as_str() == text(&expected["surfaceId"])).expect("the board body carries its scene surface");
        artifact_app_laws::built_surface_scene::<Board2dScene>(surface).expect("the board scene surface assembles")
    });
    let lane = scene.tool_run_trace.as_deref().expect("a board rendered during a run carries the toolRunTrace lane");
    let reference = scene.lanes.iter().find(|reference| reference.lane == text(&expected["laneName"])).expect("the spine names the injected lane");
    assert_eq!((reference.bytes as usize, reference.hash.as_str()), (lane.len(), semio_framework_ui_scene::scene_lane_hash(lane).as_str()));
    let delta = ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(lane).expect("base64url")).expect("one ToolRunTraceDelta");
    assert!(delta.clear, "a renderer without a cursor gets clear plus the whole log");
    assert_eq!(delta.pages.iter().map(|page| page.ops.len()).sum::<usize>() as u64, number(&expected["units"]));
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🪟️ Publishes `selected` into the world window config of `window_id` through the production emit path.
async fn publish_window_selection(app: &mut ToyApp, window_id: &str, selected: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let fixture = fixture();
    let mut view = window_view(window_id, None);
    view.window_instances.push(ViewWindowInstance { id: text(&fixture["windowSettingsReads"]["otherWindowId"]).to_string(), window_kind_id: text(&fixture["windowSettingsReads"]["windowKindId"]).to_string() });
    let mut meta = toy_meta();
    meta.view_state = Some(view);
    let mutation = WindowConfigMutation::of::<ToyWorldWindowConfig>(window_id, ChangeTestConfigSelection { selected: Some(selected.to_string()) }.into());
    app.dispatch_emit("setWorldSelection", Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { window_config_mutations: vec![mutation], ..Default::default() }, &meta, &mut caller.identity)
        .await
        .unwrap_or_else(|fault| panic!("window config publication: {fault:?}"));
}

/// ⚖️ LAW: a run started from a window reads the window-config fields it declares for that window's kind from the
/// starting window only — its job request carries that window's config snapshot, a publication on another window of the
/// same kind or of an unchanged value never reconfigures, and a changed value on the starting window does.
#[semio_framework_async_macros::async_test]
async fn tool_run_window_settings_reads_follow_the_starting_window_only() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["windowSettingsReads"];
    let (start_window, other_window) = (text(&expected["startWindowId"]), text(&expected["otherWindowId"]));
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    publish_window_selection(&mut app, start_window, 1, &mut caller).await;
    TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().clear());
    let mut meta = toy_meta();
    let mut view = window_view(start_window, None);
    view.window_instances.push(ViewWindowInstance { id: other_window.to_string(), window_kind_id: text(&expected["windowKindId"]).to_string() });
    meta.view_state = Some(view);
    let output = app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(text(&expected["toolId"]).into()))])), &meta, &mut caller.identity).await.expect("start").output;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"));
    pump_until(&mut app, "running with provisional units", |app| app.tool_runs.state() == Some(ToolRunState::Running) && !app.tool_runs.provisional().is_empty(), &mut caller).await;
    let requests = TOY_REQUEST_WINDOWS.with(|windows| windows.borrow().clone());
    assert!(requests.iter().any(|(tool, window_id, window_config)| tool == text(&expected["toolId"]) && window_id.as_deref() == Some(start_window) && *window_config), "the job request carries the starting window's config snapshot: {requests:?}");
    publish_window_selection(&mut app, other_window, 7, &mut caller).await;
    publish_window_selection(&mut app, start_window, 1, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterOtherWindow"]), "another window and an unchanged value never reconfigure");
    publish_window_selection(&mut app, start_window, 2, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    assert_eq!(u64::from(app.tool_runs.slot().expect("slot").generation), number(&expected["generationAfterStartWindow"]), "a changed value on the starting window reconfigures");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a window config partition closes through the physical release its own store publishes — its resident backings (the 1 024-slot
/// snapshot read registry alone is far over one ordinary close page) are priced by the app's close demand, so a published window config
/// reaches the exact terminal-empty witness instead of answering `Pending { 0, 0 }` against an ordinary-page grant for ever.
#[semio_framework_async_macros::async_test]
async fn a_published_window_config_closes_through_its_own_published_physical_demand() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(1, &mut caller).await;
    publish_window_selection(&mut app, text(&fixture["windowSettingsReads"]["startWindowId"]), 1, &mut caller).await;
    for _ in 0..8 {
        advance_toy(&mut app, &mut caller).await.expect("turn");
    }
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a MOUNTED typed operation that publishes a window config keeps its published receipt while the window authority refresh
/// is still yielding for the displaced snapshot's custody, resumes that refresh on its next unit and queues the page of exactly the
/// retained receipt — it never advances the already published store again ("awaits an ACK without its retained host page").
#[semio_framework_async_macros::async_test]
async fn a_mounted_window_config_publication_resumes_its_refresh_and_queues_the_retained_receipt() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let window_id = text(&fixture["windowSettingsReads"]["startWindowId"]);
    let mut app = toy_app(1, &mut caller).await;
    let view = window_view(window_id, None);
    let authority = app.stores.window_config_store.capture(Some(&view)).await.expect("window authority capture").expect("the world window owns a config partition");
    let generation_before = authority.snapshot.generation();
    let revision = app.stores.store.content_revision_now();
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(app.stores.store.generation_now()), 17);
    let lease = app.tools.tool_cancellations.begin(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).expect("cancellation lease");
    let mutation = WindowConfigMutation::of::<ToyWorldWindowConfig>(window_id, ChangeTestConfigSelection { selected: Some("5".into()) }.into());
    let mut mounted = test_blank_mounted_operation::<ToyRunApp>();
    mounted.verb = "setWorldSelection".into();
    mounted.meta = ActionMeta { actor: text(&fixture["actor"]).into(), instance_id: 7, view_state: Some(view) };
    mounted.operation = operation;
    mounted.canonical_revision = revision;
    mounted.artifact_generation = operation.generation.0;
    mounted.window_config_authority = Some(authority);
    mounted.publication_lanes = &[ArtifactToolPublicationLane::WindowConfig];
    mounted.cancellation_lease = Some(lease);
    mounted.terminal_seen = true;
    mounted.publication = Some(ArtifactToolCompletionValue::Emit(Ok(Emit::<TestMutation, TestConfigMutation, NoDraftMutation> { window_config_mutations: vec![mutation], ..Default::default() }), EphemeralEmit::default()));
    let grant = mounted_policy.maintenance;
    let mut yielded_with_retained_receipt = 0;
    let mut page = None;
    for _ in 0..4096 {
        app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, grant).unwrap_or_else(|fault| panic!("every unit of a window config publication resumes: {fault:?}"));
        mounted.publication_ownership_progress.take();
        if mounted.pending_window_config_receipt.is_some() {
            yielded_with_retained_receipt += 1;
        }
        if let Some(found) = mounted.take_result_page() {
            page = Some(found);
            break;
        }
    }
    let page = page.expect("the retained window config receipt reaches its result page");
    assert_eq!(page.lane, TypedOperationResultLane::WindowConfig);
    assert!(yielded_with_retained_receipt > 0, "the first refresh unit yields while the displaced snapshot awaits custody");
    assert!(mounted.pending_window_config_receipt.is_none() && mounted.published_window_config);
    assert!(mounted.window_config_authority.as_ref().is_some_and(|authority| authority.snapshot.generation() > generation_before), "the captured authority was refreshed to the published generation");
    assert!(mounted.acknowledge_result_page(page.token).expect("exact ACK"));
    let mut terminal = false;
    for _ in 0..4096 {
        app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, grant).unwrap_or_else(|fault| panic!("every unit after the window config ACK resumes: {fault:?}"));
        mounted.publication_ownership_progress.take();
        if mounted.granted_retirement_ready() && mounted.granted_retirement_pending() {
            let _demand = mounted.granted_retirement_demands(grant.maximum_copy_bytes).expect("quoted granted retirement");
            let quoted = grant;
            mounted.granted_retirement_step(quoted).expect("granted retirement of the acknowledged publication");
        }
        if let Some(page) = mounted.take_result_page() {
            terminal = page.lane == TypedOperationResultLane::Terminal;
            assert!(mounted.acknowledge_result_page(page.token).expect("exact ACK"));
            break;
        }
    }
    assert!(terminal, "the acknowledged window config operation ends with its terminal page");
    for _ in 0..4096 {
        if mounted.terminal_is_empty() {
            break;
        }
        if mounted.lease_retirement_ready() {
            let _demand = mounted.lease_retirement_demands(&app.tools.tool_cancellations, grant.maximum_copy_bytes).expect("quoted cancellation lease retirement");
            let quoted = grant;
            mounted.lease_retirement_step(&app.tools.tool_cancellations, quoted).expect("cancellation lease retirement");
            continue;
        }
        let _demand = mounted.retirement_demands(grant.maximum_copy_bytes).expect("quoted mounted retirement");
            let quoted = grant;
        match mounted.retirement_step(quoted).expect("mounted retirement step") {
            PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => assert!(progress.fits(quoted)),
            PluginLifecycleStep::AwaitingInput { reason } | PluginLifecycleStep::Blocked { reason } => panic!("the published window config operation retires: {reason}"),
        }
    }
    assert!(mounted.terminal_is_empty(), "the published window config operation retires every owner");
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a tick dirties the bodies of the window kinds the run declares it renders in, next to the panel and the scene
/// windows, and those bodies read the run's progress, step ring and latest tick payload through `ArtifactView::tool_run()`.
#[semio_framework_async_macros::async_test]
async fn tool_run_reader_windows_refresh_every_tick_and_read_progress_steps_and_payload() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readerWindows"];
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "job admitted", |app| app.tool_runs.state() == Some(ToolRunState::Running), &mut caller).await;
    render_world(&mut app, None, &mut caller).await;
    app.tool_run_ctx().flush_tool_run_ui_dirty();
    while app.take_typed_operation_ui_scope().is_some() {}
    let before = app.tool_runs.provisional().len();
    pump_until(&mut app, "a tick lands", |app| app.tool_runs.provisional().len() > before, &mut caller).await;
    let scope = app.take_typed_operation_ui_scope().expect("a tick owes a dirty scope");
    let strings = |value: &Value| value.as_array().expect("strings").iter().map(|item| text(item).to_string()).collect::<Vec<_>>();
    assert_eq!(
        scope,
        UiDirtyScope::Partial { window_bodies: strings(&expected["windowBodies"]), panel_bodies: strings(&expected["panelBodies"]), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        "the reader window refreshes with every tick, nothing unrelated does"
    );
    run_action(&mut app, "toolRunPause", &mut caller).await;
    pump_until(&mut app, "paused and settled", |app| app.tool_runs.state() == Some(ToolRunState::Paused) && !app.tool_runs.has_pending_work(), &mut caller).await;
    let view = app.tool_runs.view().expect("run view");
    let payload = view.payload.as_deref().map(Vec::as_slice).and_then(|payload| payload.try_into().ok()).map(u32::from_le_bytes).expect("the latest tick payload");
    assert_eq!(u64::from(payload), view.progress.completed, "the payload is the latest tick's: the toy writes its completed units");
    assert!(view.progress.total.is_some_and(|total| total == number(&expected["target"])));
    let body = render_text(&mut app, "main", &mut caller).await;
    println!("[STATS] reader body {body}");
    assert!(body.contains(&format!("completed={} steps={} payload={payload}", view.progress.completed, view.progress.steps.len())), "the reader body renders the run state it reads: {body}");
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ▶️ Starts `tool_id` from window `window_id` and answers the dispatch output.
async fn start_in_window(app: &mut ToyApp, tool_id: &str, window_id: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> DslValue {
    let mut meta = toy_meta();
    meta.view_state = Some(window_view(window_id, None));
    app.handle_action("toolRunStart", Some(&DslValue::Object(vec![("toolId".into(), DslValue::String(tool_id.into()))])), &meta, &mut caller.identity).await.unwrap_or_else(|fault| panic!("start {tool_id}: {fault:?}")).output
}

/// ⚖️ LAW: read-only runs run concurrently, one per (tool, window), each with its own run id, trace and panel group; a
/// second mutating start while a mutating run is non-terminal is `toolRun.busy`; actions address a run by its `runId`;
/// presence follows the mutating run.
#[semio_framework_async_macros::async_test]
async fn read_only_runs_in_two_windows_run_concurrently_and_a_second_mutating_start_is_busy() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["concurrentReadOnly"];
    let read_only = text(&expected["readOnlyToolId"]);
    let windows: Vec<&str> = expected["windows"].as_array().expect("windows").iter().map(text).collect();
    let mut app = toy_app(number(&expected["target"]), &mut caller).await;
    for window in &windows {
        assert_eq!(start_in_window(&mut app, read_only, window, &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "{window}: a read-only run starts beside the other window's");
    }
    assert_eq!(start_in_window(&mut app, read_only, windows[0], &mut caller).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "one non-terminal read-only run per tool and window");
    pump_until(&mut app, "both read-only runs are running and waiting on their hop", |app| {
        let views = app.tool_runs.views();
        views.len() == 2 && views.iter().all(|view| view.state == ToolRunState::Running)
    }, &mut caller)
    .await;
    let views = app.tool_runs.views();
    assert_ne!(views[0].identity.id.run, views[1].identity.id.run, "each run has its own id");
    for (window, view) in windows.iter().zip(&views) {
        assert_eq!(app.tool_runs.view_for(Some(window)).map(|found| found.identity.id.run), Some(view.identity.id.run), "{window} renders its own run");
    }
    assert_eq!(start_in_window(&mut app, text(&expected["mutatingToolId"]), windows[0], &mut caller).await.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "a mutating run starts beside read-only runs");
    assert_eq!(start_in_window(&mut app, text(&expected["secondMutatingToolId"]), windows[1], &mut caller).await.get("rejected").and_then(DslValue::as_str), Some(text(&expected["busy"])), "a second mutating start is busy");
    assert_eq!(app.tool_run_presence().map(|presence| presence.tool_id), Some(text(&expected["mutatingToolId"]).to_string()), "presence follows the mutating run");
    let panel: Value = serde_json::from_str(&render_text(&mut app, FRAMEWORK_TOOL_RUN_BODY_KEY, &mut caller).await).expect("panel parses");
    for view in app.tool_runs.views() {
        assert!(find_node(&panel, &panel_id(&fixture["panel"]["groupId"], view.identity.id.run)).is_some(), "run {} has its own panel group", view.identity.id.run);
        let delta = app
            .tool_runs
            .trace_delta(Some(ToolRunTraceCursor { run: view.identity.id.run, generation: view.identity.generation + 1, page: 0 }), usize::MAX)
            .map(|lane| ToolRunTraceDelta::decode(&base64_codec::base64_url_decode(&lane).expect("base64url")).expect("delta"));
        assert_eq!(delta.map(|delta| delta.identity.id.run), Some(view.identity.id.run), "run {} has its own trace", view.identity.id.run);
    }
    let first = views[0].clone();
    let aborted = tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(first.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(first.identity.generation)))], &mut caller).await;
    assert_eq!(aborted.get("toolRun").and_then(DslValue::as_str), Some("closeJob"), "an action addresses its run by runId");
    pump_until(&mut app, "the addressed run aborts alone", |app| app.tool_runs.views().iter().any(|view| view.identity.id.run == first.identity.id.run && view.state == ToolRunState::Aborted), &mut caller).await;
    let states: Vec<(u64, ToolRunState)> = app.tool_runs.views().iter().map(|view| (view.identity.id.run, view.state)).collect();
    println!("[STATS] concurrent runs {states:?}");
    assert!(states.iter().filter(|(run, _)| *run != first.identity.id.run).all(|(_, state)| !state.is_terminal()), "the other runs keep running: {states:?}");
    for view in app.tool_runs.views().into_iter().filter(|view| !view.state.is_terminal()) {
        tool_run_action(&mut app, "toolRunAbort", vec![("runId".into(), DslValue::String(view.identity.id.run.to_string())), ("generation".into(), DslValue::uint(u64::from(view.identity.generation)))], &mut caller).await;
    }
    pump_until(&mut app, "every run settles", |app| app.tool_runs.views().iter().all(|view| view.state.is_terminal()) && !app.tool_runs.has_pending_work(), &mut caller).await;
    close(&mut app, &mut caller);

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🧾️ A `BuiltNode` as the host wire JSON every renderer's `BuiltNode` reads — every field spelled out.
fn built_node_wire(node: &BuiltNode) -> Value {
    serde_json::json!({
        "key": node.key.as_str(),
        "component": serde_json::to_value(&node.component).expect("component"),
        "layout": serde_json::to_value(&node.layout).expect("layout"),
        "style": serde_json::to_value(node.style).expect("style"),
        "activity": serde_json::to_value(node.activity).expect("activity"),
        "disabled": node.disabled,
        "accessibility": serde_json::to_value(&node.accessibility).expect("accessibility"),
        "bindings": serde_json::to_value(&node.bindings).expect("bindings"),
        "menu": serde_json::to_value(&node.menu).expect("menu"),
        "children": node.children.iter().map(built_node_wire).collect::<Vec<_>>(),
    })
}

/// 🪧️ The language-neutral panel every shell target mounts: the framework ToolRun panel `BuiltNode` of a running run
/// (`🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json`). React (`🛠️ShellHelpers` panel law) and wgpu (`🐚️Shell`
/// panel law) render it and dispatch its buttons from this exact document.
const TOOL_RUN_PANEL_RUNNING: &str = include_str!("../🧫️fixtures/⏯️tool-run/🪧️panel-running.json");

/// ⚖️ LAW: the ToolRun panel of a running run is exactly the committed shell fixture — one run group with a polite status,
/// a progressbar, real buttons addressing the run by id (pause and abort enabled, step disabled, finalize disabled with
/// its description) and an empty step log and trace list. `SEMIO_TOOL_RUN_PANEL_OUT` rewrites the fixture.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_of_a_running_run_is_the_shell_fixture() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&fixture["port"]["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.state() == Some(ToolRunState::Running) && app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    if let Some(path) = std::env::var_os("SEMIO_TOOL_RUN_PANEL_OUT") {
        std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(&panel).expect("pretty panel"))).expect("write the panel fixture");
    }
    assert_eq!(panel, serde_json::from_str::<Value>(TOOL_RUN_PANEL_RUNNING).expect("the panel fixture parses"), "the running panel is the shell fixture");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// 🔁️ Every sibling key list under `node` names each key once — the admission law a retained UI surface enforces.
fn assert_unique_sibling_keys(node: &Value) {
    let children = node["children"].as_array().map(Vec::as_slice).unwrap_or_default();
    let mut keys: Vec<&str> = children.iter().filter_map(|child| child["key"].as_str()).collect();
    keys.sort_unstable();
    let before = keys.len();
    keys.dedup();
    assert_eq!(keys.len(), before, "duplicate sibling keys under {}", node["key"]);
    children.iter().for_each(assert_unique_sibling_keys);
}

/// ⚖️ LAW: a trace key the run re-upserts with a new verdict (testing → success/danger, the shape of every
/// collision-tested candidate) is one attempt row, moved to the newest position — never a duplicate sibling that
/// makes the panel unadmittable.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_lists_a_re_upserted_trace_key_once_at_its_newest_position() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["verdictRevisions"];
    let mut app = toy_app(1, &mut caller).await;
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let identity = app.tool_runs.identity().expect("identity");
    let mut writer = ToolRunTickWriter::new(identity);
    for upsert in expected["upserts"].as_array().expect("upserts") {
        let key = number(&upsert[0]);
        writer.upsert(key, ToolRunVerdict::parse(text(&upsert[1])).expect("verdict"), 1, ToolRunTraceSubject::Entity { entity: key });
        app.tool_runs.apply_tick(writer.finish().expect("an upsert tick")).expect("the upsert tick applies");
    }
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render the panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let scope = semio_framework_tool_run::tool_run_panel_group_id(identity.id.run);
    let rows: Vec<&str> = find_node(&panel, &format!("{scope}.trace")).expect("the attempts tree")["children"].as_array().expect("rows").iter().filter_map(|row| row["key"].as_str()).collect();
    let expected_rows: Vec<String> = expected["traceRows"].as_array().expect("trace rows").iter().map(|key| format!("{scope}.trace.{}", number(key))).collect();
    assert_eq!(rows, expected_rows, "one attempt row per trace key, newest first");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: while the active tool declares a run and this instance holds none of it, the panel offers a ready group
/// whose enabled Start dispatches `toolRunStart` with the tool id; once the run exists its own group replaces it.
#[semio_framework_async_macros::async_test]
async fn tool_run_panel_offers_start_for_the_active_run_tool_until_its_run_exists() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["readyGroup"];
    let mut app = toy_app(1, &mut caller).await;
    let view = ViewModel { active_tool_id: Some(text(&expected["toolId"]).to_string()), ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the idle panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert_unique_sibling_keys(&panel);
    let group = find_node(&panel, text(&expected["groupId"])).expect("the ready group");
    assert_eq!(group["children"][0]["component"]["value"], expected["status"], "the ready status");
    let start_button = find_node(group, text(&expected["startButtonId"])).expect("the ready Start button");
    assert_eq!(start_button["component"]["label"], expected["startLabel"]);
    assert_eq!(start_button["disabled"], false, "Start is enabled with no run");
    assert_eq!(start_button["bindings"][0]["action"]["name"], "toolRunStart");
    assert_eq!(start_button["bindings"][0]["args"]["toolId"], expected["toolId"], "Start names the active tool");
    start(&mut app, text(&expected["toolId"]), &mut caller).await;
    pump_until(&mut app, "the run waits on its first hop", |app| app.tool_runs.port().is_some_and(ToolRunJobPort::is_waiting), &mut caller).await;
    while app.take_typed_operation_effect().is_some() {}
    let tree = app.render(FRAMEWORK_TOOL_RUN_BODY_KEY, None, &view).await.expect("render the running panel");
    let panel = artifact_app_laws::observe_and_retire_fixture_tree(tree, built_node_wire);
    assert!(find_node(&panel, text(&expected["groupId"])).is_none(), "the run's group replaces the ready group");
    app.tool_runs.port().expect("the run's port").wake();
    abort_and_close(&mut app, &mut caller).await;

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

/// ⚖️ LAW: a TERMINAL run arms nothing. Once a run is finalized or aborted the ledger owes the driver no work
/// and hands the host no effect, however many turns the host takes.
///
/// 🪪️ Every wake a run hands the host is a REQUEST the host answers by dispatching an action back into the
/// guest, so a request a run that is OVER keeps re-making is an endless action storm on a shell nobody is
/// touching. Generation3d's finalized read-only `previewEval` re-armed its pace wake once per presentation
/// retry and filled the console with `toolRunPace` on a quiet, converged editor — seq 107…116 within seconds,
/// 2 800 dropped console lines (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, 2026-09-14 23:25). The latch is read
/// once per ANSWER, never once per REFRESH, and a terminal run answers nothing.
#[semio_framework_async_macros::async_test]
async fn a_terminal_run_arms_no_further_work_however_many_turns_the_host_takes() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    let fixture = fixture();
    let expected = &fixture["terminalQuiet"];
    let turns = number(&expected["turns"]);
    for row in expected["rows"].as_array().expect("the terminalQuiet rows") {
        let id = text(&row["id"]);
        let mut app = toy_app(number(&row["units"]), &mut caller).await;
        start(&mut app, text(&row["toolId"]), &mut caller).await;
        if text(&row["terminalBy"]) == "abort" {
            pump_until(&mut app, "the run leaves starting", |app| app.tool_runs.state().is_some_and(|state| state != ToolRunState::Starting), &mut caller).await;
            run_action(&mut app, "toolRunAbort", &mut caller).await;
        }
        pump_until(&mut app, "the run reaches a terminal state", |app| app.tool_runs.state().is_some_and(ToolRunState::is_terminal) && !app.tool_runs.has_pending_work(), &mut caller).await;
        while app.take_typed_operation_effect().is_some() {}
        let mut armed: Vec<String> = Vec::new();
        let mut pending_turns = 0u64;
        for _ in 0..turns {
            if app.tool_runs.has_pending_work() {
                pending_turns += 1;
            }
            advance_toy(&mut app, &mut caller).await.unwrap_or_else(|fault| panic!("{id}: a quiet turn faulted: {fault:?}"));
            while let Some(effect) = app.take_typed_operation_effect() {
                armed.push(format!("{effect:?}"));
            }
        }
        let state = app.tool_runs.state().expect("a terminal state");
        println!("[STATS] terminalQuiet {id}: state={} turns={turns} pendingWorkTurns={pending_turns} armed={}", state.as_str(), armed.len());
        assert!(state.is_terminal(), "{id}: the run stays terminal");
        assert_eq!(pending_turns == 0, expected_flag(&row["expected"]["hasPendingWork"]) == false, "{id}: a terminal run is no driver work over {turns} turns");
        assert_eq!(armed.len() as u64, number(&row["expected"]["effects"]), "{id}: a terminal run arms nothing, got {armed:?}");
        close(&mut app, &mut caller);
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}

fn expected_flag(value: &Value) -> bool {
    value.as_bool().expect("a boolean expectation")
}

//#endregion 🎯️RetargetAndSettings

//#region 🧹️RetainedConfigCloseCliff
/// 🧹️ A retained config larger than one `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES` page must still reach
/// terminal-empty AFTER a body has rendered against it.
///
/// 🏁️ Measured 2026-09-16 (ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS): with a body rendered,
/// a config over ~one page livelocked `close_step(1, 4_096)` at `Pending { 0, 0 }` for 15 M turns,
/// while the same config closed with no render. The render backfills the command log, whose LABEL
/// carries the applied config op text; `close_retained_fields_step` priced that whole log entry
/// against one turn grant and pushed it back unreleased every turn. The cliff capped EVERY app's
/// retained config at one page regardless of its declared maximum (cad declares 65 536 B), so the
/// `sourcing.module` / `cad.computer` / `process.machines` host packs could never be retained.
#[semio_framework_async_macros::async_test]
async fn a_retained_config_over_one_envelope_page_closes_after_a_render() {
    let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original external fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

    let (mounted_policy,maximum_identity_bytes)=toy_fixture_caller_policy();
    let mut observer=|_|true;
    let identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(maximum_identity_bytes,&mut observer).unwrap();
    let mut caller=ToyFixtureCaller{context:cx,policy:mounted_policy,identity,receipts:Vec::new()};

    for (bytes, rendered) in [(2_909usize, true), (3_706, true), (4_360, true), (16_384, true), (65_536, true), (3_820, false)] {
        let mut app = toy_app(1, &mut caller).await;
        app.stores.config_store
            .dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some("c".repeat(bytes)) }.into()], transaction: None }, &mut caller.identity)
            .await
            .expect("a retained config past one envelope page applies");
        if rendered {
            let _ = render_text(&mut app, "main", &mut caller).await;
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        let mut turns = 0u64;
        let mut productive = 0u64;
        let mut last = String::new();
        while std::time::Instant::now() < deadline {
            if app.close_terminal_is_empty() {
                break;
            }
            turns += 1;
            let demand = app.close_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("original fixture close demand");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_copy_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, maximum_release_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ..caller.policy.close };
            assert!(demand.copy_bytes<=grant.maximum_copy_bytes && demand.capacity_bytes<=grant.maximum_capacity_bytes && demand.release_bytes<=grant.maximum_release_bytes && demand.depth<=grant.maximum_depth,"actual demand fits original independent page and caller ceilings");
            assert!(grant.maximum_copy_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES && grant.maximum_release_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, "original fixture body and physical grant limits remain unchanged");
            match { let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(caller.context,grant,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty()))).expect("original close step"); let _ = app.take_constructor_receipt(); step } {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) => {
                    assert!(progress.fits(grant));
                    let released_items = progress.copied_items;
                    let released_bytes = progress.released_bytes;
                    assert!(released_items <= 1 && released_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, "a close page stays inside its exact grant");
                    if released_items != 0 || released_bytes != 0 {
                        productive = turns;
                    }
                    last = format!("Pending {{ {released_items}, {released_bytes} }}");
                }
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress) => { assert!(progress.fits(grant)); assert!(app.close_terminal_is_empty()); break; },
                step => last = format!("{step:?}"),
            }
        }
        println!("[STATS] retainedConfigClose bytes={bytes} rendered={rendered} turns={turns} lastProductiveTurn={productive} last={last}");
        assert!(app.close_terminal_is_empty(), "a {bytes} B retained config (rendered={rendered}) must reach terminal-empty; last step {last} after {turns} turns");
        assert_eq!(productive, turns, "every close turn released something; the pump never spends a turn on zero progress");
    }

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original external fixture paid terminal retirement");result
}
//#endregion 🧹️RetainedConfigCloseCliff

#[doc(hidden)]
#[path = "../🧪️tool-run-member/🦀️.rs"]
pub mod member;