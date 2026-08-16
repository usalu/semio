//! 📜️ 📜️ Trinity Rewrite app command — `set-parameter`.

use crate::editor::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::{Graph, JackSnapshot, PropertyValue};
use crate::artifacts::rewrite::schema::{ParameterKind, Rhs};
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};
use serde_json::Value;

pub(crate) fn set_parameter(state: &RewriteSnapshot, name: &str, value: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    if name.is_empty() {
        return Ok(Emit::default());
    }
    let Ok(rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
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
            Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
        }
        None => Ok(Emit::default()),
    }
}
