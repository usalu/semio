//! 👁️ 👁️ Imperative play app commands command — `run`.

use crate::artifacts::procedure::mutations::ProcedureMutation;
use crate::artifacts::procedure::ProcedureSnapshot;
use crate::editor::procedure::config::ImperativeConfigMutation;
use crate::editor::procedure::engine::ImperativeHost;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

use crate::editor::procedure::config::ImperativeConfig;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "run")]
pub struct Run {}

pub fn handle(_payload: &Run, doc: &ArtifactView<'_, ProcedureSnapshot>, _cfg: &ConfigView<'_, ImperativeConfig>) -> Result<Emit<ProcedureMutation, ImperativeConfigMutation>, Fault> {
    let host = ImperativeHost::from_snapshot(doc.snapshot.clone());
    let result = host.run();
    let json = dsl::os_pack::json::to_json_string(&result.scope);
    Ok(Emit::config(vec![ImperativeConfigMutation::SetRunOutput { json }]))
}
