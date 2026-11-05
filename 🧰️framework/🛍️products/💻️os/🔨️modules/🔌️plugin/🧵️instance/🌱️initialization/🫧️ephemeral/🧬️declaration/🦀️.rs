//! 🧬️ Authored ephemeral lanes declare exact initial sources and concrete lifecycle allocations.
use super::*;
/// 🧩️ Ephemeral ownership uses the actual app lane types without requiring a document controller in neutral laws.
pub trait OriginalEphemeralLanes:'static{
 type Presence:RetireOwned+Clone+Default+Send+Sync+'static;
 type PresenceMutation:store::Mutation<Self::Presence>+store::ColdMutation<Self::Presence>+Send+'static;
 type Transient:RetireOwned+Clone+Default+Send+Sync+'static;
 type TransientMutation:store::Mutation<Self::Transient>+store::ColdMutation<Self::Transient>+Send+'static;
}
impl<A:semio_framework_plugin_decl_app::ArtifactApp> OriginalEphemeralLanes for A{type Presence=A::Presence;type PresenceMutation=A::PresenceMutation;type Transient=A::Transient;type TransientMutation=A::TransientMutation;}
/// 📦️ A presence birth installs these exact local and peer issuers alongside its actual disposer and optional preparation.
pub struct OriginalPresenceOwners<A:OriginalEphemeralLanes>{pub local:Arc<dyn store::SnapshotRetirementFactory<A::Presence>>,pub peer:Arc<dyn store::SnapshotRetirementFactory<A::Presence>>,pub disposer:Disposer<Presence<A>>,pub preparation:Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Presence,A::PresenceMutation>>>}
/// 📦️ A transient birth installs its exact disposer and aliases without constructing another Store.
pub struct OriginalTransientOwners<A:OriginalEphemeralLanes>{pub retirement:Arc<dyn store::SnapshotRetirementFactory<A::Transient>>,pub disposer:Disposer<Transient<A>>,pub preparation:Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<A::Transient,A::TransientMutation>>>}
/// 🌱️ Each concrete declaration quotes every factory Arc, disposer Box and nested constructor before original admission.
pub trait OriginalEphemeralDeclaration<A:OriginalEphemeralLanes>:'static{
 type PresenceInitial:OriginalStoreInitialSource<A::Presence>;
 type TransientInitial:OriginalStoreInitialSource<A::Transient>;
 fn presence_owners_demands()->Result<RetirementDemand,ValueError>;
 fn presence_owners(initial:&Arc<A::Presence>)->OriginalPresenceOwners<A>;
 fn transient_owners_demands()->Result<RetirementDemand,ValueError>;
 fn transient_owners()->OriginalTransientOwners<A>;
}
/// 🫙️ Only genuinely empty lane pairs share the empty authored declaration.
pub trait OriginalEmptyEphemeralLanes:OriginalEphemeralLanes<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>{}
impl<A:semio_framework_plugin_decl_app::ArtifactApp<Presence=NoPresence,PresenceMutation=NoPresenceMutation,Transient=NoTransient,TransientMutation=NoTransientMutation>> OriginalEmptyEphemeralLanes for A{}
pub struct EmptyLanes;
impl OriginalEphemeralLanes for EmptyLanes{type Presence=NoPresence;type PresenceMutation=NoPresenceMutation;type Transient=NoTransient;type TransientMutation=NoTransientMutation;}
impl OriginalEmptyEphemeralLanes for EmptyLanes{}
pub struct EmptyPresenceInitial;
impl OriginalFixedStoreInitial for EmptyPresenceInitial{type State=NoPresence;fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(fixed::<NoPresence>())}fn initial_state(g:RetainedCloneGrant)->Result<(NoPresence,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Err(invalid("empty presence source requires its original fixed grant"))}Ok((NoPresence{},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}
pub struct EmptyTransientInitial;
impl OriginalFixedStoreInitial for EmptyTransientInitial{type State=NoTransient;fn initial_state_demands()->Result<RetirementDemand,ValueError>{Ok(fixed::<NoTransient>())}fn initial_state(g:RetainedCloneGrant)->Result<(NoTransient,RetainedCloneProgress),ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(g,d){return Err(invalid("empty transient source requires its original fixed grant"))}Ok((NoTransient{},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}}
pub struct OriginalEmptyEphemeralDeclaration;
impl<A:OriginalEmptyEphemeralLanes> OriginalEphemeralDeclaration<A> for OriginalEmptyEphemeralDeclaration{
 type PresenceInitial=FixedStoreInitial<EmptyPresenceInitial>;
 type TransientInitial=FixedStoreInitial<EmptyTransientInitial>;
 fn presence_owners_demands()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<OriginalPresenceOwners<A>>(),capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<presence_retirement::NoPresenceRetirementFactory>()+size_of::<presence_retirement::PresenceStoreOwnedDisposer<NoPresence>>(),depth:2,..Default::default()})}
 fn presence_owners(initial:&Arc<A::Presence>)->OriginalPresenceOwners<A>{let factory=presence_retirement::no_presence_local_root_retirement_factory();let disposer=presence_retirement::PresenceStoreOwnedDisposer::new(initial.clone(),|_|true).unwrap_or_else(|_|unreachable!("NoPresence is statically empty"));OriginalPresenceOwners{local:factory.clone(),peer:factory,disposer:Box::new(disposer),preparation:None}}
 fn transient_owners_demands()->Result<RetirementDemand,ValueError>{empty_transient_owners_demands::<A>()}
 fn transient_owners()->OriginalTransientOwners<A>{empty_transient_owners::<A>()}
}
/// 📏️ An actual NoTransient lane owns two factory allocations and its concrete disposer frame.
pub fn empty_transient_owners_demands<A:OriginalEphemeralLanes<Transient=NoTransient,TransientMutation=NoTransientMutation>>()->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:2*size_of::<OriginalTransientOwners<A>>(),capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<semio_framework_value_factory::OwnedValueRetirementFactory<NoTransient>>()+semio_framework_value::factory_arc_birth_bytes::<semio_framework_value_factory::SharedValueRetirementFactory<NoTransient>>()+size_of::<transient_publication::TransientStoreDisposer<NoTransient,NoTransientMutation>>(),depth:2,..Default::default()})}
/// 🫧️ This constructor is used only inside the declaration's admitted original birth.
pub fn empty_transient_owners<A:OriginalEphemeralLanes<Transient=NoTransient,TransientMutation=NoTransientMutation>>()->OriginalTransientOwners<A>{OriginalTransientOwners{retirement:transient_publication::bounded_transient_root_retirement_factory::<NoTransient>(),disposer:transient_publication::bounded_transient_store_disposer::<NoTransient,NoTransientMutation>(),preparation:None}}
pub type OriginalEmptyEphemeralSource<A:OriginalEmptyEphemeralLanes=EmptyLanes>=OriginalEphemeralSource<A,OriginalEmptyEphemeralDeclaration>;
pub type OriginalEmptyEphemeralStores<A:OriginalEphemeralLanes=EmptyLanes>=OriginalEphemeralStores<A>;
pub(super) type InitialValues<A>=(Option<<A as OriginalEphemeralLanes>::Presence>,Option<<A as OriginalEphemeralLanes>::Transient>);
pub(super) fn initial_empty<A:OriginalEphemeralLanes>(value:&InitialValues<A>)->bool{value.0.is_none()&&value.1.is_none()}
pub(super) fn close_initial<P,S:OriginalStoreInitialSource<P>>(source:&mut Option<Box<S>>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let Some(owner)=source.as_mut()else{return Ok(true)};if !owner.terminal_is_empty(){owner.close_step(cx)?;return Ok(false)}let d=S::boxed_release_demands();close_original_retained_step(cx,d,|_|{source.take();semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})),work:1}})?;Ok(source.is_none())}
