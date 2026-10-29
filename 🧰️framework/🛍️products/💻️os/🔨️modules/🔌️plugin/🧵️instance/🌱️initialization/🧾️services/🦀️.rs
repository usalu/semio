//! 🧾️ Registration resources, framework service owners and the retained controller closure port from which a mounted instance is assembled.
use crate::stores::{OriginalArtifactAppBirth,OriginalArtifactStores};
use semio_framework_plugin_instance_stores::stores_components::ActionIngress;
use semio_framework::kernel::ActorId;
use std::sync::Arc;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry;
use std::collections::BTreeMap;
use semio_framework_plugin_instance_children::children_components::ChildComposition;
use semio_framework_plugin_instance_ladders::ladders_components::CloseCursors;
use semio_framework_plugin_instance_history::history_components::CommandHistory;
use store::os_store::composition_coordinator::CompositionCoordinator;
use semio_framework_plugin_instance_docs::docs_components::EnvelopeJobs;
use semio_framework::Fault;
use semio_framework_plugin_instance_history::history_components::HostEvents;
use semio_framework_plugin_instance_stores::stores_components::LocalInteraction;
use semio_framework_plugin_instance_ladders::ladders_components::MaintenanceCursors;
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use store::MemberFactory;
use store::Mutation;
use semio_framework_job::OriginalConstructionDemand;
use semio_framework_job::OriginalTurnFailure;
use semio_framework_plugin_instance_stores::stores_components::PeerRoster;
use semio_framework_plugin_instance_jobs::jobs_components::PendingReserved;
use semio_framework_plugin_window_kits::RegisteredDefinitionSource;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_value::RetirementDemand;
use store::ReferenceSpaceMember;
use semio_framework_job::StepContext;
use semio_framework_plugin_time_travel::TimeTravelLedger;
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::ToolMachineRuntime;
use semio_framework_plugin_instance_jobs::jobs_components::ToolOperations;
use semio_framework_plugin_tool_run::ToolRunLedger;
use semio_framework_plugin_instance_history::history_components::Transactions;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_job::admit_original_construction;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use std::mem::size_of;
/// 🫧️ Actual admitted ephemeral roots, Window registries and installed concrete factories belong to the registration source.
pub struct OriginalInstanceLiveStores<A:ArtifactApp>{
 pub retired_window_transient:semio_framework_plugin_ownership::bounded_ownership::ArtifactFixedRegistry<WindowTransientOwnerRegistry>,
 pub presence:store::PresenceStore<A::Presence,A::PresenceMutation>,pub transient:store::TransientStore<A::Transient,A::TransientMutation>,pub window_config:semio_framework_plugin_window_config::WindowConfigOwnerRegistry,pub window_transient:WindowTransientOwnerRegistry,
 pub artifact_factory:Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot,A::Mutation>>>,pub config_factory:Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Config,A::ConfigMutation>>>,pub draft_factory:Option<Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Draft,A::DraftMutation>>>,
 pub presence_factory:Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Presence,A::PresenceMutation>>>,pub transient_factory:Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Transient,A::TransientMutation>>>,pub presence_retirement:Option<Arc<dyn store::SnapshotRetirementFactory<A::Presence>>>,pub transient_retirement:Option<Arc<dyn store::SnapshotRetirementFactory<A::Transient>>>,pub unsupported:BTreeMap<String,&'static str>,
}
/// 🏛️ These service owners are the registration's originals; assembly invokes no registry, factory or default-state constructor.
pub struct OriginalInstanceServices<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static>{
 pub policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,pub roster:PeerRoster<A>,pub ingress:ActionIngress,pub interaction:LocalInteraction<A>,pub history:CommandHistory<A>,pub host_events:HostEvents,pub txn:Transactions<A>,pub children:ChildComposition<M>,pub pending:PendingReserved<A>,pub tools:ToolOperations<A>,pub media:MediaTransfers<A>,pub ingest:EnvelopeJobs<A,M>,pub maintenance:MaintenanceCursors,pub close:CloseCursors<A,M>,pub tool_runs:ToolRunLedger<A>,pub time_travel:TimeTravelLedger<A>,pub tool_machines:ToolMachineRuntime<A::Snapshot,A::Mutation,A::Config,A::ConfigMutation>,
}
/// 🧾️ Registration retains these original issuers and tools while fixed framework service backing awaits admission.
pub struct OriginalInstanceServiceSources<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static>{
 pub policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,pub peer_presence:Arc<semio_framework_plugin_decl_views::PeerPresenceRoot>,pub peer_scope:semio_framework_job::CancelToken,
 pub field_decoders:Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry<A::Snapshot,A::Mutation>>,pub completed_records:Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry<A::Snapshot,A::Mutation>>,
 pub tools:ToolOperations<A>,pub close:CloseCursors<A,M>,pub dispatch_report:store::DispatchReport,
}
impl<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static> OriginalInstanceServices<A,M>{
 pub(crate) fn from_original(source:OriginalInstanceServiceSources<A,M>)->Self{Self{policy:source.policy,roster:PeerRoster::from_original(source.peer_presence,source.peer_scope),ingress:ActionIngress::new(),interaction:LocalInteraction::new(),history:CommandHistory::new(),host_events:HostEvents::new(),txn:Transactions::new(),children:ChildComposition::new(CompositionCoordinator::default()),pending:PendingReserved::new(source.dispatch_report),tools:source.tools,media:MediaTransfers::new(),ingest:EnvelopeJobs::from_original(source.field_decoders,source.completed_records),maintenance:MaintenanceCursors::new(),close:source.close,tool_runs:ToolRunLedger::default(),time_travel:TimeTravelLedger::default(),tool_machines:ToolMachineRuntime::default()}}
 /// 📏️ Prices every fixed service arena while the actual registration issuers remain outside this birth.
 pub fn birth_demands()->Result<RetirementDemand,ValueError>{let capacity_bytes=[PeerRoster::<A>::birth_demands()?.capacity_bytes,ChildComposition::<M>::birth_demands()?.capacity_bytes,PendingReserved::<A>::birth_demands()?.capacity_bytes,MediaTransfers::<A>::birth_demands()?.capacity_bytes,EnvelopeJobs::<A,M>::birth_demands()?.capacity_bytes].into_iter().try_fold(0usize,|sum,bytes|sum.checked_add(bytes)).ok_or_else(||invalid("original instance service backing extent overflow"))?;Ok(RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<OriginalInstanceServiceSources<A,M>>(),capacity_bytes,depth:2,..Default::default()})}

 /// 🌱️ Constructs bounded framework services from actual registration roots without minting a caller, default payload or registry issuer.
 pub fn admit(sources:&mut Option<OriginalInstanceServiceSources<A,M>>,cx:&mut StepContext<'_>,output:&mut Option<Self>)->Result<bool,OriginalTurnFailure>{if output.is_some()||sources.is_none(){return Err(invalid("original service admission requires retained registration sources and an empty destination").into())}let d=Self::birth_demands()?;let mut construct=|_:semio_framework_value::RetainedCloneGrant|{let source=sources.take().ok_or_else(||invalid("instance services require their retained sources"))?;let value=Self::from_original(source);Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))};let grant=cx.retained_grant();admit_original_construction(cx,output,construction(d),|_|construct(grant))?;Ok(output.is_some())}
}

/// 🧭️ The mounted instance retains its concrete controller's original closure port through final retirement.
pub struct OriginalControllerClose<A>{pub demands:fn(&A,usize)->Result<RetirementDemand,ValueError>,pub step:fn(&mut A,&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>,pub terminal:fn(&A)->bool}
impl<A:OriginalArtifactAppBirth> OriginalControllerClose<A>{pub fn new()->Self{Self{demands:A::controller_retirement_demands,step:A::controller_close_step,terminal:A::controller_terminal_is_empty}}}
/// 🧺️ The registration producer hands off the exact live Store and framework-service owners together.
pub struct OriginalInstanceRegistrationParts<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static>{pub live:OriginalInstanceLiveStores<A>,pub services:OriginalInstanceServices<A,M>}
/// 🎟️ An authored source quotes and admits its real per-instance registrations; no Default or reference factory is implied.
pub trait OriginalInstanceRegistration<A:OriginalArtifactAppBirth,M:ReferenceSpaceMember+MemberFactory+'static>:Send+'static{
 fn produce(source:RegisteredDefinitionSource<'_>)->Result<Box<dyn semio_framework_plugin_reactor_app::OriginalPluginAppRegistration>,semio_framework::Fault>{semio_framework_plugin_instance_jobs::jobs_components::produce_original_tool_registration::<A>(source)}
 fn admit(source:RegisteredDefinitionSource<'_>,registration:&dyn semio_framework_plugin_reactor_app::OriginalPluginAppRegistration,actor:&protocol::ActorId,policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,cx:&mut StepContext<'_>,output:&mut Option<Box<Self>>)->Result<bool,OriginalTurnFailure> where Self:Sized;
 fn advance(&mut self,source:RegisteredDefinitionSource<'_>,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn take(&mut self,lanes:&OriginalArtifactStores<A>,cx:&mut StepContext<'_>,output:&mut Option<OriginalInstanceRegistrationParts<A,M>>)->Result<bool,OriginalTurnFailure>;
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>;
 fn terminal_is_empty(&self)->bool;
 fn boxed_release_demands()->RetirementDemand where Self:Sized;
}

/// 🚫️ A violated registration or assembly invariant, refused without panicking.
pub fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
/// 🧱️ The one-item construction demand that prices a retained-clone demand.
pub fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
