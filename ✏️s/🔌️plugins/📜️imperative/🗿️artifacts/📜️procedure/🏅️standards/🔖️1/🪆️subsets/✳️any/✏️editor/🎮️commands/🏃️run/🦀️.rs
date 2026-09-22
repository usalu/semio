//! 👁️ 👁️ Imperative play app commands command — `run`.

use crate::editor::procedure::config::ImperativeConfigMutation;
use crate::editor::procedure::engine::ImperativeHost;
use crate::mutations::ProcedureMutation;
use crate::ProcedureSnapshot;
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
    // 🧊️ The run result owns the accumulated scope and every effect row's input/output
    // dictionaries; a bare drop here aborts the guest with `final Dictionary ownership must be
    // explicitly retired or owned by a cold boundary` (bucket XCUT-DICT).
    neural_engine::ColdRetire::retire_cold(result);
    Ok(Emit::config(vec![ImperativeConfigMutation::SetRunOutput(crate::editor::procedure::config::SetRunOutput { json })]))
}
