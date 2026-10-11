//! 🕸️ 🕸️ DAG play app commands command — `delete-selection`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::{DagMutation, DagSnapshot};
use semio_framework_plugin::{app::InteractionView, ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` is framework-fixed at this exact 3-arg shape (no
/// `interaction` slot — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — reachable only
/// through that macro-generated path (`DagPlayApp::handle` always routes this command through `apply`
/// below instead), so it degrades to treating the selection as empty, matching `space`'s identical
/// `delete_selection` split.
pub fn handle(payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let _ = cfg;
    apply_to(payload, doc, &[])
}

pub fn apply(payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, interaction: &InteractionView<'_>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    apply_to(payload, doc, &interaction.selection("graph").ids)
}

/// 🧵️ The retained-tool twin of [`apply`]: a bounded tool-job reducer is handed the raw
/// `protocol::InteractionState`, and `InteractionView`'s fields are framework-private, so the domain
/// selection is read straight off the state instead of being wrapped first.
pub fn apply_with_state(payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>, interaction: &protocol::InteractionState) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let selected = interaction.selection.get("graph").map(|domain| domain.ids.clone()).unwrap_or_default();
    apply_to(payload, doc, &selected)
}

/// 🗑️ The selected nodes leave the `content` child with every edge they hold (edges first, each row point-invertible); the
/// framework prunes the deleted ids out of `graph`'s selection via `DagPlayApp::interaction_topology`.
fn apply_to(_payload: &DeleteSelection, doc: &ArtifactView<'_, DagSnapshot>, selected: &[String]) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(crate::dag_child_emit(doc.snapshot, crate::schema::remove_nodes_leaves(&crate::dag_scene(doc)?, selected)))
}
