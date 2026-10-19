//! 

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_precompute_fill::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::VORTEX_MARKER_MESH_KIND;
use semio_s_artifact_puzzle_3d_precompute_brush::brush_fill_candidate_at;
use semio_s_artifact_puzzle_3d_precompute_brush::brush_object_id;
use semio_s_artifact_puzzle_3d_precompute_brush::brush_preview_from_candidate;
use semio_s_artifact_puzzle_3d_precompute_brush::fill_rng;
use semio_s_artifact_puzzle_3d_precompute_brush::resolve_object_kind_mesh_url;
use semio_s_artifact_puzzle_3d_precompute_brush::resolve_placed_object_mesh_url;
use semio_s_artifact_puzzle_3d_precompute_brush::vortex_world_from_object;
use semio_s_artifact_puzzle_3d_precompute_brush::AttractionVortexContext;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushCatalogView;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushFillVortexTarget;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSceneView;
use semio_s_artifact_puzzle_3d_precompute_brush::TargetVortexWorld;
use semio_s_artifact_puzzle_3d_precompute_geometry::pose_isometry;
use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work;
use semio_s_artifact_puzzle_3d_precompute_geometry::world_bounds;
use semio_s_artifact_puzzle_3d_precompute_geometry::world_volumes_contain_aabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionAabb;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionBody;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexMutation;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepContext;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexOwner;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexRejectedOwner;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionIndexRemoval;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionMutationStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionPenetrationState;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionQueryCursor;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionQueryStep;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionSpatialIndex;
use semio_s_artifact_puzzle_3d_precompute_geometry::CollisionStepResult;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerMap;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerMapInsert;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerSet;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerSetInsert;
use semio_s_artifact_puzzle_3d_precompute_geometry::FixedOwnerVec;
use semio_s_artifact_puzzle_3d_precompute_geometry::Pose3d;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_ATTRACTION_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_CANDIDATE_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_KIND_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_OBJECT_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_VOLUME_SLOTS;
use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_VORTEX_SLOTS;
use {semio_s_artifact_puzzle_3d_editor_snapshot::empty_scene_snapshot, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject};
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::AttractionProps;
use semio_s_artifact_puzzle_3d_model::BrushCompatibleCandidate;
use semio_s_artifact_puzzle_3d_model::BrushHostRules;
use semio_s_artifact_puzzle_3d_model::BrushPlacePayload;
use semio_s_artifact_puzzle_3d_model::BrushPreviewState;
use semio_s_artifact_puzzle_3d_model::CableKindCatalog;
use semio_s_artifact_puzzle_3d_model::EngineSceneObject;
use semio_s_artifact_puzzle_3d_model::FillRunCheckpoint;
use semio_s_artifact_puzzle_3d_model::FillRunCounter;
use semio_s_artifact_puzzle_3d_model::FillRunReason;
use semio_s_artifact_puzzle_3d_model::FillRunStage;
use semio_s_artifact_puzzle_3d_model::KindCompatEntry;
use semio_s_artifact_puzzle_3d_model::KindCatalogBundle;
use semio_s_artifact_puzzle_3d_model::ObjectKind;
use semio_s_artifact_puzzle_3d_model::SceneConfig;
use semio_s_artifact_puzzle_3d_model::VortexKindCatalog;
use semio_s_artifact_puzzle_3d_model::VortexProps;
use semio_s_artifact_puzzle_3d_model::WorldVolumeProps;
use semio_s_plugin_puzzle_jobs::JobTurn;
use semio_framework_job::Generation;
use semio_framework_job::InteractiveJob;
use semio_framework_job::Operation;
use semio_framework_job::OperationId;
use semio_framework_tool_run::ToolRunCounter;
use semio_framework_tool_run::ToolRunIdentity;
use semio_framework_tool_run::ToolRunProgress;
use semio_framework_tool_run::ToolRunState;
use semio_framework_tool_run::ToolRunStep;
use semio_framework_tool_run::ToolRunStepArg;
use semio_framework_tool_run::ToolRunStepKind;
use semio_framework_tool_run::ToolRunStepRing;
use semio_framework_tool_run::ToolRunTick;
use semio_framework_tool_run::ToolRunTickWriter;
use semio_framework_tool_run::ToolRunTraceOp;
use semio_framework_tool_run::ToolRunTracePage;
use semio_framework_tool_run::ToolRunTraceSubject;
use semio_framework_tool_run::ToolRunVerdict;
use semio_framework_tool_run::TOOL_RUN_PROVISIONAL_OPS_MAX;
use semio_framework_tool_run::TOOL_RUN_REASON_CONFLICT;
use semio_framework_tool_run::TOOL_RUN_REASON_PROVISIONAL_CAP;
use std::collections::HashMap;
use std::sync::Arc;

fn faulted(turn: JobTurn) -> bool {
    matches!(turn, JobTurn::Fault(_))
}

use semio_s_artifact_puzzle_3d_precompute_geometry::collision_body_from_buffers;

use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work_clock;

use semio_s_artifact_puzzle_3d_precompute_geometry::precompute_work_done;

use semio_s_artifact_puzzle_3d_precompute_geometry::OwnerReservationLimit;

use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_CELL_MEMBER_SLOTS;

use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_CELL_SLOTS;

use semio_s_artifact_puzzle_3d_precompute_geometry::DOCUMENT_OWNER_PAGE_BYTES;

use semio_s_artifact_puzzle_3d_precompute_geometry::FIXED_OWNER_PAGE_BYTES;

use semio_s_artifact_puzzle_3d_precompute_geometry::FIXED_OWNER_SLOTS;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use semio_s_artifact_puzzle_3d_model::BrushKindWeights;
use semio_s_artifact_puzzle_3d_model::EngineSceneSnapshot;
use semio_s_artifact_puzzle_3d_model::ObjectKindRepresentation;
use semio_s_artifact_puzzle_3d_model::ObjectKindVortexTemplate;
use semio_framework_job::RevisionId;

/// 🧮️ Primitive work one fill builder step may cost ([`precompute_work_done`]): one transition of the cursorized planner —
/// one preparation item, one candidate, one broad-phase lookup or one collision probe. The measured maximum is 177 units
/// (the adversarial broad-phase plan; an empty scene's steps cost 1).
const FILL_STEP_WORK_CEILING: u64 = 512;

/// 🧮️ Primitive work the adversarial broad-phase plan may spend before its first candidate is on screen. It measures
/// 1490 units.
const FILL_FIRST_CANDIDATE_WORK_CEILING: u64 = 4_096;

/// 🎯️ What a fresh editor asks for — the product default, never a planner ceiling.
const TEST_REQUESTED_COUNT: usize = 100;

fn empty_builder() -> FillBuilderFixture {
    let scene = Arc::new(SceneConfig {
        scene_snapshot: EngineSceneSnapshot::default(),
        kind_catalogs: Some(KindCatalogBundle::default()),
        kind_compatibility: Vec::new(),
        contact_tolerance: 0.0,
        seed: 17,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    });
    FillBuilderFixture::new(FillBuilder::begin_preparation(FillPreparationRoots::new(scene, Arc::new(FillMeshMap::new())), Operation::new(OperationId(1), RevisionId(1), Generation(1), 17), TEST_REQUESTED_COUNT))
}

fn builder_clock()->Option<u64>{Some(0)}
struct FillBuilderFixture{builder:Option<FillBuilder>,original:Option<semio_s_plugin_puzzle_jobs::testing::OriginalFixture>}
impl FillBuilderFixture{
 fn new(builder:FillBuilder)->Self{let operation=builder.operation;Self{builder:Some(builder),original:Some(semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(operation.operation,operation.generation,semio_s_plugin_puzzle_jobs::testing::fixture_stock(),builder_clock,u64::MAX,1<<48))}}
 fn advance(&mut self,fuel:u64,deadline:u64)->JobTurn{let builder=self.builder.as_mut().unwrap();let mut outcome=None;self.original.as_mut().unwrap().turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(),|cx|cx.with_original_work_loan(fuel,deadline,|cx|{outcome=Some(builder.advance(cx));Ok(())})).expect("same original fill builder turn");outcome.unwrap()}
 fn advance_foreign(&mut self,fuel:u64,deadline:u64)->JobTurn{let operation=self.operation;let mut foreign=semio_s_plugin_puzzle_jobs::testing::OriginalFixture::new(operation.operation,Generation(operation.generation.0+1),semio_s_plugin_puzzle_jobs::testing::fixture_stock(),builder_clock,u64::MAX,1<<48);let mut outcome=None;foreign.turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(),|cx|cx.with_original_work_loan(fuel,deadline,|cx|{outcome=Some(self.builder.as_mut().unwrap().advance(cx));Ok(())})).expect("actual foreign source rejection turn");foreign.finish();outcome.unwrap()}
 fn cancel(&self){self.original.as_ref().unwrap().cancel().cancel_now();}
 fn close(&mut self){if let Some(original)=self.original.as_mut(){let builder=self.builder.as_mut().unwrap();builder.begin_close();for _ in 0..1_000_000{if builder.terminal_is_empty(){break}original.turn(semio_s_plugin_puzzle_jobs::testing::fixture_stock(),|cx|builder.close_turn(cx)).expect("same original fill builder close");}assert!(builder.terminal_is_empty(),"original fill builder close finishes");}self.builder.take();if let Some(original)=self.original.take(){original.finish();}}
}
impl std::ops::Deref for FillBuilderFixture{type Target=FillBuilder;fn deref(&self)->&FillBuilder{self.builder.as_ref().unwrap()}}
impl std::ops::DerefMut for FillBuilderFixture{fn deref_mut(&mut self)->&mut FillBuilder{self.builder.as_mut().unwrap()}}
impl Drop for FillBuilderFixture{fn drop(&mut self){if !std::thread::panicking(){self.close();}}}

include!("🧮️planner-laws/🦀️.rs");

include!("⏯️fill-run-laws/🦀️.rs");

include!("🛑️run-end-laws/🦀️.rs");

/// 🧮️ Every original close frontier preserves custody under denial and records its actual allocator event.
#[test]
fn original_fill_builder_close_denials_and_receipts_match_actual_heap(){
 use semio_framework_value::RetainedCloneGrant as Grant;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️caller/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=&law["closeGrant"];
 let policy=Grant{maximum_items:row["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:row["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:row["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};
 for cut in law["closeCuts"].as_array().unwrap(){let mut fixture=empty_builder();for _ in 0..cut.as_u64().unwrap(){let _=fixture.advance(100,10);}let builder=fixture.builder.as_mut().unwrap();let original=fixture.original.as_mut().unwrap();builder.begin_close();let mut turns=0;
  while !builder.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||builder.close_demands(policy.maximum_copy_bytes).expect("actual original builder close quote"));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denials=vec![Grant{maximum_items:0,..policy},Grant{maximum_depth:0,..policy}];if demand.copy_bytes>0{denials.push(Grant{maximum_copy_bytes:demand.copy_bytes-1,..policy});}if demand.capacity_bytes>0{denials.push(Grant{maximum_capacity_bytes:demand.capacity_bytes-1,..policy});}if demand.release_bytes>0{denials.push(Grant{maximum_release_bytes:demand.release_bytes-1,..policy});}
   for denied in denials{if let Ok(selected)=builder.close_demands(denied.maximum_copy_bytes){if selected.copy_bytes<=denied.maximum_copy_bytes&&selected.capacity_bytes<=denied.maximum_capacity_bytes&&selected.release_bytes<=denied.maximum_release_bytes&&selected.depth<=denied.maximum_depth&&denied.maximum_items>0{continue}}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn(denied,|cx|builder.close_turn(cx)).expect("denied original builder cursor remains retained"));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!builder.terminal_is_empty());assert_eq!(builder.close_demands(policy.maximum_copy_bytes).unwrap(),demand);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn(policy,|cx|builder.close_turn(cx)).expect("funded original builder cursor"));let progress=step.progress();assert!(progress.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(progress,Default::default(),"authored fixed close policy must fund its next action: {demand:?}");turns+=1;assert!(turns<1_000_000);
  }
  let owner=fixture.builder.take().unwrap();let original=fixture.original.take().unwrap();let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));original.finish();println!("[DEBUG] actual original fill builder cut={cut} closeTurns={turns} all denied currencies unchanged0heap actual receipts and terminalDrop0");
 }
}

/// ♻️ Each whole run retains its actual source until every original owner and descriptor has a paid terminal receipt.
fn original_fill_job_close_law<J:InteractiveJob>(mut job:J,mut original:semio_s_plugin_puzzle_jobs::testing::OriginalFixture,quote:impl Fn(&J,usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>){
 use semio_framework_value::RetainedCloneGrant as Grant;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️caller/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=&law["closeGrant"];let policy=Grant{maximum_items:row["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:row["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:row["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};let mut turns=0;
 while !job.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||quote(&job,policy.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denials=vec![Grant{maximum_items:0,..policy}];if demand.depth>0{denials.push(Grant{maximum_depth:demand.depth-1,..policy});}if demand.copy_bytes>0{denials.push(Grant{maximum_copy_bytes:demand.copy_bytes-1,..policy});}if demand.capacity_bytes>0{denials.push(Grant{maximum_capacity_bytes:demand.capacity_bytes-1,..policy});}if demand.release_bytes>0{denials.push(Grant{maximum_release_bytes:demand.release_bytes-1,..policy});}
  for denied in denials{if let Ok(selected)=quote(&job,denied.maximum_copy_bytes){if selected.copy_bytes<=denied.maximum_copy_bytes&&selected.capacity_bytes<=denied.maximum_capacity_bytes&&selected.release_bytes<=denied.maximum_release_bytes&&selected.depth<=denied.maximum_depth&&denied.maximum_items>0{continue}}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn(denied,|cx|job.close_step(cx)).expect("original whole-job denied close preserves its owner"));assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!job.terminal_is_empty());assert_eq!(quote(&job,policy.maximum_copy_bytes).unwrap(),demand);}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||original.turn(policy,|cx|job.close_step(cx)).expect("original whole-job fixed close loan"));let progress=step.progress();assert!(progress.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_ne!(progress,Default::default(),"fixed authored close grant cannot advance actual demand: {demand:?}");turns+=1;assert!(turns<1_000_000);
 }
 let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));original.finish();println!("[DEBUG] whole original fill job closeTurns={turns} actual allocator receipt all denied currencies0 terminalDrop0");
}

/// 🧮️ Run writer, maps, arrays and revalidation root owners all participate in the actual paid terminal close.
#[test]
fn original_fill_run_and_revalidation_close_denials_match_actual_heap(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️caller/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for cut in law["closeCuts"].as_array().unwrap(){let mut run=FillRunFixture::new(FillRunJob::new(FillBuilder::begin_preparation(own_mesh_roots(false,0.0),Operation::new(OperationId(97),RevisionId(1),Generation(1),1),4),fill_run_identity(),own_mesh_lane(),[0;32]),never);for _ in 0..cut.as_u64().unwrap(){let _=run.advance(100,u64::MAX);}let job=run.job.take().unwrap();let original=run.original.take().unwrap();original_fill_job_close_law(job,original,FillRunJob::close_demands);
  let (placed,_)=nakagin_scale_run(4);let placements=placed.provisional_placements();drop(placed);let operation=Operation::new(OperationId(98),RevisionId(2),Generation(1),1);let mut revalidation=FillRevalidateFixture::new(FillRevalidateJob::new(operation,fill_run_identity(),nakagin_scale_roots(),placements,1000),operation);for _ in 0..cut.as_u64().unwrap(){let _=revalidation.advance();}let job=revalidation.job.take().unwrap();let original=revalidation.original.take().unwrap();original_fill_job_close_law(job,original,FillRevalidateJob::close_demands);
 }
}
