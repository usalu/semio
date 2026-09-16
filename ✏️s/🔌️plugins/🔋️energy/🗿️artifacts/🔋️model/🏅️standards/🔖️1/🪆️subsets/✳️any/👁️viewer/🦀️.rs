//! 👁️ Energy model viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EnergyModelViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EnergyModelViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact mutation.
//! Must not import anything from the sibling mutation-capable surface (`policyViewerPurityBreaches`).

use crate::viewer::model::modes::view;
use crate::viewer::model::modes::view::windows::{simulation, structure, zones};
use crate::{EnergyModelMutation, EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiAssemblyResult, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor`'s typed command
/// enum carries them would be pure ceremony for a surface that never dispatches anything through
/// `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EnergyModelViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for EnergyModelViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(EnergyModelViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EnergyModelViewer;

impl ArtifactViewer for EnergyModelViewer {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelViewCommand;
    /// 🧩️ Read-only twin of the editor's roster: `structure`/`zones` are stdio `value`/`table` members.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        crate::energy_child_restore_projection(snapshot)
    }

    /// 🌱️ Both composed children derive from the model — see `crate::energy_genesis_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::energy_genesis_child_pack(snapshot, slot, child_id)
    }

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    /// 🛂️ A viewer declares every nontrivial store owner explicitly (the trait defaults fail closed);
    /// the document store is the bounded pair the editor also uses, and every other lane is `No*`.
    /// Without these the two genesis members can never be retired (`interactive-job.close-owned-
    /// disposer-missing`).
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 👁️ Structurally read-only: the sole `EnergyModelViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit`. Kept as a real dispatch (not
    /// `unreachable!()`) so a future view-only action is a pure addition, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        let node = match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot)?,
            zones::BODY_KEY => zones::render(doc.snapshot)?,
            simulation::BODY_KEY => simulation::render(&doc.snapshot.model),
            _ => {
                semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.viewer.render", "the unknown-body label could not be assembled"))?
            }
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_energy_model_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(view::definition())
        .default_mode_id(view::ENERGY_MODEL_VIEW_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
