//! 📜️ 📜️ Trinity Rewriting app command — `set-parameter`.

use crate::artifacts::jack::PropertyValue;
use crate::artifacts::rewriting::rewriting_snapshot_mutations;
use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::artifacts::rewriting::schema::{ParameterKind, Rhs};
use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn set_parameter(state: &RewritingSnapshot, name: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    if name.is_empty() {
        return Ok(Emit::default());
    }
    let Ok(rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) else {
        return Ok(Emit::default());
    };
    let kind = rhs.parameters.iter().find(|param| param.name == name).map(|param| param.kind.clone());
    let parsed = match kind {
        Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
        Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
        Some(ParameterKind::String) | None => Some(PropertyValue::String(value.to_string())),
    };
    match parsed {
        Some(parsed) => {
            let mut next = state.clone();
            next.parameter_bindings.insert(name.to_string(), parsed);
            Ok(Emit::mutations(rewriting_snapshot_mutations(state, &next)))
        }
        None => Ok(Emit::default()),
    }
}
