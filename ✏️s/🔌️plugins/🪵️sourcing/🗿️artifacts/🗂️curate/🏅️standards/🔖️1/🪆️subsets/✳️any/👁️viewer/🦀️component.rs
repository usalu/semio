//! 👁️ Sourcing viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `SourcingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<SourcingViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::curate::{CurateSnapshot, SOURCING_CURATE_SCHEMA, SOURCING_DIALECT};
use crate::viewer::sourcing::modes::view;
use crate::viewer::sourcing::modes::view::windows::pool;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🚧️ SDK GAP: same note as the editor's own import block — `InteractionView` is only reachable
// through `app`, not yet in the crate-root re-export list (w0-f Gap 1 only closed the surface
// traits/builders/adapters, not this pre-existing type).
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
/// `Default` is required by `testkit::assert_viewer_never_mutates::<V>()` (contract §2.5) to
/// synthesize a representative command with zero caller-supplied arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SourcingViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for SourcingViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SourcingViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct SourcingViewer;

impl ArtifactViewer for SourcingViewer {
    /// 📜️ Snapshot/decode-only Mutation are the SAME artifact-level types the sibling editor uses
    /// (contract §2.2) — they already live outside both surfaces, under `crate::artifacts::curate`.
    type Snapshot = CurateSnapshot;
    type Mutation = crate::artifacts::curate::SourcingMutation;
    /// 👁️ A viewer needs no persisted per-session state to render a read-only catalogue table —
    /// framework `NoConfig`/`NoPresence`/`NoTransient` throughout, an intentional simplification, not
    /// a bug (mirrors the cad pilot's viewer, contract §2.2/§8).
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = SourcingViewCommand;

    const DIALECT: Dialect = SOURCING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SOURCING_CURATE_SCHEMA;

    async fn initial_snapshot() -> CurateSnapshot {
        crate::artifacts::curate::schema::default_document()
    }

    /// 👁️ Structurally read-only: the sole `SourcingViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g. a
    /// local sort/search toggle) is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            pool::BODY_KEY => pool::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_sourcing_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SOURCING_DIALECT)
        .document(["semio", "sourcing", "curate"])
        .icon_id("library")
        .mode_def(view::definition())
        .default_mode_id(view::SOURCING_VIEW_MODE_VIEW)
        .window_kind_def(pool::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_sourcing_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_sourcing_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, SOURCING_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SourcingViewer as ArtifactViewer>::DIALECT, SOURCING_DIALECT);
    }
}
//#endregion 🧪️Tests
