//! 📐️ Owner contract and mutation of the window config: the window config owner trait and its bounded factories, the mutation, the rejected emission and the snapshot. See [`super`].

use super::*;

/// 🪟️ Declares one concrete window kind's persisted-local configuration owner.
pub trait WindowConfigOwner: Send + Sync + 'static {
    const WINDOW_KIND_ID: &'static str;
    const SCHEMA: &'static str;
    const MAXIMUM_PUBLICATION_BYTES: usize;
    type State: semio_framework_value::retained_clone::RetainedClone + Clone + Default + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + store::ConfigRecord + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_dsl_record::DslField + semio_framework_value::retirement::RetireOwned + 'static;
    type Mutation: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + protocol::Mutation<Self::State> + protocol::ColdMutation<Self::State> + store::ArtifactCanonicalJsonTree + PartialEq + Send + Sync + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + 'static;

    type Edit: store::snapshot_clone_preparation::RetainedCloneEdit<Self::State,Self::Mutation> + semio_framework_value::FactoryPayloadRetirement;
    const MAXIMUM_PREPARATION_DEPTH: usize;
    fn build_retained_edit() -> Arc<Self::Edit>;

    fn build_mutation_retirement_factory()->Arc<dyn store::ArtifactOwnedValueRetirementFactory<Self::Mutation>>{Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<Self::Mutation>::default())}
    fn build_snapshot_retirement_factory()->Arc<dyn store::SnapshotRetirementFactory<Self::State>>{Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<Self::State>::default())}
    fn preparation_source_birth()->semio_framework_value::retained_clone::RetainedCloneBirthDemand;
    fn build_store_owners(grant:RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State,Self::Mutation>>;
    fn build_one_item_preparation_factory() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>>;
    fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>>;
}

/// 📏️ Quotes the concrete zero-payload issuer tree before any original Window factory allocates.
pub fn bounded_window_config_preparation_source_birth<O:WindowConfigOwner>()->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:semio_framework_value::factory_arc_birth_bytes::<O::Edit>()+semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::OwnedValueRetirementFactory<O::Mutation>>()+semio_framework_value::factory_arc_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<O::State>>(),depth:1}}
/// 🎟️ Constructs the exact paired native Window factory only after the caller funds its declared original sources.
pub fn bounded_window_config_store_owners<O:WindowConfigOwner>(grant:RetainedCloneGrant)->Result<(store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>,RetainedCloneProgress),store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<O::State,O::Mutation>>{
 let copy=2*std::mem::size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State,O::Mutation>>();if grant.maximum_copy_bytes<copy{return Err(store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError{error:ValueError::literal(ValueRefusalKind::WorkLimit,"original Window catalog header transfer not funded"),owners:None,progress:Default::default()})}
 store::os_store::document_store_owners::bounded_artifact_store_owners_with_shared_semantic_preparation(grant,O::preparation_source_birth(),copy,||store::snapshot_clone_preparation::RetainedClonePreparationFactory::new(O::build_retained_edit(),O::build_mutation_retirement_factory(),O::build_snapshot_retirement_factory(),O::MAXIMUM_PREPARATION_DEPTH).expect("original Window native depth declared before admission"))
}
/// 🧬️ Uses the original domain edit issuer and Store's admitted clone, metadata and canonical-seal owners.
pub fn bounded_window_config_preparation_factory<O: WindowConfigOwner>() -> Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<O::State, O::Mutation>> {
    assert!(O::MAXIMUM_PUBLICATION_BYTES>0&&O::MAXIMUM_PUBLICATION_BYTES<=store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES,"original window owner declares its admissible publication bound");
    Arc::new(store::snapshot_clone_preparation::RetainedClonePreparationFactory::new(O::build_retained_edit(),O::build_mutation_retirement_factory(),O::build_snapshot_retirement_factory(),O::MAXIMUM_PREPARATION_DEPTH).expect("original window owner declares nonzero preparation depth"))
}

pub fn bounded_window_config_store_disposer<O: WindowConfigOwner>() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<O::State, O::Mutation>>> {
    semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer::<O::State, O::Mutation>()
}

/// 📬️ One typed config mutation addressed to one exact concrete window instance; clonable, so a press holds it as one of its
/// provisional leaves (design §20.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowConfigMutation {
    #[doc(hidden)]
    pub window_id: String,
    #[doc(hidden)]
    pub window_kind_id: &'static str,
    #[doc(hidden)]
    pub mutation: Box<dyn ErasedWindowMutationValue>,
}

impl Clone for WindowConfigMutation {
    fn clone(&self) -> Self {
        Self { window_id: self.window_id.clone(), window_kind_id: self.window_kind_id, mutation: self.mutation.clone_value() }
    }
}

impl std::fmt::Debug for WindowConfigMutation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("WindowConfigMutation").field("window_id", &self.window_id).field("window_kind_id", &self.window_kind_id).finish_non_exhaustive()
    }
}

/// 🚫️ A refused window-config emission hands its mutation BACK. The retained publication ladder
/// pops one mutation out of the operation's `Emit` before it begins a batch, so a refusal that kept the
/// mutation left the retry with nothing to publish and the operation completed clean — no page, no
/// fault, the amend lost (ticket 26/09/19 `📓️flow.md` §5.2). Every other publication lane of that ladder
/// already returns its owners on rejection; this is the window-config lane's carrier for the same rule.
pub struct RejectedWindowConfigEmission {
    pub mutation: WindowConfigMutation,
    pub fault: Fault,
}

impl std::fmt::Debug for RejectedWindowConfigEmission {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("RejectedWindowConfigEmission").field("fault", &self.fault).finish_non_exhaustive()
    }
}

impl WindowConfigMutation {
    pub fn of<O: WindowConfigOwner>(window_id: impl Into<String>, mutation: O::Mutation) -> Self {
        Self::from_issued(window_id,O::WINDOW_KIND_ID,mutation,O::build_mutation_retirement_factory())
    }

    #[doc(hidden)]
    pub fn from_issued<T:Clone+Send+Sync+'static>(window_id:impl Into<String>,window_kind_id:&'static str,mutation:T,factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<T>>)->Self{Self{window_id:window_id.into(),window_kind_id,mutation:Box::new(FactoryBoxedValue{original:Box::new(mutation),factory})}}

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }
}

impl crate::window_mutation::refresh::WindowRefreshSnapshot for WindowConfigSnapshot{fn swap_address(&mut self,other:&mut Self){std::mem::swap(&mut self.window_id,&mut other.window_id);}}

impl WindowConfigSnapshot{pub fn try_duplicate(&self)->Result<Self,ValueError>{Ok(Self{authority:self.authority,window_id:self.window_id.clone(),window_kind_id:self.window_kind_id,generation:self.generation,revision:self.revision,snapshot:self.snapshot.try_duplicate()?})}}

/// 📖️ Immutable snapshot of one exact window-owned config partition.
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowConfigSnapshot {
    #[doc(hidden)]
    pub authority: WindowConfigAuthorityIdentity,
    #[doc(hidden)]
    pub window_id: semio_framework_value::SharedUtf8,
    #[doc(hidden)]
    pub window_kind_id: &'static str,
    #[doc(hidden)]
    pub generation: u64,
    #[doc(hidden)]
    pub revision: [u8; 32],
    #[doc(hidden)]
    pub snapshot: store::os_store::snapshot_read::ErasedSnapshotRead,
}

/// 🪪️ Checked process-unique identity issued to one original window partition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_value::RetireOwned)]
pub struct WindowConfigAuthorityIdentity(u64);

impl WindowConfigAuthorityIdentity {
    pub(crate) fn issue() -> Option<Self> {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        NEXT.fetch_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed, |next| next.checked_add(1)).ok().map(Self)
    }
}

/// 👁️ Borrows one original committed window partition without issuing or copying an owner.
pub struct WindowConfigBorrow<'a> {
    pub authority: WindowConfigAuthorityIdentity,
    pub window_id: &'a str,
    pub window_kind_id: &'static str,
    pub generation: u64,
    pub revision: [u8; 32],
    pub(crate) state: &'a dyn std::any::Any,
}

impl WindowConfigBorrow<'_> {
    /// 🧬️ Lends only the state of the exact declared window owner.
    pub fn get<O: WindowConfigOwner>(&self) -> Option<&O::State> {
        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.state.downcast_ref::<O::State>()).flatten()
    }
}

impl WindowConfigSnapshot {
    /// 🪪️ Returns the exact original partition authority captured with this snapshot.
    pub fn authority(&self) -> WindowConfigAuthorityIdentity { self.authority }
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> [u8; 32] {
        self.revision
    }

    pub fn window_id(&self) -> &str {
        &self.window_id
    }

    pub fn window_kind_id(&self) -> &str {
        self.window_kind_id
    }

    pub fn get<O: WindowConfigOwner>(&self) -> Option<&O::State> {
        (self.window_kind_id == O::WINDOW_KIND_ID).then(|| self.snapshot.get::<O::State>()).flatten()
    }
}
