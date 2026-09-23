//! 👁️ Note viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `NoteViewer` implements
//! `ArtifactViewer`, never the sibling `ArtifactEditor`/runtime `ArtifactApp` — `ViewerApp<NoteViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches` forbids it outright).

use crate::op::NoteMutation;
use crate::schema::empty_note_snapshot;
use crate::viewer::note::modes::view;
use crate::viewer::note::modes::view::windows::composite;
use crate::{NoteSnapshot, NOTE_DIALECT, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NoteViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for NoteViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(NoteViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct NoteViewer;

impl ArtifactViewer for NoteViewer {
    type Snapshot = NoteSnapshot;
    type Mutation = NoteMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = NoteViewCommand;

    const DIALECT: Dialect = NOTE_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::note_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::note_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = NOTE_DOCUMENT_SCHEMA;

    /// 🔐️ The document-store owner catalogue, identical to the sibling editor's: a viewer owns the
    /// very same `NoteSnapshot` envelope and must allocate and retire it the same way. Read-only
    /// says nothing about ownership.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    /// 🧹️ The four bounded disposers `VcsArtifactApp`'s close ladder drives, one per owned lane
    /// (`document-store`, `config-store`, `presence-store`, `transient-store`); `ViewerApp` supplies
    /// the draft lane itself.
    ///
    /// 🐛️ Left at the trait default (`None`) this surface can never close. Every close of a
    /// `ViewerApp<NoteViewer>` faults `interactive-job.close-owned-disposer-missing` ("app owner did
    /// not provide the required bounded disposer for document-store") and the store then reaches
    /// `Drop` without its terminal-empty witness — which on a `panic = "abort"` wasm32 guest is an
    /// `unreachable` that kills the whole instance. That is what took down every `codec.genesis`,
    /// `codec.pack-schema-hash`, `codec.print-mirror` and `codec.apply-ops` call against the note
    /// component (ticket 26/09/18 slices TC3c §5f, TC3d §1, TC3e): the codec resolver constructs
    /// every app of the bundle to read its schema and closes the ones it does not return, so the
    /// viewer's missing disposer faulted a call that never touched the viewer at all.
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
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

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn initial_snapshot() -> NoteSnapshot {
        empty_note_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `NoteViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g. a
    /// live pan/zoom) is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            composite::BODY_KEY => composite::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "note viewer diagnostic admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_note_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(NOTE_DIALECT).document(["semio", "note"]).icon_id("note").mode_def(view::definition()).default_mode_id(view::NOTE_VIEW_MODE_VIEW).window_kind_def(composite::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
