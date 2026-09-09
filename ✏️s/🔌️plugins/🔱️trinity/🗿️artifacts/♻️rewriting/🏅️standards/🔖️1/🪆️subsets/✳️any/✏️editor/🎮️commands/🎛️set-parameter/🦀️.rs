//! 📜️ 📜️ Trinity Rewriting app command — `set-parameter`.

use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{ParameterKind, Rhs};
use crate::RewritingSnapshot;
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

pub(crate) fn set_parameter(state: &RewritingSnapshot, name: &str, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    if name.is_empty() {
        return Emit::default();
    }
    let Ok(rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) else {
        return Emit::default();
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
            Emit::mutations(rewriting_snapshot_mutations(state, &next))
        }
        None => Emit::default(),
    }
}
