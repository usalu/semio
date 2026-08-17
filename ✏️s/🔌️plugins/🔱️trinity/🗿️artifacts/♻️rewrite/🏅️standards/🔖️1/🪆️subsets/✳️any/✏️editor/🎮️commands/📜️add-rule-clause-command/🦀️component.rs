//! 📜️ 📜️ Trinity Rewrite app command — `add-rule-clause-command`.

use crate::editor::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::PropertyValue;
use crate::artifacts::rewrite::schema::{ParameterKind, Rhs};
use crate::artifacts::rewrite::mutations::rewrite_snapshot_mutations;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

fn add_rule_clause(state: &mut RewriteSnapshot, clause_kind: &str) -> bool {
    let Ok(mut lhs) = serde_json::from_str::<crate::artifacts::rewrite::schema::Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = serde_json::from_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let left_var = lhs.pattern.left_var.clone();
    let changed = match clause_kind {
        "where" => {
            if lhs.where_clause.is_some() {
                false
            } else {
                lhs.where_clause = Some(format!("{left_var}.name = 'value'"));
                true
            }
        }
        "create" => {
            rhs.create.push(crate::artifacts::rewrite::schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "merge" => {
            rhs.merge.push(crate::artifacts::rewrite::schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "set" => {
            rhs.set.push(crate::artifacts::rewrite::schema::AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
            true
        }
        "delete" => {
            rhs.delete.push(left_var);
            true
        }
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(crate::artifacts::rewrite::schema::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
            true
        }
        _ => false,
    };
    if changed {
        state.lhs_json = serde_json::to_string(&lhs).unwrap_or_default();
        state.rhs_json = serde_json::to_string(&rhs).unwrap_or_default();
    }
    changed
}
pub(crate) fn add_rule_clause_command(state: &RewriteSnapshot, kind: &str) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    let mut next = state.clone();
    if add_rule_clause(&mut next, kind) {
        Ok(Emit::mutations(rewrite_snapshot_mutations(state, &next)))
    } else {
        Ok(Emit::default())
    }
}
