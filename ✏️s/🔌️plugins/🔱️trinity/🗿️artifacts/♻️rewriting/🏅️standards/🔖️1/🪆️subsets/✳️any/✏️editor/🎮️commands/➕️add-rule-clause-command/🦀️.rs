//! 📜️ 📜️ Trinity Rewriting app command — `add-rule-clause-command`.

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::mutations::{change_parameter_binding, edit_lhs, edit_rhs};
use crate::standards::v1::subsets::any::schema::{self, ParameterKind, Rhs};
use crate::RewritingSnapshot;
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// ➕️ The leaves that add one clause of `clause_kind` to the rule: `edit-lhs` for a WHERE clause, `edit-rhs` for every other
/// clause (plus the new parameter's `change-parameter-binding`), so the edit carries no re-printed copy of the untouched side. A
/// request that cannot move the rule is refused by name — a second WHERE clause, an unknown clause kind, or a rule whose own JSON
/// no longer decodes — instead of answering an empty emit that read as an accepted edit (S15).
fn add_rule_clause(state: &RewritingSnapshot, clause_kind: &str) -> Result<Vec<RewriteRuleMutation>, Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    let mut lhs=state.lhs.clone();
    let mut rhs=state.rhs.clone();
    let left_var = lhs.pattern.left_var.clone();
    let mut bindings = Vec::new();
    match clause_kind {
        "where" if lhs.where_clause.is_some() => return Err(invalid("the rule already has a WHERE clause".into())),
        "where" => {
            lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
            return Ok(vec![edit_lhs(lhs)]);
        }
        "create" => rhs.create.push(schema::Pattern { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }),
        "merge" => rhs.merge.push(schema::Pattern { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }),
        "set" => rhs.set.push(schema::Assignment { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) }),
        "delete" => rhs.delete.push(left_var),
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            bindings.push(change_parameter_binding(name.clone(), PropertyValue::String(String::new())));
            rhs.parameters.push(schema::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
        }
        other => return Err(invalid(format!("unknown rule clause kind '{other}' (where, create, merge, set, delete, parameter)"))),
    }
    Ok(std::iter::once(edit_rhs(rhs)).chain(bindings).collect())
}

/// ➕️ The `addRuleClause` verb: one edit adding the requested clause, or a named refusal.
pub(crate) fn add_rule_clause_command(state: &RewritingSnapshot, kind: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(add_rule_clause(state, kind)?))
}
