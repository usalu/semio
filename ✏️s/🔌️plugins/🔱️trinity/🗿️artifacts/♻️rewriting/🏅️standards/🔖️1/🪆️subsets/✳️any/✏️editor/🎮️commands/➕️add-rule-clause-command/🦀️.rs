//! 📜️ 📜️ Trinity Rewriting app command — `add-rule-clause-command`.

use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{self, ParameterKind, Rhs};
use crate::RewritingSnapshot;
use semio_framework_graph::manifest::PropertyValue;
use semio_framework_plugin::{Emit, Fault, FaultCode, FaultOrigin, NoConfigMutation};

/// ➕️ Adds one clause of `clause_kind` to the rule and rewrites only the side it changed (a WHERE clause the
/// LHS, every other clause the RHS), so the edit carries no re-printed copy of the untouched side. A request
/// that cannot move the rule is refused by name — a second WHERE clause, an unknown clause kind, or a rule
/// whose own JSON no longer decodes — instead of answering an empty emit that read as an accepted edit (S15).
fn add_rule_clause(state: &mut RewritingSnapshot, clause_kind: &str) -> Result<(), Fault> {
    let invalid = |detail: String| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), detail);
    let undecodable = |side: &str, error: String| Fault::new(FaultOrigin::App, FaultCode::new("trinity.rewriting.rule-undecodable"), format!("the rule's {side} does not decode: {error}"));
    let mut lhs = pack::from_json_str::<schema::Lhs>(&state.lhs_json).map_err(|error| undecodable("LHS", error.to_string()))?;
    let mut rhs = pack::from_json_str::<Rhs>(&state.rhs_json).map_err(|error| undecodable("RHS", error.to_string()))?;
    let left_var = lhs.pattern.left_var.clone();
    match clause_kind {
        "where" if lhs.where_clause.is_some() => return Err(invalid("the rule already has a WHERE clause".into())),
        "where" => {
            lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
            state.lhs_json = pack::to_json_string(&lhs);
            return Ok(());
        }
        "create" => rhs.create.push(schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }),
        "merge" => rhs.merge.push(schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None }),
        "set" => rhs.set.push(schema::AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) }),
        "delete" => rhs.delete.push(left_var),
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(schema::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
        }
        other => return Err(invalid(format!("unknown rule clause kind '{other}' (where, create, merge, set, delete, parameter)"))),
    }
    state.rhs_json = pack::to_json_string(&rhs);
    Ok(())
}

/// ➕️ The `addRuleClause` verb: one edit adding the requested clause, or a named refusal.
pub(crate) fn add_rule_clause_command(state: &RewritingSnapshot, kind: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    let mut next = state.clone();
    add_rule_clause(&mut next, kind)?;
    Ok(Emit::mutations(rewriting_snapshot_mutations(state, &next)))
}
