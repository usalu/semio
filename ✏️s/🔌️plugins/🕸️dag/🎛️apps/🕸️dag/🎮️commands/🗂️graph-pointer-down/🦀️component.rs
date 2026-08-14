//! 🗂️ 🗂️ DAG play app commands command — `graph-pointer-down`.

use crate::apps::dag::config::{DagConfig, DagConfigMutation};
use crate::artifacts::dag::op::DagMutation;
use crate::artifacts::dag::DagSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "graph-pointer-down")]
pub struct GraphPointerDown {}

/// 🕹️ No longer clears the `graph` selection directly — no `Emit` channel writes it anymore (the
/// framework owns it exclusively; ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). A bare
/// pointer-down on empty canvas now dispatches the framework's own `clearSelection` action instead;
/// this row survives as a genuine no-op purely for wire/dispatch compatibility with the surface's
/// existing pointer-down event, mirroring `procedural3d`'s identical `graph-pointer-down` stub.
pub fn handle(_payload: &GraphPointerDown, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::default())
}
