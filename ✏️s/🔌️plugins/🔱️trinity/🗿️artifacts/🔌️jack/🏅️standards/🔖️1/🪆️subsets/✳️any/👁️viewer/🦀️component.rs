//! 👁️ Trinity Jack viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `TrinityJackViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<TrinityJackViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::{empty_trinity_graph_fixture, JackSnapshot, TRINITY_GRAPH_SCHEMA, TRINITY_JACK_DIALECT};
use crate::viewer::jack::modes::view;
use crate::viewer::jack::modes::view::windows::graph;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrinityJackViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for TrinityJackViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(TrinityJackViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct TrinityJackViewer;

impl ArtifactViewer for TrinityJackViewer {
    type Snapshot = JackSnapshot;
    type Mutation = TrinityGraphMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TrinityJackViewCommand;

    const DIALECT: Dialect = TRINITY_JACK_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = TRINITY_GRAPH_SCHEMA;

    async fn initial_snapshot() -> JackSnapshot {
        empty_trinity_graph_fixture()
    }

    /// 👁️ Structurally read-only: the sole `TrinityJackViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is
    /// a pure addition here, never a signature change.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            graph::BODY_KEY => graph::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_trinity_jack_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(TRINITY_JACK_DIALECT)
        .document(["semio", "trinity", "jack"])
        .icon_id("trinity")
        .mode_def(view::definition())
        .default_mode_id(view::TRINITY_JACK_VIEW_MODE_VIEW)
        .window_kind_def(graph::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_trinity_jack_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_trinity_jack_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, TRINITY_JACK_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TrinityJackViewer as ArtifactViewer>::DIALECT, TRINITY_JACK_DIALECT);
    }
}
//#endregion 🧪️Tests
