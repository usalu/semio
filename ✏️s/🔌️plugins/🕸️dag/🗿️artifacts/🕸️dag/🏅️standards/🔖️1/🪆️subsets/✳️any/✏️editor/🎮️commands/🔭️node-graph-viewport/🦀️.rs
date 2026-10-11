//! 🗂️ 🗂️ DAG play app commands command — `node-graph-viewport`.

use crate::editor::dag::config::{DagConfig, DagConfigMutation};
use crate::DagMutation;
use crate::DagSnapshot;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "node-graph-viewport")]
pub struct NodeGraphViewport {
    #[dsl(block)]
    pub viewport: Viewport2d,
}

pub fn handle(payload: &NodeGraphViewport, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    Ok(Emit::config(vec![DagConfigMutation::ChangeCamera(crate::editor::dag::config::ChangeCamera {
        x: payload.viewport.x,
        y: payload.viewport.y,
        zoom: payload.viewport.zoom,
    })]))
}
