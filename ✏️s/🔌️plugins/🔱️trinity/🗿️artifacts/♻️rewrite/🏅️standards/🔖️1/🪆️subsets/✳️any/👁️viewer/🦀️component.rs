//! 👁️ Trinity Rewrite viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `TrinityRewriteViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<TrinityRewriteViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches`).

use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::{RewriteSnapshot, REWRITE_RULE_SCHEMA, TRINITY_REWRITE_DIALECT};
use crate::viewer::rewrite::modes::view;
use crate::viewer::rewrite::modes::view::windows::rule;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TrinityRewriteViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for TrinityRewriteViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(TrinityRewriteViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct TrinityRewriteViewer;

/// 👁️ Read-only initial rule state — the empty/default snapshot (no pattern, no bound parameters, no
/// working fixture), distinct from the editor's `default_rule_state()` (Nakagin fixture + seeded
/// `label-core` demo rule): a fresh viewer session has no editor-authored rule to show yet.
async fn empty_rule_state() -> RewriteSnapshot {
    RewriteSnapshot::default()
}

impl ArtifactViewer for TrinityRewriteViewer {
    type Snapshot = RewriteSnapshot;
    type Mutation = RewriteRuleMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = TrinityRewriteViewCommand;

    const DIALECT: Dialect = TRINITY_REWRITE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = REWRITE_RULE_SCHEMA;

    async fn initial_snapshot() -> RewriteSnapshot {
        empty_rule_state()
    }

    /// 👁️ Structurally read-only: the sole `TrinityRewriteViewCommand::Noop` variant never carries a
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
            rule::BODY_KEY => rule::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_trinity_rewrite_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(TRINITY_REWRITE_DIALECT)
        .document(["semio", "trinity", "rewrite"])
        .icon_id("trinity-rewrite")
        .mode_def(view::definition())
        .default_mode_id(view::TRINITY_REWRITE_VIEW_MODE_VIEW)
        .window_kind_def(rule::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_trinity_rewrite_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_trinity_rewrite_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, TRINITY_REWRITE_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<TrinityRewriteViewer as ArtifactViewer>::DIALECT, TRINITY_REWRITE_DIALECT);
    }
}
//#endregion 🧪️Tests
