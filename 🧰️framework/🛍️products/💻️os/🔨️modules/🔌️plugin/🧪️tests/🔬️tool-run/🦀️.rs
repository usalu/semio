// ⏯️ Support module 1 of the split test module `🔬️tool-run`: the fixtures that more than one piece uses, as real definitions the pieces import.
use semio_framework_plugin_decl_emission::{ActionMeta, AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_app::{ArtifactApp, NaturalFileDecodeCursor};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
#[cfg(test)]
use semio_framework_ui_locale::{Label, LocalizedLabel};
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_decl_views::tool_run_contract::{OriginalToolRunJobPortRetirement, ToolRunJob, ToolRunJobPort, ToolRunJobPurpose, ToolRunRetargetableJob};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_job::StepContext;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::{WindowConfigOwner, WindowConfigOwnerRegistry, bounded_window_config_preparation_factory, bounded_window_config_store_disposer, bounded_window_config_store_owners};
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_builder as builder;
use semio_framework_plugin_document_media as document_ingress;
use semio_framework_plugin_window_config as window_config;
use semio_framework_plugin_window_transient::{built_text_to_component_tree, built_to_component_tree, scene_surface};
use semio_framework_os_kernel::io;
#[cfg(test)]
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture;
use semio_framework_plugin_ownership::transient_publication::transient_store_disposer;
use semio_framework::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework::RequestId;
use semio_framework_tool_run::{JobKindId, ToolRunCounter, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunSettingsReads, ToolRunStageDefinition, ToolRunState, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceKind, ToolRunTraceSubject, ToolRunVerdict};
use semio_framework_ui_scene::{Board2dScene, World3dScene};
use store::os_store::backbone::Backbone;
use crate::tool_run_support_1::original_toy_operation::OriginalToyOperationSource;
use crate::tool_run_support_1::original_toy_unit::OriginalToyUnit;
use crate::tool_run_support_1::original_toy_emission::OriginalToyEmission;


#[doc(hidden)]
#[path="🌱️operation/🦀️.rs"]
pub mod original_toy_operation;
#[doc(hidden)]
#[path="🚦️unit/🦀️.rs"]
pub mod original_toy_unit;
#[doc(hidden)]
#[path="📬️outcome/🦀️.rs"]
pub mod original_toy_emission;

#[doc(hidden)]
pub const TOOL_RUN_FIXTURE_JSON: &str = include_str!("../🧫️fixtures/⏯️tool-run/🔣️.json");

#[doc(hidden)]
pub fn fixture() -> Value {
    let fixture: Value = serde_json::from_str(TOOL_RUN_FIXTURE_JSON).expect("tool run fixture parses");
    assert_eq!(fixture["schema"], "framework.plugin.tool-run.v1");
    fixture
}

#[doc(hidden)]
pub fn number(value: &Value) -> u64 {
    value.as_u64().unwrap_or_else(|| panic!("fixture number expected, found {value}"))
}

#[doc(hidden)]
pub fn text(value: &Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

//#region 🧸️ToyRunApp
#[doc(hidden)]
pub fn toy_definition(rebase: ToolRunRebasePolicy) -> ToolRunDefinition {
    ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
        mutating: true,
        rebase,
        reconfigure: ToolRunReconfigurePolicy::Resume,
        unit: LocalizedLabel::native("units", "Einheiten"),
        stages: vec![ToolRunStageDefinition { id: "fill".into(), label: LocalizedLabel::native("Filling", "Füllen") }],
        counters: vec![semio_framework_tool_run::ToolRunCounterDefinition { id: "resumedFrom".into(), label: LocalizedLabel::native("Resumed from", "Fortgesetzt ab") }],
        reasons: vec![
            ToolRunReasonDefinition { code: 1, id: "fits".into(), verdict: ToolRunVerdict::Success, template: LocalizedLabel::native("Fits", "Passt") },
            ToolRunReasonDefinition { code: 2, id: "conflict".into(), verdict: ToolRunVerdict::Danger, template: LocalizedLabel::native("Conflicts with the document", "Konflikt mit dem Dokument") },
        ],
        trace: ToolRunTraceKind::Entity,
        run_job: JobKindId::new("toyFill.run"),
        revalidate_job: Some(JobKindId::new("toyFill.revalidate")),
        settings: ToolRunSettingsReads { config: fixture()["settingsReads"]["config"].as_array().expect("declared config reads").iter().map(|pointer| text(pointer).to_string()).collect(), ..ToolRunSettingsReads::default() },
        windows: Vec::new(),
        member: None,
    }
}

#[doc(hidden)]
pub fn toy_target(config: &TestConfig) -> u32 {
    config.selected.as_deref().and_then(|value| value.parse().ok()).unwrap_or(number(&fixture()["defaultTarget"]) as u32)
}

#[doc(hidden)]
pub fn encoded(mutation: TestMutation) -> Vec<u8> {
    ::protocol::OpBinary::encode_op(&mutation).expect("toy op encodes")
}

/// 🧸️ One unit appends `SetCount(base + unit)` and `SetLabel("unit-<unit>")`, one entity and one `success`
/// trace record per unit; a revalidation retracts from the first unit the head no longer continues.
/// ✍️ Both jobs write through `ToolRunTickWriter::with_provisional_base`: a resumed or revalidate job starts at
/// the provisional length it continues from, so its `retract_to` below that length emits `retractTo`.
#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ToyRunJobOriginal { pub writer:ToolRunTickWriter,pub provisional_counts:Vec<i32> }
#[doc(hidden)]
pub struct ToyRunJob {
    pub unit:OriginalToyUnit,
    pub emission:OriginalToyEmission,
    pub phase:u8,
    pub batch_units:u32,
    pub scan:usize,
    pub retraction:Option<u32>,
    pub progress_source:Option<ToolRunProgress>,
    pub counter_source:Option<ToolRunCounter>,
    pub trace_source:Option<semio_framework_tool_run::ToolRunTraceOp>,
    pub payload_source:[u8;4],
    pub payload_source_ready:bool,
    pub payload_position:usize,
    pub successor:Option<ToolRunTickWriter>,
    pub writer_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ToolRunTickWriter>>,
    pub progress_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ToolRunProgress>>,
    pub purpose: ToolRunJobPurpose,
    pub original:std::mem::ManuallyDrop<ToyRunJobOriginal>,
    pub original_live:bool,
    pub original_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ToyRunJobOriginal>>,
    pub base_count: i32,
    pub target: u32,
    pub done: u32,
    pub resumed_from: u32,
    pub checkpoint_due: bool,
    pub finished: bool,
    pub closing: bool,
}

impl Drop for ToyRunJob {
    fn drop(&mut self){assert!(std::thread::panicking()||self.closing&&!self.original_live&&self.original_close.is_none(),"toy original writer/counts require granted terminal closure");}
}

impl ToyRunJob {
    pub fn retirement_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?,..Default::default()})}
    pub fn auxiliary_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
     let child=if !self.unit.terminal_is_empty(){Some(self.unit.retirement_demands()?)}else if !self.emission.terminal_is_empty(){Some(self.emission.retirement_demands()?)}else if let Some(owner)=self.writer_retirement.as_ref(){Some(Self::retirement_demand(owner,body)?)}else if let Some(owner)=self.progress_retirement.as_ref(){Some(Self::retirement_demand(owner,body)?)}else if self.successor.is_some()||self.progress_source.is_some()||self.counter_source.is_some()||self.trace_source.is_some()||self.payload_source_ready||self.payload_position>0||self.retraction.is_some()||self.phase!=0{Some(RetirementDemand{depth:1,..Default::default()})}else{None};child.map(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original Toy auxiliary depth overflow"))?;Ok(demand)}).transpose()
    }
    pub fn auxiliary_close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,ValueError>{
     use semio_framework_value::RetainedCloneStep as S;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if !self.unit.terminal_is_empty(){return self.unit.close_step(child)}if !self.emission.terminal_is_empty(){return self.emission.close_step(child)}if let Some(owner)=self.writer_retirement.as_mut(){if !owner.terminal_is_empty(){return owner.step(child)}drop(self.writer_retirement.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if let Some(owner)=self.progress_retirement.as_mut(){if !owner.terminal_is_empty(){return owner.step(child)}drop(self.progress_retirement.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.successor.is_some(){drop(self.successor.take());return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if let Some(original)=self.progress_source.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.progress_retirement=Some(owner),Err((error,original))=>{self.progress_source=Some(original);return Err(error)}}return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}if self.counter_source.take().is_some()||self.trace_source.take().is_some(){return Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}self.payload_source_ready=false;self.payload_position=0;self.retraction=None;self.phase=0;Ok(S::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    pub fn original_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(demand)=self.auxiliary_demands(body)?{return Ok(demand)}if let Some(owner)=self.original_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"toy original writer depth overflow"))?});}Ok(if self.original_live{RetirementDemand{depth:1,..Default::default()}}else{Default::default()})}

    pub fn metadata(cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,ValueError>{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;Ok(true)}
    pub fn receive(cx:&mut semio_framework_job::StepContext<'_>,result:Result<semio_framework_value::RetainedCloneStep,ValueError>)->Result<bool,semio_framework_job::OriginalTurnFailure>{match result{Ok(step)=>{cx.consume_retained(step.progress())?;Ok(matches!(step,semio_framework_value::RetainedCloneStep::Complete(_)))},Err(original)=>{Err(semio_framework_job::OriginalTurnFailure::debit_original(original,cx))}}}
    pub fn run_step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
     let grant=cx.retained_grant();match self.phase{
      0=>{if self.purpose==ToolRunJobPurpose::Revalidate{if self.scan==self.original.provisional_counts.len(){if Self::metadata(cx)?{self.finished=true;self.phase=12;}return Ok(None)}let valid=self.original.provisional_counts[self.scan]==self.base_count.checked_add(self.scan as i32+1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original revalidation count overflow"))?;if Self::metadata(cx)?{if valid{self.scan+=1;}else{self.retraction=Some((self.scan as u32).checked_mul(2).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original revalidation index overflow"))?);self.phase=13;}}return Ok(None)}
       if self.done>self.target{let result=self.original.writer.admit_original_retraction(self.target.checked_mul(2).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original retarget extent overflow"))?,grant);let complete=Self::receive(cx,result)?;if complete&&cx.retained_progress().copied_items==0&&!Self::metadata(cx)?{return Ok(None)}if complete{self.done=self.target;}return Ok(None)}
       if self.done==self.target{if self.original.writer.is_empty(){return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(semio_framework_job::OriginalTurnFailure::from)}if Self::metadata(cx)?{self.phase=1;}return Ok(None)}
       let unit=self.done.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Toy unit overflow"))?;if self.unit.advance(&mut self.original.writer,self.base_count,unit,cx)?{self.done=unit;self.batch_units+=1;if self.batch_units==64||self.done==self.target||self.original.writer.should_flush(){self.phase=1;}}}
      1=>{if Self::metadata(cx)?{self.progress_source=Some(ToolRunProgress{identity:self.original.writer.identity(),sequence:0,state:ToolRunState::Running,stage:0,completed:u64::from(self.done),total:Some(u64::from(self.target)),counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()});self.counter_source=Some(ToolRunCounter{counter:0,value:u64::from(self.resumed_from)});self.phase=2;}}
      2=>{let result=ToolRunTickWriter::admit_original_progress_counter(self.progress_source.as_mut().unwrap(),&mut self.counter_source,grant);if Self::receive(cx,result)?{self.phase=3;}}
      3=>{let result=self.original.writer.admit_original_progress(&mut self.progress_source,grant);if Self::receive(cx,result)?{self.phase=4;}}
      4=>{if !self.payload_source_ready{if grant.maximum_items>0&&grant.maximum_copy_bytes>=4&&grant.maximum_depth>0{self.payload_source=self.done.to_le_bytes();self.payload_source_ready=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:4,..Default::default()})?;}}else{let result=self.original.writer.admit_original_payload(&self.payload_source,&mut self.payload_position,grant);if Self::receive(cx,result)?{self.phase=5;}}}
      5=>{let original=self.emission.advance_preview(&self.original.writer,cx)?;if original.is_some(){self.phase=6;}return Ok(original)}
      6=>{if self.emission.close_in_context(cx)?{self.phase=7;}}
      7=>{if let Some((successor,progress))=self.original.writer.admit_original_successor(grant)?{cx.consume_retained(progress)?;self.successor=Some(successor);self.phase=8;}}
      8=>{if Self::metadata(cx)?{let original=std::mem::replace(&mut self.original.writer,self.successor.take().unwrap());match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>{self.writer_retirement=Some(owner);self.phase=9;},Err((error,original))=>{self.successor=Some(std::mem::replace(&mut self.original.writer,original));return Err(error)}}}}
      9=>{let owner=self.writer_retirement.as_mut().unwrap();if owner.terminal_is_empty(){if Self::metadata(cx)?{drop(self.writer_retirement.take());self.phase=if self.purpose==ToolRunJobPurpose::Run{10}else{12};}}else{let result=owner.step(grant);Self::receive(cx,result)?;}}
      10=>{let original=self.emission.advance_checkpoint(self.done,cx)?;if original.is_some(){self.phase=11;}return Ok(original)}
      11=>{if self.emission.close_in_context(cx)?{self.phase=0;self.batch_units=0;self.payload_position=0;self.payload_source_ready=false;}}
      12=>return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(semio_framework_job::OriginalTurnFailure::from),
      13=>{let result=self.original.writer.admit_original_retraction(self.retraction.unwrap(),grant);let done=Self::receive(cx,result)?;if done&&cx.retained_progress().copied_items==0&&!Self::metadata(cx)?{return Ok(None)}if done{self.retraction=None;self.phase=14;}}
      14=>{if Self::metadata(cx)?{if self.scan==self.original.provisional_counts.len(){self.finished=true;self.phase=5;}else{let unit=self.scan as u64+1;self.trace_source=Some(semio_framework_tool_run::ToolRunTraceOp::Upsert{key:unit,verdict:ToolRunVerdict::Danger,reason:2,subject:ToolRunTraceSubject::Entity{entity:unit}});self.phase=15;}}}
      15=>{let result=self.original.writer.admit_original_trace(&mut self.trace_source,grant);if Self::receive(cx,result)?{self.scan+=1;self.phase=14;}}
      _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Toy run phase is absent").into()),
     }Ok(None)
    }
}

impl semio_framework_job::InteractiveJob for ToyRunJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{if cx.is_cancelled(){return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}self.run_step(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}
    fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{self.emission.borrow_outcome(descriptor)}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.original_demands(0)?.depth)}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let demand=self.original_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{
        let empty=Default::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}if !self.closing||(grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth){return Ok(RetainedCloneStep::Progress(empty));}
        if self.auxiliary_demands(grant.maximum_copy_bytes).ok().flatten().is_some(){return match self.auxiliary_close_step(grant){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(original)=>Err(original)};}
        if let Some(owner)=self.original_close.as_mut(){if owner.terminal_is_empty(){drop(self.original_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return match owner.step(child).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original toy writer and counts")){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(error)=>Err(error)};}
        let original=unsafe{std::mem::ManuallyDrop::take(&mut self.original)};match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>{self.original_close=Some(owner);self.original_live=false;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))},Err((error,original))=>{self.original=std::mem::ManuallyDrop::new(original);Err(error)}}
    })
    }

    fn terminal_is_empty(&self)->bool{self.closing&&!self.original_live&&self.original_close.is_none()&&self.auxiliary_demands(0).is_ok_and(|d|d.is_none())}

}

/// 🎯️ The toy run job honours the in-place retarget hooks when it is built through
/// `build_retargetable_tool_run_job`: a rebind restamps its writer, a reconfigure adopts the new target (a lower
/// one retracts on its next step) and wakes a job that already completed.
impl ToolRunRetargetableJob<TestConfig> for ToyRunJob {
    fn rebind(&mut self, identity: ToolRunIdentity) {
        self.original.writer.rebind(identity);
    }

    fn reconfigure(&mut self, identity: ToolRunIdentity, config: std::sync::Arc<TestConfig>) -> bool {
        self.original.writer.rebind(identity);
        self.target = toy_target(&config);
        true
    }
}

/// 🗜️ A run that places `initialCount`, checkpoints, then compacts: its first compaction tick retracts everything
/// and re-appends the first count, every later tick re-appends the next, each followed by a wait on its port, and a
/// checkpoint ends the compaction — the shape of a layout run whose compaction spans several driver turns.
#[doc(hidden)]
pub struct ToyPortJobCustody {
    pub unit:OriginalToyUnit,pub emission:OriginalToyEmission,pub successor:Option<ToolRunTickWriter>,pub entity:Option<u64>,pub trace:Option<semio_framework_tool_run::ToolRunTraceOp>,pub action:Option<String>,pub effect:std::mem::ManuallyDrop<Option<Effect>>,pub action_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<String>>,
    pub writer:std::mem::ManuallyDrop<ToolRunTickWriter>,pub writer_live:bool,
    pub writer_close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ToolRunTickWriter>>,
    pub port:std::mem::ManuallyDrop<Option<ToolRunJobPort>>,
    pub port_close:Option<OriginalToolRunJobPortRetirement>,
}
impl ToyPortJobCustody {
    pub fn new(writer:ToolRunTickWriter,port:ToolRunJobPort)->Self{Self{unit:OriginalToyUnit::new(),emission:OriginalToyEmission::new(),successor:None,entity:None,trace:None,action:None,effect:std::mem::ManuallyDrop::new(None),action_close:None,writer:std::mem::ManuallyDrop::new(writer),writer_live:true,writer_close:None,port:std::mem::ManuallyDrop::new(Some(port)),port_close:None}}
    pub fn reset_writer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let grant=cx.retained_grant();if let Some(owner)=self.writer_close.as_mut(){if !owner.terminal_is_empty(){let result=owner.step(grant);ToyRunJob::receive(cx,result)?;return Ok(false)}if ToyRunJob::metadata(cx)?{drop(self.writer_close.take());return Ok(true)}return Ok(false)}if let Some(successor)=self.successor.as_ref(){let _=successor;if ToyRunJob::metadata(cx)?{let original=std::mem::replace(&mut *self.writer,self.successor.take().unwrap());match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.writer_close=Some(owner),Err((error,original))=>{self.successor=Some(std::mem::replace(&mut *self.writer,original));return Err(error.into())}}}return Ok(false)}if let Some((successor,progress))=self.writer.admit_original_successor(grant)?{cx.consume_retained(progress)?;self.successor=Some(successor);}Ok(false)}
    pub fn port(&self)->&ToolRunJobPort{self.port.as_ref().expect("live toy port")}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let auxiliary=if !self.unit.terminal_is_empty(){Some(self.unit.retirement_demands()?)}else if !self.emission.terminal_is_empty(){Some(self.emission.retirement_demands()?)}else if let Some(owner)=self.action_close.as_ref(){Some(ToyRunJob::retirement_demand(owner,body)?)}else if self.successor.is_some()||self.entity.is_some()||self.trace.is_some()||self.action.is_some()||self.effect.is_some(){Some(RetirementDemand{depth:1,..Default::default()})}else{None};if let Some(mut demand)=auxiliary{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port auxiliary depth overflow"))?;return Ok(demand)}
        if let Some(owner)=self.writer_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port writer depth overflow"))?});}
        if self.writer_live{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.port_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}let mut demand=owner.demands()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original toy port depth overflow"))?;return Ok(demand);}
        if let Some(port)=self.port.as_ref(){let mut demand=port.original_close_demands()?;demand.depth+=1;return Ok(demand);}
        Ok(Default::default())
    }
    pub fn terminal_is_empty(&self)->bool{self.unit.terminal_is_empty()&&self.emission.terminal_is_empty()&&self.successor.is_none()&&self.entity.is_none()&&self.trace.is_none()&&self.action.is_none()&&self.effect.is_none()&&self.action_close.is_none()&&!self.writer_live&&self.writer_close.is_none()&&self.port.is_none()&&self.port_close.is_none()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=Default::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}let demand=match self.demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return Err(e)};if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if !self.unit.terminal_is_empty()||!self.emission.terminal_is_empty(){let result=if !self.unit.terminal_is_empty(){self.unit.close_step(child)}else{self.emission.close_step(child)};return match result{Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(error)=>Err(error)};}if self.successor.is_some(){drop(self.successor.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
        if let Some(owner)=self.action_close.as_mut(){if owner.terminal_is_empty(){drop(self.action_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}return match owner.step(child){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(error)=>Err(error)};}if self.effect.is_some(){match self.effect.take().unwrap(){Effect::DispatchAction{action,args:None,..}=>self.action=Some(action),original=>{*self.effect=Some(original);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original Toy effect retains its missing typed retirement cursor"))}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}if let Some(original)=self.action.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.action_close=Some(owner),Err((error,original))=>{self.action=Some(original);return Err(error)}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}if self.entity.take().is_some()||self.trace.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
        if let Some(owner)=self.writer_close.as_mut(){if owner.terminal_is_empty(){drop(self.writer_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return match owner.step(child).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original toy port writer")){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(e)=>Err(e)};}
        if self.writer_live{let original=unsafe{std::mem::ManuallyDrop::take(&mut self.writer)};return match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>{self.writer_close=Some(owner);self.writer_live=false;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))},Err((e,original))=>{self.writer=std::mem::ManuallyDrop::new(original);Err(e)}};}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(owner)=self.port_close.as_mut(){if owner.terminal_is_empty(){drop(self.port_close.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}return match owner.close_step(child).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original toy port lease")){Ok(step)=>Ok(RetainedCloneStep::Progress(step.progress())),Err(e)=>Err(e)};}
        match ToolRunJobPort::admit_original_close(&mut self.port,child){Ok(Some((owner,progress)))=>{self.port_close=Some(owner);Ok(RetainedCloneStep::Progress(progress))},Ok(None)=>Ok(RetainedCloneStep::Progress(empty)),Err(e)=>Err(e)}
    }
}
impl Drop for ToyPortJobCustody{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"toy port job abandoned original writer or lease");}}
macro_rules! toy_port_job_close {
    ()=>{
        fn begin_close(&mut self){self.closing=true;}
        fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.demands(0)?.copy_bytes)}
        fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.original.demands(body)?.capacity_bytes)}
        fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.original.demands(0)?.release_bytes)}
        fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.original.demands(0)?.depth)}
        fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{if !self.closing{return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}if self.original.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}let demand=self.original.demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.original.close_step(grant))}
        fn terminal_is_empty(&self)->bool{self.closing&&self.original.terminal_is_empty()}
    };
}
#[doc(hidden)]
pub struct ToyCompactJob {pub original:ToyPortJobCustody,pub stage:usize,pub phase:u8,pub initial:i32,pub counts:[i32;3],pub closing:bool}
impl semio_framework_job::InteractiveJob for ToyCompactJob{
 fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  if cx.is_cancelled(){return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}if self.original.port().is_waiting(){return Ok(None)}let grant=cx.retained_grant();match self.phase{
   0=>{if self.stage>=4{return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(semio_framework_job::OriginalTurnFailure::from)}if self.stage==1{let result=self.original.writer.admit_original_retraction(0,grant);let complete=ToyRunJob::receive(cx,result)?;if complete&&cx.retained_progress().copied_items==0&&!ToyRunJob::metadata(cx)?{return Ok(None)}if complete{self.phase=1;}}else if ToyRunJob::metadata(cx)?{self.phase=1;}}
   1=>{let value=if self.stage==0{self.initial}else{self.counts[self.stage-1]};if self.original.unit.advance_count(&mut self.original.writer,value,cx)?{self.phase=if self.stage==0{2}else{4};}}
   2=>{if ToyRunJob::metadata(cx)?{self.original.entity=Some(1);self.phase=3;}}
   3=>{let result=self.original.writer.admit_original_entity(&mut self.original.entity,grant);if ToyRunJob::receive(cx,result)?{self.phase=4;}}
   4=>{let original=self.original.emission.advance_preview(&self.original.writer,cx)?;if original.is_some(){self.phase=5;}return Ok(original)}
   5=>{if self.original.emission.close_in_context(cx)?{self.phase=6;}}
   6=>{if self.original.reset_writer(cx)?{self.phase=if self.stage==0||self.stage==3{7}else{9};}}
   7=>{let original=self.original.emission.advance_checkpoint(0,cx)?;if original.is_some(){self.phase=8;}return Ok(original)}
   8=>{if self.original.emission.close_in_context(cx)?{self.phase=9;}}
   9=>{if ToyRunJob::metadata(cx)?{self.stage+=1;self.phase=0;if self.stage>1{self.original.port().wait();}}}
   _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original compact phase absent").into()),
  }Ok(None)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{self.original.emission.borrow_outcome(descriptor)}
 toy_port_job_close!();
}

thread_local! {
    /// 🪟️ `(tool id, window id, window config present)` of every job request the toy app received on this thread.
    pub static TOY_REQUEST_WINDOWS: std::cell::RefCell<Vec<(String, Option<String>, bool)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 📮️ A job whose every unit is an external round trip: it hands the host one `DispatchAction` through its
/// [`ToolRunJobPort`], waits, and counts the hop answered when the port is woken — the shape of a run whose
/// algorithm units live in another component.
#[doc(hidden)]
pub struct ToyWaitJob {pub original:ToyPortJobCustody,pub hops:u64,pub dispatched:u64,pub answered:u64,pub phase:u8,pub action_position:usize,pub closing:bool}
impl semio_framework_job::InteractiveJob for ToyWaitJob{
 fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  if cx.is_cancelled(){return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}let grant=cx.retained_grant();match self.phase{
   0=>{if self.dispatched>self.answered{if ToyRunJob::metadata(cx)?{let answered=self.dispatched;self.original.trace=Some(semio_framework_tool_run::ToolRunTraceOp::Upsert{key:answered,verdict:ToolRunVerdict::Success,reason:1,subject:ToolRunTraceSubject::Entity{entity:answered}});self.phase=1;}}else if self.dispatched==self.hops{return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(semio_framework_job::OriginalTurnFailure::from)}else if ToyRunJob::metadata(cx)?{self.original.action=Some(String::new());self.action_position=0;self.phase=5;}}
   1=>{let result=self.original.writer.admit_original_trace(&mut self.original.trace,grant);if ToyRunJob::receive(cx,result)?{self.phase=2;}}
   2=>{let original=self.original.emission.advance_preview(&self.original.writer,cx)?;if original.is_some(){self.phase=3;}return Ok(original)}
   3=>{if self.original.emission.close_in_context(cx)?{self.phase=4;}}
   4=>{if self.original.reset_writer(cx)?{self.answered=self.dispatched;self.phase=0;}}
   5=>{const ACTION:&str="toyExternalHop";let source=self.original.action.as_mut().unwrap();if source.capacity()==0{if grant.maximum_items>0&&grant.maximum_capacity_bytes>=ACTION.len()&&grant.maximum_depth>0{source.try_reserve_exact(ACTION.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Toy action backing unavailable"))?;if source.capacity()!=ACTION.len(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original Toy action native extent changed").with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:source.capacity(),..Default::default()}).into())}cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:source.capacity(),..Default::default()})?;}}else if self.action_position<ACTION.len(){let count=grant.maximum_copy_bytes.min(ACTION.len()-self.action_position);if grant.maximum_items>0&&count>0&&grant.maximum_depth>0{source.push_str(&ACTION[self.action_position..self.action_position+count]);self.action_position+=count;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;}}else if ToyRunJob::metadata(cx)?{let request=self.dispatched.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original Toy request overflow"))?;*self.original.effect=Some(Effect::DispatchAction{req:RequestId(request),action:self.original.action.take().unwrap(),args:None,delay_ms:0});self.phase=6;}}
   6=>{let result=self.original.port.as_ref().unwrap().admit_original_dispatch(&mut self.original.effect,grant);if ToyRunJob::receive(cx,result)?{self.phase=7;}}
   7=>{if ToyRunJob::metadata(cx)?{self.dispatched+=1;self.phase=0;self.original.port().wait();}}
   _=>return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original wait phase absent").into()),
  }Ok(None)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{self.original.emission.borrow_outcome(descriptor)}
 toy_port_job_close!();
}

/// 🧸️ The toy run or revalidate job a request describes: it continues from the request's checkpoint.
#[doc(hidden)]
pub fn toy_run_job(request: ToolRunJobRequest<'_, ToyRunApp>) -> ToyRunJob {
    let done = request.checkpoint.and_then(|bytes| bytes.try_into().ok()).map_or(0, u32::from_le_bytes);
    let provisional_counts = request.provisional.iter().filter_map(|op| match op {
        TestMutation::SetCount(set) => Some(set.value),
        TestMutation::SetLabel(_) | TestMutation::SetSlotChildren(_) => None,
    });
    ToyRunJob {
        unit:OriginalToyUnit::new(),emission:OriginalToyEmission::new(),phase:0,batch_units:0,scan:0,retraction:None,progress_source:None,counter_source:None,trace_source:None,payload_source:[0;4],payload_source_ready:false,payload_position:0,successor:None,writer_retirement:None,progress_retirement:None,
        purpose: request.purpose,
        base_count: request.snapshot.count,
        target: toy_target(&request.config),
        done,
        resumed_from: done,
        original:std::mem::ManuallyDrop::new(ToyRunJobOriginal{writer:ToolRunTickWriter::with_provisional_base(request.identity,request.provisional.len()as u32),provisional_counts: match request.purpose {
            ToolRunJobPurpose::Run => Vec::new(),
            ToolRunJobPurpose::Revalidate => provisional_counts.collect(),
        },
}),original_live:true,original_close:None,
        checkpoint_due: false,
        finished: false,
        closing: false,
    }
}

/// 🪟️ The toy world window's config: the same one-field document as the app config.
#[doc(hidden)]
pub struct ToyWorldWindowConfig;

impl WindowConfigOwner for ToyWorldWindowConfig {
    const WINDOW_KIND_ID: &'static str = "world";
    const SCHEMA: &'static str = "semio.testkit-tool-run.world-window-config/v1";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = TestConfig;
    type Mutation = TestConfigMutation;
    type Edit=semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::preparation::SelectionRetainedEdit;
    const MAXIMUM_PREPARATION_DEPTH:usize=64;
    fn build_retained_edit()->std::sync::Arc<Self::Edit>{std::sync::Arc::new(semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::preparation::SelectionRetainedEdit)}

    fn build_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> {
        semio_framework_plugin_window_config::bounded_window_config_store_owners::<Self>()
    }

    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin_window_config::bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin_window_config::bounded_window_config_store_disposer::<Self>()
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct ToyRunApp;

impl ArtifactApp for ToyRunApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.test.tool-run", standard: StandardId("1"), subset: SubsetId::ANY };
    const APP_ID: &'static str = "s.test.tool-run@1/*#editor";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-tool-run/v1";
    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TestMutation;

    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<ToyWorldWindowConfig>()
    }

    fn build_retargetable_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<Box<dyn ToolRunRetargetableJob<TestConfig>>>, Fault> {
        if request.tool_id != text(&fixture()["retarget"]["toolId"]) {
            return Ok(None);
        }
        Ok(Some(Box::new(toy_run_job(request))))
    }

    fn build_tool_run_job(request: ToolRunJobRequest<'_, Self>) -> Result<Option<ToolRunJob>, Fault> {
        TOY_REQUEST_WINDOWS.with(|windows| windows.borrow_mut().push((request.tool_id.to_string(), request.window_id.map(str::to_string), request.window_config.is_some())));
        if request.tool_id == text(&fixture()["member"]["toolId"]) {
            return Ok(Some(Box::new(member::toy_member_job(request)?)));
        }
        if request.tool_id == text(&fixture()["compact"]["toolId"]) {
            return Ok(Some(Box::new(ToyCompactJob { original:ToyPortJobCustody::new(ToolRunTickWriter::with_provisional_base(request.identity,request.provisional.len()as u32),request.port),stage:0,phase:0,initial:number(&fixture()["compact"]["initialCount"])as i32,counts:std::array::from_fn(|index|number(&fixture()["compact"]["compactCounts"][index])as i32),closing:false })));
        }
        if request.tool_id == text(&fixture()["port"]["toolId"]) || request.tool_id == text(&fixture()["concurrentReadOnly"]["readOnlyToolId"]) {
            request.instance_owner.with_mut::<EmptyArtifactInstanceOperationOwner, _>(|_| Ok(()))?;
            return Ok(Some(Box::new(ToyWaitJob { original:ToyPortJobCustody::new(ToolRunTickWriter::new(request.identity),request.port),hops: number(&fixture()["port"]["hops"]), dispatched: 0, answered:0,phase:0,action_position:0,closing:false })));
        }
        Ok(Some(Box::new(toy_run_job(request))))
    }

    async fn initial_snapshot() -> TestSnapshot {
        TestSnapshot::default()
    }

    async fn handle(
        command: &TestMutation,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, NoDraftMutation> {
        Ok(Emit { artifact_mutations: vec![command.clone()], ..Default::default() })
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        if body_key == text(&fixture()["boardLane"]["bodyKey"]) {
            let scene = Board2dScene::base(format!("{{\"count\":{}}}", doc.snapshot.count), "{}".into(), true);
            let surface = scene_surface(text(&fixture()["boardLane"]["surfaceId"]), SurfaceKind::Board2d, &scene)?;
            return column()
                .try_id("board")
                .map_err(|_| PluginAssemblyError::new("toy", "board id"))?
                .try_child(surface)
                .map_err(|_| PluginAssemblyError::new("toy", "board child"))?
                .try_build()
                .map(built_to_component_tree)
                .map_err(|_| PluginAssemblyError::new("toy", "board build"));
        }
        if body_key != text(&fixture()["traceLane"]["bodyKey"]) {
            let run = doc
                .tool_run()
                .map_or_else(String::new, |run| format!(" completed={} steps={} payload={}", run.progress.completed, run.progress.steps.len(), run.payload.as_deref().and_then(|payload| payload.as_slice().try_into().ok()).map_or(0, u32::from_le_bytes)));
            return built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("count={}{run}", doc.snapshot.count)));
        }
        let provisional = |unit: i32| doc.tool_run().is_some_and(|run| run.provisional_entities.contains(&(unit as u64)));
        let instances: Vec<Value> = (1..=doc.snapshot.count).map(|unit| serde_json::json!({ "id": format!("unit-{unit}"), "meshId": "unit", "provisional": provisional(unit) })).collect();
        let scene = World3dScene::base("{}".into(), "[]".into(), Value::Array(instances).to_string(), semio_framework_ui_scene::world3d_default_selection_json());
        let surface = scene_surface(text(&fixture()["traceLane"]["surfaceId"]), SurfaceKind::World3d, &scene)?;
        column()
            .try_id("world")
            .map_err(|_| PluginAssemblyError::new("toy", "world id"))?
            .try_child(surface)
            .map_err(|_| PluginAssemblyError::new("toy", "world child"))?
            .try_build()
            .map(built_to_component_tree)
            .map_err(|_| PluginAssemblyError::new("toy", "world build"))
    }

    fn build_document_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>(grant))
    }

    fn build_config_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Config, Self::ConfigMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Config, Self::ConfigMutation>>> {
        Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>(grant))
    }

    fn build_draft_store_owners(grant: RetainedCloneGrant) -> Option<Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::Draft, Self::DraftMutation>, RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Draft, Self::DraftMutation>>> {
        Some(bounded_document_store_owners::<Self::Draft, Self::DraftMutation>(grant))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(store::snapshot_clone_preparation::config_apply_preparation_factory::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(mutation_fixture::no_state::presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(mutation_fixture::no_state::transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(mutation_fixture::no_state::presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(mutation_fixture::no_state::transient_local_root_retirement_factory())
    }
}
//#endregion 🧸️ToyRunApp

#[doc(hidden)]
pub type ToyApp = VcsArtifactApp<ToyRunApp>;

/// 🛂️ The toy manifest declares both tools with their `ToolRunDefinition` — the run's source of record.
///
/// 📖️ The READ-ONLY twin of the toy run: the same job, declared as publishing nothing.
#[doc(hidden)]
pub async fn toy_manifest() -> App {
    let fixture = fixture();
    let mut builder = App::builder(ToyRunApp::APP_ID, LocalizedLabel::data("Tool Run Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "tool-run.main", SurfaceKind::Canvas2d, IconName::AppWindow)
        .await
        .window_kind(text(&fixture["traceLane"]["windowId"]), LocalizedLabel::data("World"), text(&fixture["traceLane"]["bodyKey"]), SurfaceKind::World3d, IconName::AppWindow)
        .await
        .window_kind(text(&fixture["boardLane"]["windowId"]), LocalizedLabel::data("Board"), text(&fixture["boardLane"]["bodyKey"]), SurfaceKind::Board2d, IconName::AppWindow)
        .await;
    let mut tools = Vec::new();
    for (id, rebase) in [
        (text(&fixture["toolId"]), ToolRunRebasePolicy::Revalidate),
        (text(&fixture["freezeToolId"]), ToolRunRebasePolicy::Freeze),
        (text(&fixture["port"]["toolId"]), ToolRunRebasePolicy::Restart),
        (text(&fixture["retarget"]["toolId"]), ToolRunRebasePolicy::Revalidate),
        (text(&fixture["compact"]["toolId"]), ToolRunRebasePolicy::Revalidate),
    ] {
        builder = builder.tool(ToolDefinition { run: Some(toy_definition(rebase)), ..ToolDefinition::new(id, LocalizedLabel::native("Toy fill", "Spielfüllung"), IconName::PaintBucket).await }).await;
        tools.push(ToolRef::new(id).await);
    }
    let window_settings = &fixture["windowSettingsReads"];
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition {
                settings: ToolRunSettingsReads { window_config: [(text(&window_settings["windowKindId"]).to_string(), vec![text(&window_settings["pointer"]).to_string()])].into_iter().collect(), ..ToolRunSettingsReads::default() },
                ..toy_definition(ToolRunRebasePolicy::Revalidate)
            }),
            ..ToolDefinition::new(text(&window_settings["toolId"]), LocalizedLabel::native("Toy fill reading window settings", "Spielfüllung mit Fenstereinstellungen"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(text(&window_settings["toolId"])).await);
    let read_only_wait = text(&fixture["concurrentReadOnly"]["readOnlyToolId"]);
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { mutating: false, ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(read_only_wait, LocalizedLabel::native("Toy read-only hops", "Schreibgeschützte Spielsprünge"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(read_only_wait).await);
    let reader = &fixture["readerWindows"];
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { windows: reader["windows"].as_array().expect("reader windows").iter().map(|window| text(window).to_string()).collect(), ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(text(&reader["toolId"]), LocalizedLabel::native("Toy fill with a reader window", "Spielfüllung mit Lesefenster"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(text(&reader["toolId"])).await);
    let undeclared = text(&fixture["settingsReads"]["undeclaredToolId"]);
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { settings: ToolRunSettingsReads::default(), ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(undeclared, LocalizedLabel::native("Toy fill without settings reads", "Spielfüllung ohne Einstellungen"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(undeclared).await);
    let read_only = text(&fixture["readOnlyRun"]["toolId"]);
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { mutating: false, ..toy_definition(ToolRunRebasePolicy::Revalidate) }),
            ..ToolDefinition::new(read_only, LocalizedLabel::native("Toy read-only fill", "Schreibgeschützte Spielfüllung"), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(read_only).await);
    let member = &fixture["member"];
    builder = builder
        .tool(ToolDefinition {
            run: Some(ToolRunDefinition { rebase: ToolRunRebasePolicy::Restart, revalidate_job: None, member: Some(text(&member["slot"]).to_string()), ..toy_definition(ToolRunRebasePolicy::Restart) }),
            ..ToolDefinition::new(text(&member["toolId"]), LocalizedLabel::native(text(&member["toolLabel"]["en"]), text(&member["toolLabel"]["de"])), IconName::PaintBucket).await
        })
        .await;
    tools.push(ToolRef::new(text(&member["toolId"])).await);
    App::from_builder(builder.mode_tools("edit", tools).await).await
}

/// 🎟️ Captures the unchanged explicit neutral caller policy before any app admission.
#[doc(hidden)]
pub fn toy_fixture_caller_policy()->(semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,usize){let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json")).unwrap();let g=&law["native"]["grant"];let grant=RetainedCloneGrant{maximum_items:number(&g[0])as usize,maximum_copy_bytes:number(&g[1])as usize,maximum_capacity_bytes:number(&g[2])as usize,maximum_release_bytes:number(&g[3])as usize,maximum_depth:number(&g[4])as usize};(semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:grant,maintenance:grant,close:grant}.validate().unwrap(),number(&law["native"]["maximumIdentityBytes"])as usize)}

#[doc(hidden)]
pub struct ToyFixtureCaller<'a,'context,'original>{pub context:&'context mut semio_framework_job::StepContext<'original>,pub policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,pub identity:semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'a>,pub receipts:Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt>}
impl ToyFixtureCaller<'_,'_,'_>{
    pub fn retain_original_receipts<P:PluginApp>(&mut self,app:&mut P){
        if let Some((kind,phase,grant,progress))=app.take_constructor_receipt(){let expected=match phase{semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Preparation=>self.policy.preparation,semio_framework_plugin_operation::original_catalog_constructor::ArtifactStoreConstructorPhase::Close=>self.policy.close};assert_eq!(grant,expected);assert!(progress.fits(grant));assert!(grant.maximum_depth>0||progress==Default::default());self.receipts.push(artifact_app_laws::TypedOperationFixtureOwnershipReceipt::Constructor{kind,phase,grant,progress});}
        if let Some((operation,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,self.policy.preparation);assert!(progress.fits(grant));assert!(grant.maximum_depth>0||progress==Default::default());self.receipts.push(artifact_app_laws::TypedOperationFixtureOwnershipReceipt::Preparation{operation,grant,progress});}
        if let Some((operation,grant,progress))=app.take_typed_operation_retirement_receipt(){assert_eq!(grant,self.policy.close);assert!(progress.fits(grant));assert!(grant.maximum_depth>0||progress==Default::default());self.receipts.push(artifact_app_laws::TypedOperationFixtureOwnershipReceipt::Retirement{operation,grant,progress});}
        if let Some((grant,progress))=app.take_history_command_receipt(){assert_eq!(grant,self.policy.maintenance);assert!(progress.fits(grant));assert!(grant.maximum_depth>0||progress==Default::default());self.receipts.push(artifact_app_laws::TypedOperationFixtureOwnershipReceipt::History{grant,progress});}
        if let Some((grant,progress))=app.take_tool_run_receipt(){assert_eq!(grant,self.policy.maintenance);assert!(progress.fits(grant));assert!(grant.maximum_depth>0||progress==Default::default());self.receipts.push(artifact_app_laws::TypedOperationFixtureOwnershipReceipt::ToolRun{grant,progress});}
    }
}
#[doc(hidden)]
pub async fn advance_toy<P:PluginApp>(app:&mut P,caller:&mut ToyFixtureCaller<'_,'_,'_>)->Result<(),semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{caller.retain_original_receipts(app);let result=app.advance_typed_operation_publication(&mut caller.identity,caller.context).await;caller.retain_original_receipts(app);result}

#[doc(hidden)]
pub async fn toy_app(target: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> ToyApp {
    let mut app = artifact_app_laws::new_registered_app::<ToyRunApp, _>(toy_manifest(), protocol::ActorId(text(&fixture()["actor"]).into()), caller.policy, &mut caller.identity).await;
    set_target(&mut app, target, caller).await;
    app
}

#[doc(hidden)]
pub async fn set_target(app: &mut ToyApp, target: u64, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    app.stores.config_store.dispatch(ArtifactCommand::Apply { mutations: vec![ChangeTestConfigSelection { selected: Some(target.to_string()) }.into()], transaction: None }, &mut caller.identity).await.expect("toy target config applies");
}

#[doc(hidden)]
pub fn toy_meta() -> ActionMeta {
    artifact_app_laws::meta(text(&fixture()["actor"]))
}

#[doc(hidden)]
pub async fn tool_run_action(app: &mut ToyApp, action: &str, arguments: Vec<(String, DslValue)>, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> DslValue {
    let mut source=Some(DslValue::Object(arguments));
    for _ in 0..semio_framework_plugin_testing_caller::OriginalFixtureCaller::maximum_turns(){if let Some(result)=app.handle_action(action,&mut source,&toy_meta(),&mut caller.identity,caller.context).await.unwrap_or_else(|fault|panic!("{action} dispatch: {fault:?}")){return result.output}}
    panic!("{action} did not settle under the original fixture caller")
}

#[doc(hidden)]
pub async fn run_action(app: &mut ToyApp, action: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) -> DslValue {
    let arguments = run_arguments(app);
    tool_run_action(app, action, arguments, caller).await
}

#[doc(hidden)]
pub fn run_arguments(app: &ToyApp) -> Vec<(String, DslValue)> {
    let slot = app.tool_runs.slot().expect("a run slot exists");
    vec![("runId".into(), DslValue::String(slot.run.to_string())), ("generation".into(), DslValue::String(slot.generation.to_string()))]
}

#[doc(hidden)]
pub async fn start(app: &mut ToyApp, tool_id: &str, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let output = tool_run_action(app, "toolRunStart", vec![("toolId".into(), DslValue::String(tool_id.into()))], caller).await;
    assert_eq!(output.get("toolRun").and_then(DslValue::as_str), Some("spawnJob"), "start spawns the run job");
}

#[doc(hidden)]
pub async fn pump_until(app: &mut ToyApp, what: &str, done: impl Fn(&ToyApp) -> bool, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while std::time::Instant::now() < deadline {
        if done(app) {
            return;
        }
        advance_toy(app, caller).await.unwrap_or_else(|fault| panic!("{what}: driver turn faulted: {fault:?}"));
    }
    panic!("{what} never settled; state {:?}", app.tool_runs.state());
}

#[doc(hidden)]
pub fn close(app: &mut ToyApp, caller:&mut ToyFixtureCaller<'_,'_,'_>) {
    caller.retain_original_receipts(app);
    artifact_app_laws::close_registered_fixture_app(app,caller.policy,caller.context).expect("original tool fixture paid terminal close");
    assert!(app.tool_runs.terminal_is_empty(), "the tool run ledger retires every owner on close");
}