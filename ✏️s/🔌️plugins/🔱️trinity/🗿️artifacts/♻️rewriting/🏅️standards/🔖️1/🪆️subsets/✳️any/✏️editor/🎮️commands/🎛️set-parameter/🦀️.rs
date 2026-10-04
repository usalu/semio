//! 📜️ 📜️ Trinity Rewriting app command — `set-parameter`.

use crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{ParameterKind, Rhs};
use crate::RewritingSnapshot;
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_plugin::Emit;
use semio_framework_plugin::NoConfigMutation;

/// 🎛️ One rule parameter's bound value, read by its declared kind, as ONE `change-parameter-binding {key, newValue}` — the
/// field-parametric leaf history edits; a value the binding already holds, an unknown name's unparseable number or an empty
/// name moves nothing. The parameters window commits it once per field, on blur or Enter (design §13.2).
pub(crate) fn set_parameter(state: &RewritingSnapshot, name: &str, value: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    if name.is_empty() {
        return Emit::default();
    }
    let kind = state.rhs.parameters.iter().find(|param| param.name == name).map(|param| param.kind.clone());
    let parsed = match kind {
        Some(ParameterKind::Number) => value.parse::<f64>().ok().map(PropertyValue::Number),
        Some(ParameterKind::Boolean) => Some(PropertyValue::Bool(value.eq_ignore_ascii_case("true"))),
        Some(ParameterKind::String) | None => Some(PropertyValue::String(value.to_string())),
    };
    match parsed {
        Some(parsed) if state.parameter_bindings.get(name) != Some(&parsed) => Emit::mutations(vec![change_parameter_binding(name.to_string(), parsed)]),
        _ => Emit::default(),
    }
}
