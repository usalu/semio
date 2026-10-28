//! 🧾️ Bounded proof and factory registry of the tool job contract: the bounded first step proof, the publication lane and contract, the owned tool job factory, the registration and the factory registry. See [`super`].

use super::*;

pub const BOUNDED_FIRST_STEP_FACTORY: &str = "BoundedFirstStepCommandJobFactory";

/// 📜️ One owner-local bounded reducer proof. The constructor derives the concrete Rust
/// owner witness; copied controller, schema, file, or tool strings cannot grant authority.
#[derive(Clone,Copy)]
pub struct ArtifactBoundedFirstStepProof {
    pub owner_file: &'static str,
    pub owner: ToolOwnerWitness,
    pub controller_id: &'static str,
    pub factory: &'static str,
    pub factory_type_id: Option<std::any::TypeId>,
    pub factory_type_name: Option<&'static str>,
    pub tool_id: &'static str,
    pub artifact_schema: &'static str,
    pub contract: semio_framework::ToolExecutionContract,
}

impl ArtifactBoundedFirstStepProof {
    /// 🏷️ Identifies the tool described by this immutable proof.
    pub fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    pub fn new<A: ArtifactApp>(owner_file: &'static str, controller_id: &'static str, factory: &'static str, tool_id: &'static str, artifact_schema: &'static str, contract: semio_framework::ToolExecutionContract) -> Self {
        Self { owner_file, owner: ToolOwnerWitness::of::<A>(), controller_id, factory, factory_type_id: None, factory_type_name: None, tool_id, artifact_schema, contract }
    }

    pub fn with_factory_type<A: ArtifactApp, F: ArtifactOwnedToolJobFactory<Owner = A>>(mut self) -> Self {
        assert_eq!(self.owner, ToolOwnerWitness::of::<A>(), "proof factory belongs to its exact concrete app owner");
        self.factory_type_id = Some(std::any::TypeId::of::<F>());
        self.factory_type_name = Some(std::any::type_name::<F>());
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactToolPublicationLane {
    HostOnly,
    Artifact,
    Config,
    Draft,
    Presence,
    Transient,
    WindowConfig,
    WindowTransient,
    Child,
    /// 🕹️ This tool's emit may carry [`InteractionWrite`]s — the framework-owned selection lane
    /// (`interaction_store`, `HistoryLane::Interaction`). Never a document/config lane: it is
    /// persisted-local and excluded from undo/redo by declaration.
    Interaction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactToolPublicationContract {
    pub tool_id: &'static str,
    pub lanes: &'static [ArtifactToolPublicationLane],
}

/// 🔐️ A plugin factory's concrete owner is part of its Rust type, so a copied `ArtifactApp`
/// cannot reuse the factory by copying controller/schema/tool constants.
pub trait ArtifactOwnedToolJobFactory: semio_framework::ToolJobFactory {
    type Owner: ArtifactApp;
    const TOOL_IDS: &'static [&'static str];
    const DOCUMENT_SCHEMA: &'static str;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[];

    fn latest_wins_target(_command: &<Self::Owner as ArtifactApp>::Command) -> Option<&str> {
        None
    }

    fn build_latest_wins_command_disposer() -> ArtifactDisposal<<Self::Owner as ArtifactApp>::Command> {
        None
    }
}

#[derive(Clone)]
pub struct ArtifactToolRegistration {
    pub owner: ToolOwnerWitness,
    pub key: semio_framework::ToolFactoryKey,
    pub schema_id: String,
    pub contract: semio_framework::ToolExecutionContract,
    pub factory_type_id: std::any::TypeId,
    pub factory_type_name: &'static str,
    pub publication_lanes: &'static [ArtifactToolPublicationLane],
    pub latest_wins_target: for<'a> fn(&'a dyn std::any::Any) -> Option<&'a str>,
    pub latest_wins_command_disposer: fn() -> Option<Box<dyn std::any::Any + Send>>,
}

/// 🧾️ Public predecode projection of one exact app-contributed registration. It contains no
/// factory handle and therefore cannot grant execution authority by itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactToolPublicContract {
    pub owner: ToolOwnerWitness,
    pub controller_id: String,
    pub tool_id: String,
    pub schema_id: String,
    pub max_raw_wire_bytes: usize,
    pub publication_lanes: &'static [ArtifactToolPublicationLane],
}

/// 🧰️ Compiler-bound registration context passed only to `ArtifactApp`'s explicit hook.
pub struct ArtifactToolFactoryRegistry<'a, A: ArtifactApp> {
    bus: &'a semio_framework::ActionBus,
    pub(super) controller_id: &'a str,
    registrations: BTreeMap<String, ArtifactToolRegistration>,
    marker: std::marker::PhantomData<A>,
}

impl<'a, A: ArtifactApp> ArtifactToolFactoryRegistry<'a, A> {
    pub fn new(bus: &'a semio_framework::ActionBus, controller_id: &'a str) -> Self {
        Self { bus, controller_id, registrations: BTreeMap::new(), marker: std::marker::PhantomData }
    }

    pub fn controller_id(&self) -> &str {
        self.controller_id
    }

    pub fn register<F>(&mut self, factory: F) -> Result<(), Fault>
    where
        F: ArtifactOwnedToolJobFactory<Owner = A>,
    {
        if F::DOCUMENT_SCHEMA != A::DOCUMENT_SCHEMA || factory.payload_schema_id().trim().is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-schema"), "app-owned factory schema does not match its concrete ArtifactApp owner"));
        }
        let expected = F::TOOL_IDS.iter().copied().collect::<BTreeSet<_>>();
        let actual = factory.keys().iter().map(|key| key.tool_id.as_str()).collect::<BTreeSet<_>>();
        if expected.is_empty() || expected != actual || factory.keys().iter().any(|key| key.controller_id != self.controller_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-key"), "app-owned factory keys are not an exact live-controller/tool bijection"));
        }
        let publication = F::PUBLICATION_CONTRACTS.iter().map(|contract| (contract.tool_id, contract.lanes)).collect::<BTreeMap<_, _>>();
        if publication.len() != F::PUBLICATION_CONTRACTS.len()
            || publication.keys().copied().collect::<BTreeSet<_>>() != expected
            || publication.values().any(|lanes| lanes.is_empty() || (lanes.contains(&ArtifactToolPublicationLane::HostOnly) && lanes.len() != 1))
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-contract"), "app-owned factory lacks an exact nonempty per-tool publication-lane contract"));
        }
        if factory.classification() != semio_framework::InteractiveJobClassification::Migrated {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-classification"), "app-owned factory must be explicitly Migrated"));
        }
        let owner = ToolOwnerWitness::of::<A>();
        let schema_id = factory.payload_schema_id().to_string();
        let contract = factory.execution_contract();
        let factory_type_id = std::any::TypeId::of::<F>();
        let factory_type_name = std::any::type_name::<F>();
        let keys = factory.keys().to_vec();
        self.bus.register_once(factory).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-registration"), error.to_string()))?;
        for key in keys {
            if self.registrations.contains_key(&key.tool_id) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.owner-duplicate"), format!("duplicate app-owned tool {}", key.tool_id)));
            }
            let publication_lanes = publication.get(key.tool_id.as_str()).copied().expect("validated exact tool publication contract");
            self.registrations.insert(
                key.tool_id.clone(),
                ArtifactToolRegistration {
                    owner,
                    key,
                    schema_id: schema_id.clone(),
                    contract,
                    factory_type_id,
                    factory_type_name,
                    publication_lanes,
                    latest_wins_target: |command| command.downcast_ref::<<F::Owner as ArtifactApp>::Command>().and_then(F::latest_wins_target),
                    latest_wins_command_disposer: || F::build_latest_wins_command_disposer().map(|owner| Box::new(owner) as Box<dyn std::any::Any + Send>),
                },
            );
        }
        Ok(())
    }

    pub fn finish(self) -> BTreeMap<String, ArtifactToolRegistration> {
        self.registrations
    }
}

impl semio_framework_value::retirement::RetireOwned for ArtifactToolRegistration{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;(self.key,self.schema_id).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes::<semio_framework::ToolFactoryKey>(),semio_framework_value::retirement::deferred_birth_bytes::<String>()])}
 fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value::retirement::RetireOwned for ArtifactToolPublicContract{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{use semio_framework_value::retirement::RetireOwned;(self.controller_id,self.tool_id,self.schema_id).retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes::<String>();3])}
 fn controlled_retirement_supported()->bool{true}
}
