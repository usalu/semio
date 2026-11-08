//! 🪁️ The instance components of `instance-jobs`: PendingReserved, ToolOperations (WP6, plan section 7.3).
use semio_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, ArtifactFixedQueue, ArtifactFixedRegistry};
use semio_plugin_window_kits::AppActionRegistry;
use semio_framework::kernel::{AppEvent, Effect, InvocationResult, UiDirtyScope};
use semio_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
use semio_plugin_decl_tool_contract::ArtifactInstanceOperationOwnerHandle;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_app::tool_contract::{ArtifactToolPublicContract, ArtifactToolRegistration};
use std::collections::BTreeMap;
use semio_ui_runtime::ComponentTree;
use semio_plugin_composition_publication::ComposedGestureResult;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use crate::jobs_document_replacement::DocumentWindowResetAdmission;
use semio_framework_value::DslValue;
use semio_plugin_decl_emission::Emit;
use semio_plugin_job::{EmitWire, TYPED_OPERATION_HOST_OUTBOX_SLOTS, TypedOperationCompletionWitness, TypedOperationLeftover};
use semio_framework::{Fault, FaultFrom};
use semio_plugin_job::reserved_jobs::{FrameworkReservedCommit, FrameworkReservedCommitPermit, PendingFrameworkReserved, QualifiedBoundedFirstStepProof};
use std::future::Future;
use std::mem::ManuallyDrop;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_retained_command::typed_operation::MountedTypedCommandFullOperation;
use protocol::OpText;
use crate::jobs_instance_root::OriginalRenderTreeIdentity;
use crate::jobs_lease::OriginalToolLeaseAdmission;
use semio_plugin_command::command_admission::{PendingLatestWinsCommand, ToolLatestWinsRegistry};
use semio_plugin_reactor_app::PluginApp;
use semio_ui_contract::*;

/// 🪂️ Pending reserved work: the reserved commits, render trees, configuration commands and their failure custody.
pub struct PendingReserved<A: ArtifactApp> {
    /// 🧾 Last Emit op packs produced by `dispatch_emit` — consumed by `AppCommand::PureCommand`.
    pub last_emit_wire: Option<EmitWire>,
    /// 📨️ The authoritative outcome of the current app command, reset before dispatch and then
    /// filled from the store receipt or policy rejection before the runtime frames it.
    pub dispatch_report: protocol::DispatchReport,
    pub pending_render_tree: std::mem::ManuallyDrop<Option<ComponentTree>>,
    pub pending_render_tree_identity: Option<OriginalRenderTreeIdentity>,
    pub pending_render_work: ManuallyDrop<Option<crate::jobs_render::work::OriginalRenderWork<A::RenderWork>>>,
    pub pending_render_source: ManuallyDrop<Option<crate::jobs_render::work::OriginalRenderWork<std::sync::Arc<A::Snapshot>>>>,
    pub pending_render_tree_ready: bool,
    pub pending_render_tree_close:std::mem::ManuallyDrop<Option<semio_ui_contract::OriginalBuiltTreeRetirement>>,
    pub pending_configuration_source:Option<semio_framework::action_bus::OriginalToolSourceBinding>,
    pub pending_configuration_rejected:bool,
    pub pending_configuration_worker:Option<semio_plugin_job::reserved_jobs::configuration_worker::ConfigurationWorker<A>>,
    pub pending_configuration_completion:Option<semio_plugin_job::reserved_jobs::ConfigurationDecodeRetirement<A::ConfigMutation>>,
    pub pending_configuration_command:std::mem::ManuallyDrop<Option<ArtifactCommand<A::ConfigMutation>>>,
    pub pending_configuration_command_close:Option<ControlledRetirement<ArtifactCommand<A::ConfigMutation>>>,
    pub pending_configuration_permit:Option<FrameworkReservedCommitPermit>,
    pub pending_configuration_failure:std::mem::ManuallyDrop<Option<semio_plugin_job::reserved_jobs::OriginalConfigurationFailure>>,
    pub pending_configuration_identity:Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub pending_configuration_retirement:Option<ControlledRetirement<semio_plugin_job::reserved_jobs::OriginalConfigurationFailure>>,
    pub pending_render_failure: std::mem::ManuallyDrop<Option<semio_plugin_ownership::sdk_fault::OriginalRenderFailure>>,
    pub pending_render_identity: Option<(semio_framework_job::OperationId,semio_framework_job::Generation)>,
    pub pending_render_close_failure:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalTurnFailure>>,
    pub pending_render_compound:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalClosureFailure>>,
    pub pending_render_failure_place:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<semio_plugin_ownership::sdk_fault::OriginalRenderFailure>>>,
    pub pending_render_compound_place:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<semio_framework_job::OriginalClosureFailure>>>,
    pub pending_configuration_failure_place:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalDomainReservation<semio_plugin_job::reserved_jobs::OriginalConfigurationFailure>>>,
    pub pending_render_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_plugin_ownership::sdk_fault::OriginalRenderFailure>>>,
    pub pending_reserved: ArtifactFixedRegistry<PendingFrameworkReserved>,
    pub reserved_commits: std::collections::VecDeque<FrameworkReservedCommit<A::Snapshot,A::Mutation>>,
    pub reserved_commit_outcome: Option<Result<InvocationResult, Fault>>,
    pub pending_document_window_admission:std::mem::ManuallyDrop<Option<DocumentWindowResetAdmission>>,
    pub reserved_preparation:ManuallyDrop<Option<crate::reserved_preparation::ReservedPreparation<A>>>,
}

impl<A: ArtifactApp> PendingReserved<A> {
    /// 📏️ Quotes this component frame and its fixed original backing; input roots remain separately admitted.
    pub fn birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_plugin_ownership::bounded_ownership::original_component_birth::<Self>(&[semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.pending_reserved,64)?,semio_plugin_ownership::bounded_ownership::original_deque_backing::<Self,_>(|owner|&owner.reserved_commits,64)?])}

    /// 🛟️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
            /// 📋️ Enabled fields form one retained initialization inventory.
    pub const INITIAL_FIELDS:usize=31;
    /// 📏️ Quotes one bounded absent chunk or one atomic live field and its actual backing.
    pub fn initial_field_demands(field:usize,byte:usize,body:usize,backing:bool)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(match field{
  0=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.last_emit_wire);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  1=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.dispatch_report)+std::mem::size_of::<Option<protocol::DispatchReport>>(),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  2=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_tree);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  3=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_tree_identity);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  4=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_work);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  5=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_source);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  6=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.pending_render_tree_ready),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  7=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_tree_close);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  8=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_source);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  9=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_rejected),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  10=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_worker);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  11=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_completion);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  12=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_command);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  13=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_command_close);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  14=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_permit);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  15=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_failure);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  16=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_identity);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  17=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_retirement);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  18=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_failure);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  19=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_identity);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  20=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_close_failure);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  21=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_compound);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  22=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_failure_place);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  23=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_compound_place);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  24=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_failure_place);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  25=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_render_retirement);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  26=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.pending_reserved),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.pending_reserved,64)?}else{0},depth:1,..Default::default()},
  27=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.reserved_commits),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_deque_backing::<Self,_>(|owner|&owner.reserved_commits,ARTIFACT_LIVE_OUTPUT_SLOTS)?}else{0},depth:1,..Default::default()},
  28=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.reserved_commit_outcome);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  29=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_document_window_admission);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  30=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.reserved_preparation);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  _=>return Err(semio_framework_value::ValueError::invariant("initial field cursor exceeds its inventory")),
 })}
    /// 🖋️ Copies one funded static absent chunk or initializes one selected live field in place.
    /// # Safety
    /// The target is exclusive caller-funded storage; the retained cursor advances only after successful entry.
    pub unsafe fn initialize_field_at(target:*mut Self,report:&mut Option<protocol::DispatchReport>,field:usize,byte:usize,count:usize,backing:bool)->Result<bool,semio_framework_value::ValueError>{match field{
  0=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).last_emit_wire),byte,count)},
  1=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.dispatch_report)+std::mem::size_of::<Option<protocol::DispatchReport>>(){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).dispatch_report).write(report.take().ok_or_else(||semio_framework_value::ValueError::invariant("native dispatch report absent"))?);}Ok(true)},
  2=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_tree),byte,count)},
  3=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_render_tree_identity),byte,count)},
  4=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_work),byte,count)},
  5=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_source),byte,count)},
  6=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.pending_render_tree_ready){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).pending_render_tree_ready).write(false);}Ok(true)},
  7=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_tree_close),byte,count)},
  8=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_source),byte,count)},
  9=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.pending_configuration_rejected){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).pending_configuration_rejected).write(false);}Ok(true)},
  10=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_worker),byte,count)},
  11=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_completion),byte,count)},
  12=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_configuration_command),byte,count)},
  13=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_command_close),byte,count)},
  14=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_permit),byte,count)},
  15=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_configuration_failure),byte,count)},
  16=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_identity),byte,count)},
  17=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_configuration_retirement),byte,count)},
  18=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_failure),byte,count)},
  19=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).pending_render_identity),byte,count)},
  20=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_close_failure),byte,count)},
  21=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_compound),byte,count)},
  22=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_failure_place),byte,count)},
  23=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_compound_place),byte,count)},
  24=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_configuration_failure_place),byte,count)},
  25=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_render_retirement),byte,count)},
  26=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.pending_reserved){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).pending_reserved).write(if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(ARTIFACT_LIVE_OUTPUT_SLOTS)});}Ok(true)},
  27=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.reserved_commits){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).reserved_commits).write(if backing{std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS)}else{std::collections::VecDeque::new()});}Ok(true)},
  28=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).reserved_commit_outcome),byte,count)},
  29=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_document_window_admission),byte,count)},
  30=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).reserved_preparation),byte,count)},
  _=>Err(semio_framework_value::ValueError::invariant("initial field cursor exceeds its inventory")),
 }}

pub fn new(dispatch_report:protocol::DispatchReport)->Self{let backing=true;Self{reserved_preparation:ManuallyDrop::new(None),last_emit_wire:None,dispatch_report:dispatch_report,pending_render_tree:std::mem::ManuallyDrop::new(None),pending_render_tree_identity:None,pending_render_work:ManuallyDrop::new(None),pending_render_source:ManuallyDrop::new(None),pending_render_tree_ready:false,pending_render_tree_close:std::mem::ManuallyDrop::new(None),pending_configuration_source:None,pending_configuration_rejected:false,pending_configuration_worker:None,pending_configuration_completion:None,pending_configuration_command:std::mem::ManuallyDrop::new(None),pending_configuration_command_close:None,pending_configuration_permit:None,pending_configuration_failure:std::mem::ManuallyDrop::new(None),pending_configuration_identity:None,pending_configuration_retirement:None,pending_render_failure:std::mem::ManuallyDrop::new(None),pending_render_identity:None,pending_render_close_failure:std::mem::ManuallyDrop::new(None),pending_render_compound:std::mem::ManuallyDrop::new(None),pending_render_failure_place:std::mem::ManuallyDrop::new(None),pending_render_compound_place:std::mem::ManuallyDrop::new(None),pending_configuration_failure_place:std::mem::ManuallyDrop::new(None),pending_render_retirement:std::mem::ManuallyDrop::new(None),pending_reserved:if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(ARTIFACT_LIVE_OUTPUT_SLOTS)},reserved_commits:if backing{std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS)}else{std::collections::VecDeque::new()},reserved_commit_outcome:None,pending_document_window_admission:std::mem::ManuallyDrop::new(None)}}

    pub fn original_render_tree_source<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<&'a mut Option<semio_ui_runtime::ComponentTree>,semio_framework_job::OriginalTurnFailure>{
        self.pending_render_tree_identity.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source has no initiating identity"))?.validate(cx)?;
        if !self.pending_render_tree_ready || self.pending_render_work.is_some() || self.pending_render_source.is_some() || self.pending_render_failure.is_some() || self.pending_render_failure_place.is_some() || self.pending_render_compound.is_some() || self.pending_render_compound_place.is_some() || self.pending_render_close_failure.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source is not ready for donation").into());}
        Ok(&mut self.pending_render_tree)
    }
}

/// 🪐️ Tool operations: typed operation outboxes, tool jobs and registrations, cancellations and latest-wins commands.
pub struct ToolOperations<A: ArtifactApp> {
    pub pending_tool_lease_admission:ManuallyDrop<Option<OriginalToolLeaseAdmission>>,
    pub original_tool_follow:ManuallyDrop<Option<Box<semio_plugin_tool_machine::OriginalToolMachineFollow<A>>>>,
    pub original_tool_tag_retirement:ManuallyDrop<Option<Box<semio_plugin_scrub_host::OriginalToolTagRetirement>>>,
    pub original_tool_tag_capture:ManuallyDrop<Option<Box<semio_plugin_tool_machine::OriginalToolTagCapture>>>,
    pub original_scrub_dispatch:ManuallyDrop<Option<Box<semio_plugin_scrub_host::OriginalScrubDispatch<semio_plugin_tool_machine::PressLeaf<A::Mutation,A::ConfigMutation>>>>>,
    pub original_press_settlement:ManuallyDrop<Option<Box<semio_plugin_tool_machine::OriginalPressSettlement<A>>>>,
    pub original_typing_settlement:ManuallyDrop<Option<Box<semio_plugin_typing_edit::OriginalTypingSettlement<A>>>>,
    pub typed_effect_outbox: ArtifactFixedQueue<Effect>,
    pub typed_event_outbox: ArtifactFixedQueue<AppEvent>,
    pub typed_ui_outbox: ArtifactFixedQueue<UiDirtyScope>,
    pub operation_progress_retired: bool,
    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — per mounted typed operation (keyed by
    /// its operation id), the guest-emitted interaction verbs (`Effect::ReplayShellCommand`/
    /// `DispatchAction` with an `INTERACTION_ACTION_IDS` action, see [`is_inline_interaction_verb`])
    /// peeled out of its emit the INSTANT the ladder takes the completion
    /// (`publish_mounted_typed_operation_unit`'s install step) — before any effect can be popped
    /// into `typed_effect_outbox` — and parked until `mounted_typed_inline_interaction_verbs_are_next`
    /// folds them in-reactor. The host never sees one: a verb that reached the effect outbox on
    /// the install pump was re-dispatched by the shell as a second `onAction` (18
    /// `replayShellCommand dispatch interactionSelect` lines per battery on the fem2d lane),
    /// behind any pointer input issued meanwhile. Bounded by the live operation slots; an entry
    /// is consumed by the fold or dropped with its operation's exact removal.
    pub typed_inline_interaction_verbs: Vec<(u64, Vec<(String, Option<DslValue>)>)>,
    /// 🗜️ The LAST folded verb's leftover `InteractionView` (`AppliedInteractionVerb::leftover`),
    /// one slot, latest wins (a later verb's snapshot already includes the earlier ones — the
    /// same rule `InlineInteractionFold::leftover` follows). It rides the next `Ui` progress
    /// frame's `output` (`take_typed_operation_ui_progress`) — the typed ladder's twin of the
    /// `Invocation.output.interactionView` lane a host-dispatched verb answers on.
    pub typed_inline_interaction_leftover: Option<TypedOperationLeftover>,
    pub typed_completion_outbox: ArtifactFixedQueue<TypedOperationCompletionWitness>,
    /// 🧩️ See [`ComposedGestureResult`] — one entry per composed gesture this actor published.
    pub typed_composed_outbox: ArtifactFixedQueue<ComposedGestureResult>,
    pub declared_tools:ManuallyDrop<Option<DeclaredToolCatalogAlias>>,
    pub declared_owner:ManuallyDrop<Option<semio_plugin_job::original_declared_tools::OriginalDeclaredToolCatalog>>,
    pub registry: AppActionRegistry,
    pub tool_job_controller_id: String,
    pub tool_jobs: ToolActionBus,
    pub agent_lane_preview:Option<semio_plugin_job::AgentLanePreview<A>>,
    pub bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>,
    pub bounded_tool_contracts: Vec<ArtifactToolPublicContract>,
    pub framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    pub app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>,
    pub instance_operation_owner: ArtifactInstanceOperationOwnerHandle,
    pub tool_cancellations: semio_plugin_operation::ToolCancellationIssuer,
    pub live_runtime_instance_id: Option<u32>,
    pub tool_operations: ArtifactFixedRegistry<Box<MountedTypedCommandFullOperation<A>>>,
    pub original_emission_admission:std::mem::ManuallyDrop<Option<Box<semio_framework_plugin_retained_command::typed_operation::original_emission::OriginalEmissionAdmission<A>>>>,
    pub original_emission_receipt:Option<semio_plugin_job::original_emission::OriginalEmissionReceipt>,
    #[cfg(test)]
    pub test_tool_clock: Option<fn() -> Option<u64>>,
    pub typed_operation_reservations: [Option<u64>; ARTIFACT_LIVE_OUTPUT_SLOTS],
    pub typed_publication_cursor: usize,
    pub typed_result_cursor: usize,
    pub latest_wins_commands: ArtifactFixedRegistry<PendingLatestWinsCommand<A>>,
    pub latest_wins_order: ArtifactFixedQueue<u64>,
    pub latest_wins_keys: ToolLatestWinsRegistry,
    pub latest_wins_turn: bool,
}

/// 🛠️ Original tool registration resources remain retained until the fixed runtime queues are admitted.
pub struct OriginalToolOperationsSources{pub registry:AppActionRegistry,pub declared:DeclaredToolCatalogAlias,pub bus:semio_framework::RegisteredActionBusAlias,pub owner:ArtifactInstanceOperationOwnerHandle}
/// 🌱️ Every original tool handle stays present until its final-address field receives it.
pub struct OriginalToolOperationsInitialization{registry:Option<AppActionRegistry>,declared:Option<DeclaredToolCatalogAlias>,bus:Option<semio_framework::RegisteredActionBusAlias>,owner:Option<ArtifactInstanceOperationOwnerHandle>}
impl OriginalToolOperationsInitialization{
 pub fn from_original(source:OriginalToolOperationsSources)->Self{Self{registry:Some(source.registry),declared:Some(source.declared),bus:Some(source.bus),owner:Some(source.owner)}}
 pub fn terminal_is_empty(&self)->bool{self.registry.is_none()&&self.declared.is_none()&&self.bus.is_none()&&self.owner.is_none()}
}
fn initial_field_bytes<S,T>(_:fn(&S)->&T)->usize{std::mem::size_of::<T>()}
unsafe fn initialize_manual_absent_at<T:'static>(target:*mut ManuallyDrop<Option<T>>,byte:usize,count:usize)->Result<bool,semio_framework_value::ValueError>{unsafe{semio_framework_value::initialization::initialize_absent_field_at(target.cast::<Option<T>>(),byte,count)}}


impl<A: ArtifactApp> ToolOperations<A> {
    /// 🎞️ Quotes this component frame and its fixed original backing; input roots remain separately admitted.
    pub fn birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_plugin_ownership::bounded_ownership::original_component_birth::<Self>(&[semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.tool_operations,64)?,semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.latest_wins_commands,64)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_effect_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_event_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_ui_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_completion_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_composed_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?,semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.latest_wins_order,64)?])}

    /// 🌱️ Moves original tool registrations into their actual fixed-queue runtime under the same mounted caller.
    pub fn admit_original(sources:&mut Option<OriginalToolOperationsSources>,cx:&mut semio_framework_job::StepContext<'_>,output:&mut Option<Self>)->Result<bool,semio_framework_job::OriginalTurnFailure>{if sources.is_none()||output.is_some(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool runtime birth requires retained registrations and an empty destination").into())}let mut d=Self::birth_demands()?;d.copy_bytes=d.copy_bytes.checked_add(2*std::mem::size_of::<OriginalToolOperationsSources>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original tool source extent overflow"))?;semio_framework_job::admit_original_construction(cx,output,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:d.depth},|_|{let source=sources.take().unwrap();semio_framework_value::RetainedWorkOutcome{result:Ok((Self::from_original(source),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:1}})?;Ok(output.is_some())}

    /// ⌨️ The composite source preserves both original input and actual fixed runtime headers.
    pub fn original_birth_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{let mut demand=Self::birth_demands()?;demand.copy_bytes=demand.copy_bytes.checked_add(2*std::mem::size_of::<OriginalToolOperationsSources>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original tool source extent overflow"))?;Ok(demand)}
    /// 🧱️ An enclosing original construction can admit the same runtime without another transient owner.
    pub fn build_original(source:OriginalToolOperationsSources,grant:semio_framework_value::RetainedCloneGrant)->Result<(Self,semio_framework_value::RetainedCloneProgress),(semio_framework_value::ValueError,OriginalToolOperationsSources)>{let demand=match Self::original_birth_demands(){Ok(demand)=>demand,Err(error)=>return Err((error,source))};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original tool runtime source is not funded"),source))}Ok((Self::from_original(source),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}

    /// 📋️ Enabled fields form one retained initialization inventory.
    pub const EFFECT_OUTBOX_FIELD:usize=7;
    /// 📋️ Every enabled runtime field has one retained cursor entry.
    pub const INITIAL_FIELDS:usize=38+(if cfg!(test){1}else{0});
    /// 📏️ Quotes one bounded absent chunk or one atomic live field and its actual backing.
    pub fn initial_field_demands(field:usize,byte:usize,body:usize,backing:bool)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(match field{
  0=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.pending_tool_lease_admission);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  1=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_tool_follow);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  2=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_scrub_dispatch);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  3=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_press_settlement);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  4=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_typing_settlement);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  5=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_tool_tag_capture);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  6=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_tool_tag_retirement);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  Self::EFFECT_OUTBOX_FIELD=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_effect_outbox),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_effect_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?}else{0},depth:1,..Default::default()},
  8=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_event_outbox),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_event_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?}else{0},depth:1,..Default::default()},
  9=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_ui_outbox),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_ui_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?}else{0},depth:1,..Default::default()},
  10=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.operation_progress_retired),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  11=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_inline_interaction_verbs),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  12=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.typed_inline_interaction_leftover);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  13=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_completion_outbox),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_completion_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?}else{0},depth:1,..Default::default()},
  14=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_composed_outbox),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.typed_composed_outbox,TYPED_OPERATION_HOST_OUTBOX_SLOTS)?}else{0},depth:1,..Default::default()},
  15=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.declared_tools)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.declared),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  16=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.declared_owner);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  17=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.registry)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.registry),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  18=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.tool_job_controller_id),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  19=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.tool_jobs)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.bus),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  20=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.agent_lane_preview);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  21=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.bounded_tool_proofs),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  22=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.bounded_tool_contracts),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  23=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.framework_tool_registrations),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  24=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.app_tool_registrations),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  25=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.instance_operation_owner)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.owner)+2*std::mem::size_of::<Option<OriginalToolOperationsInitialization>>(),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  26=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.tool_cancellations),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  27=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.live_runtime_instance_id);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  28=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.tool_operations),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.tool_operations,64)?}else{0},depth:1,..Default::default()},
  29=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_emission_admission);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  30=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.original_emission_receipt);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  31=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.typed_operation_reservations);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  32=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_publication_cursor),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  33=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.typed_result_cursor),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  34=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_commands),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_registry_backing::<Self,_>(|owner|&owner.latest_wins_commands,64)?}else{0},depth:1,..Default::default()},
  35=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_order),capacity_bytes:if backing{semio_plugin_ownership::bounded_ownership::original_queue_backing::<Self,_>(|owner|&owner.latest_wins_order,ARTIFACT_LIVE_OUTPUT_SLOTS)?}else{0},depth:1,..Default::default()},
  36=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_keys),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  37=>semio_framework_value::RetirementDemand{copy_bytes:2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_turn),capacity_bytes:if backing{0}else{0},depth:1,..Default::default()},
  #[cfg(test)]
  38=>{let extent=initial_field_bytes::<Self,_>(|owner|&owner.test_tool_clock);if byte>extent{return Err(semio_framework_value::ValueError::invariant("absent field cursor exceeds its extent"))}semio_framework_value::RetirementDemand{copy_bytes:if byte==extent{0}else{body.max(1).min(extent-byte)},depth:1,..Default::default()}},
  _=>return Err(semio_framework_value::ValueError::invariant("initial field cursor exceeds its inventory")),
 })}
    /// 🖋️ Copies one funded static absent chunk or initializes one selected live field in place.
    /// # Safety
    /// The target is exclusive caller-funded storage; the retained cursor advances only after successful entry.
    pub unsafe fn initialize_field_at(target:*mut Self,source:&mut Option<OriginalToolOperationsInitialization>,field:usize,byte:usize,count:usize,backing:bool)->Result<bool,semio_framework_value::ValueError>{match field{
  0=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).pending_tool_lease_admission),byte,count)},
  1=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_tool_follow),byte,count)},
  2=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_scrub_dispatch),byte,count)},
  3=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_press_settlement),byte,count)},
  4=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_typing_settlement),byte,count)},
  5=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_tool_tag_capture),byte,count)},
  6=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_tool_tag_retirement),byte,count)},
  Self::EFFECT_OUTBOX_FIELD=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_effect_outbox){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_effect_outbox).write(if backing{ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS)}else{ArtifactFixedQueue::unallocated(TYPED_OPERATION_HOST_OUTBOX_SLOTS)});}Ok(true)},
  8=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_event_outbox){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_event_outbox).write(if backing{ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS)}else{ArtifactFixedQueue::unallocated(TYPED_OPERATION_HOST_OUTBOX_SLOTS)});}Ok(true)},
  9=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_ui_outbox){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_ui_outbox).write(if backing{ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS)}else{ArtifactFixedQueue::unallocated(TYPED_OPERATION_HOST_OUTBOX_SLOTS)});}Ok(true)},
  10=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.operation_progress_retired){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).operation_progress_retired).write(false);}Ok(true)},
  11=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_inline_interaction_verbs){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_inline_interaction_verbs).write(Vec::new());}Ok(true)},
  12=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).typed_inline_interaction_leftover),byte,count)},
  13=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_completion_outbox){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_completion_outbox).write(if backing{ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS)}else{ArtifactFixedQueue::unallocated(TYPED_OPERATION_HOST_OUTBOX_SLOTS)});}Ok(true)},
  14=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_composed_outbox){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_composed_outbox).write(if backing{ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS)}else{ArtifactFixedQueue::unallocated(TYPED_OPERATION_HOST_OUTBOX_SLOTS)});}Ok(true)},
  15=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.declared_tools)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.declared){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).declared_tools).write(ManuallyDrop::new(Some(source.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("original tool source absent"))?.declared.take().ok_or_else(||semio_framework_value::ValueError::invariant("original tool declared absent"))?)));}Ok(true)},
  16=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).declared_owner),byte,count)},
  17=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.registry)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.registry){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).registry).write(source.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("original tool source absent"))?.registry.take().ok_or_else(||semio_framework_value::ValueError::invariant("original tool registry absent"))?);}Ok(true)},
  18=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.tool_job_controller_id){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).tool_job_controller_id).write(String::new());}Ok(true)},
  19=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.tool_jobs)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.bus){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).tool_jobs).write(ToolActionBus::Original(source.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("original tool source absent"))?.bus.take().ok_or_else(||semio_framework_value::ValueError::invariant("original tool bus absent"))?));}Ok(true)},
  20=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).agent_lane_preview),byte,count)},
  21=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.bounded_tool_proofs){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).bounded_tool_proofs).write(BTreeMap::new());}Ok(true)},
  22=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.bounded_tool_contracts){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).bounded_tool_contracts).write(Vec::new());}Ok(true)},
  23=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.framework_tool_registrations){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).framework_tool_registrations).write(BTreeMap::new());}Ok(true)},
  24=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.app_tool_registrations){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).app_tool_registrations).write(BTreeMap::new());}Ok(true)},
  25=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.instance_operation_owner)+initial_field_bytes::<OriginalToolOperationsInitialization,_>(|source|&source.owner)+2*std::mem::size_of::<Option<OriginalToolOperationsInitialization>>(){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).instance_operation_owner).write(source.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("original tool source absent"))?.owner.take().ok_or_else(||semio_framework_value::ValueError::invariant("original tool owner absent"))?);if !source.as_ref().is_some_and(OriginalToolOperationsInitialization::terminal_is_empty){return Err(semio_framework_value::ValueError::invariant("original tool donation retains a handle"))}source.take();}Ok(true)},
  26=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.tool_cancellations){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).tool_cancellations).write(semio_plugin_operation::ToolCancellationIssuer::empty());}Ok(true)},
  27=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).live_runtime_instance_id),byte,count)},
  28=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.tool_operations){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).tool_operations).write(if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(ARTIFACT_LIVE_OUTPUT_SLOTS)});}Ok(true)},
  29=>unsafe{initialize_manual_absent_at(std::ptr::addr_of_mut!((*target).original_emission_admission),byte,count)},
  30=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).original_emission_receipt),byte,count)},
  31=>unsafe{semio_framework_value::initialization::initialize_absent_array_at(std::ptr::addr_of_mut!((*target).typed_operation_reservations),byte,count)},
  32=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_publication_cursor){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_publication_cursor).write(0);}Ok(true)},
  33=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.typed_result_cursor){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).typed_result_cursor).write(0);}Ok(true)},
  34=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_commands){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).latest_wins_commands).write(if backing{ArtifactFixedRegistry::new()}else{ArtifactFixedRegistry::unallocated(ARTIFACT_LIVE_OUTPUT_SLOTS)});}Ok(true)},
  35=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_order){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).latest_wins_order).write(if backing{ArtifactFixedQueue::new(ARTIFACT_LIVE_OUTPUT_SLOTS)}else{ArtifactFixedQueue::unallocated(ARTIFACT_LIVE_OUTPUT_SLOTS)});}Ok(true)},
  36=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_keys){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).latest_wins_keys).write(ToolLatestWinsRegistry::new());}Ok(true)},
  37=>{if byte!=0||count!=2*initial_field_bytes::<Self,_>(|owner|&owner.latest_wins_turn){return Err(semio_framework_value::ValueError::invariant("live initial field requires its full atomic copy extent"))}unsafe{std::ptr::addr_of_mut!((*target).latest_wins_turn).write(true);}Ok(true)},
  #[cfg(test)]
  38=>unsafe{semio_framework_value::initialization::initialize_absent_field_at(std::ptr::addr_of_mut!((*target).test_tool_clock),byte,count)},
  _=>Err(semio_framework_value::ValueError::invariant("initial field cursor exceeds its inventory")),
 }}

    fn from_original(source:OriginalToolOperationsSources)->Self{let mut value=Self::new_with_bus(source.registry,String::new(),ToolActionBus::Original(source.bus),BTreeMap::new(),Vec::new(),BTreeMap::new(),BTreeMap::new(),source.owner);*value.declared_tools=Some(source.declared);value}
    pub fn tool_controller_id(&self)->&str{self.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::catalog).map_or(self.tool_job_controller_id.as_str(),|catalog|catalog.controller_id.as_str())}
    pub fn app_registrations(&self)->impl Iterator<Item=(&String,&ArtifactToolRegistration)>{self.app_tool_registrations.iter().chain(self.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::catalog).into_iter().flat_map(|catalog|catalog.app.iter().map(|(key,row)|(key,row))))}
    pub fn framework_registrations(&self)->impl Iterator<Item=(&String,&ArtifactToolRegistration)>{self.framework_tool_registrations.iter().chain(self.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::catalog).into_iter().flat_map(|catalog|catalog.framework.iter().map(|(key,row)|(key,row))))}
    pub fn app_registration(&self,id:&str)->Option<&ArtifactToolRegistration>{self.app_registrations().find_map(|(key,row)|(key==id).then_some(row))}
    pub fn framework_registration(&self,id:&str)->Option<&ArtifactToolRegistration>{self.framework_registrations().find_map(|(key,row)|(key==id).then_some(row))}
    pub fn bounded_proof(&self,id:&str)->Option<&QualifiedBoundedFirstStepProof>{self.bounded_tool_proofs.get(id).or_else(||self.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::catalog).and_then(|catalog|catalog.bounded(id)))}
    pub fn bounded_contracts(&self)->impl Iterator<Item=&ArtifactToolPublicContract>{self.bounded_tool_contracts.iter().chain(self.declared_tools.as_ref().and_then(DeclaredToolCatalogAlias::catalog).into_iter().flat_map(|catalog|catalog.bounded_contracts.iter()))}
    /// 🔭️ Builds the component from the parts the composer constructs first; every other field starts at its construction value.
    pub fn new(registry: AppActionRegistry, tool_job_controller_id: String, tool_jobs: semio_framework::ActionBus, bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>, mut bounded_tool_contracts: Vec<ArtifactToolPublicContract>, framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, instance_operation_owner: ArtifactInstanceOperationOwnerHandle) -> Self {
        bounded_tool_contracts.sort_by(|left,right|left.tool_id.cmp(&right.tool_id));
        let catalog=DeclaredToolCatalog{controller_id:tool_job_controller_id.clone(),bus:semio_framework::RegisteredActionBusSource::reference(tool_jobs.clone()),bounded_proofs:bounded_tool_proofs.into_iter().collect(),bounded_contracts:bounded_tool_contracts.into_iter().collect(),framework:framework_tool_registrations.into_iter().collect(),app:app_tool_registrations.into_iter().collect()};
        let mut owner=semio_plugin_job::original_declared_tools::OriginalDeclaredToolCatalog::new(catalog);
        owner.commit_producer();
        let grant=semio_framework_value::RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX};
        let(alias,_)=DeclaredToolCatalogAlias::admit(owner.source().expect("reference tool catalog owner is live at construction"),grant).expect("reference tool catalog source is committed").expect("reference tool catalog lease is funded");
        let mut value=Self::new_with_bus(registry,tool_job_controller_id,ToolActionBus::Reference(tool_jobs),BTreeMap::new(),Vec::new(),BTreeMap::new(),BTreeMap::new(),instance_operation_owner);
        *value.declared_tools=Some(alias);
        *value.declared_owner=Some(owner);
        value
    }
    fn new_with_bus(registry: AppActionRegistry, tool_job_controller_id: String, tool_jobs: ToolActionBus, bounded_tool_proofs: BTreeMap<String, QualifiedBoundedFirstStepProof>, bounded_tool_contracts: Vec<ArtifactToolPublicContract>, framework_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, app_tool_registrations: BTreeMap<String, ArtifactToolRegistration>, instance_operation_owner: ArtifactInstanceOperationOwnerHandle) -> Self {
        Self {
            pending_tool_lease_admission: ManuallyDrop::new(None),
            original_tool_follow:ManuallyDrop::new(None),original_scrub_dispatch:ManuallyDrop::new(None),original_press_settlement:ManuallyDrop::new(None),original_typing_settlement:ManuallyDrop::new(None),original_tool_tag_capture:ManuallyDrop::new(None),original_tool_tag_retirement:ManuallyDrop::new(None),
            typed_effect_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_event_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_ui_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            operation_progress_retired: false,
            typed_inline_interaction_verbs: Vec::new(),
            typed_inline_interaction_leftover: None,
            typed_completion_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_composed_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            declared_tools:ManuallyDrop::new(None),
            declared_owner:ManuallyDrop::new(None),
            registry,
            tool_job_controller_id,
            tool_jobs,
            agent_lane_preview: None,
            bounded_tool_proofs,
            bounded_tool_contracts,
            framework_tool_registrations,
            app_tool_registrations,
            instance_operation_owner,
            tool_cancellations: semio_plugin_operation::ToolCancellationIssuer::empty(),
            live_runtime_instance_id: None,
            tool_operations: ArtifactFixedRegistry::new(),
            original_emission_admission:std::mem::ManuallyDrop::new(None),original_emission_receipt:None,
            #[cfg(test)]
            test_tool_clock: None,
            typed_operation_reservations: [None; ARTIFACT_LIVE_OUTPUT_SLOTS],
            typed_publication_cursor: 0,
            typed_result_cursor: 0,
            latest_wins_commands: ArtifactFixedRegistry::new(),
            latest_wins_order: ArtifactFixedQueue::new(ARTIFACT_LIVE_OUTPUT_SLOTS),
            latest_wins_keys: ToolLatestWinsRegistry::new(),
            latest_wins_turn: true,
        }
    }

    pub fn take_typed_operation_preparation_receipt(&mut self) -> Option<(semio_framework_job::OperationId, semio_framework_value::retained_clone::RetainedCloneGrant, semio_framework_value::retained_clone::RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.latest_wins_commands.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.receipt.take(){return Some((owner.operation.operation,grant,progress));}
        }

        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.tool_operations.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.publication_ownership_progress.take(){return Some((owner.operation.operation,grant,progress));}
        }
        None
    }
}

#[path="🏛️declarations/🦀️.rs"]
mod declared_tools;
pub use declared_tools::*;

/// 🧷️ Native job dispatch borrows the committed registry alias until its paid return; reference construction retains its explicit bus.
pub enum ToolActionBus{Reference(semio_framework::ActionBus),Original(semio_framework::RegisteredActionBusAlias)}
impl std::ops::Deref for ToolActionBus{type Target=semio_framework::ActionBus;fn deref(&self)->&Self::Target{match self{Self::Reference(bus)=>bus,Self::Original(bus)=>bus.bus()}}}
impl ToolActionBus{
 pub fn original_terminal_is_empty(&self)->bool{match self{Self::Reference(_)=>true,Self::Original(bus)=>bus.terminal_is_empty()}}
 pub fn original_close_demands(&self)->semio_framework_value::RetirementDemand{match self{Self::Reference(_)=>Default::default(),Self::Original(bus)=>bus.close_demands()}}
 pub fn original_close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedCloneStep{match self{Self::Reference(_)=>semio_framework_value::RetainedCloneStep::Complete(Default::default()),Self::Original(bus)=>bus.close_step(grant)}}
}

/// 🛡️ Moves one resident failure into its prepaid domain reservation and publishes it as the original turn failure once the reservation is funded.
pub fn admit_pending_domain_failure<T:semio_framework_job::OriginalDomainFailure>(failure:&mut Option<T>,place:&mut Option<semio_framework_job::OriginalDomainReservation<T>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::OriginalTurnFailure>,semio_framework_job::OriginalTurnFailure>{
    if failure.is_none()&&place.is_none(){return Ok(None)}
    if !place.as_ref().is_some_and(|place|place.ready()){place.get_or_insert_with(Default::default).prepare(cx)?;return Ok(None)}
    let reservation=place.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("original domain failure place"))?;
    if !reservation.pending(){reservation.with_body(cx,|_,slot|{*slot=failure.take();Ok(())})?;}
    let admitted=semio_framework_job::admit_original_domain_failure(reservation,cx)?;
    if place.as_ref().is_some_and(|place|place.terminal_is_empty()){*place=None;}
    Ok(admitted)
}
