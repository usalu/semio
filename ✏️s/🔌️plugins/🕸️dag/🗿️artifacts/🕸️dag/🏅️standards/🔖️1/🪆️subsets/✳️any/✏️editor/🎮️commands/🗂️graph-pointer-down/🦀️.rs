//! 🗂️ 🗂️ DAG play app commands command — `graph-pointer-down`.

use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[dsl(keyword = "graph-pointer-down")]
pub struct GraphPointerDown {}

/// 🕹️ No longer clears the `graph` selection directly — no `Emit` channel writes it anymore (the
/// framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). A bare
/// pointer-down on empty canvas now dispatches the framework's own `clearSelection` action instead;
/// this row survives as a genuine no-op purely for wire/dispatch compatibility with the surface's
/// existing pointer-down event, mirroring `procedural3d`'s identical `graph-pointer-down` stub.
pub async fn handle(_payload: &GraphPointerDown, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::default())
}
