//! 📜️ 📜️ Trinity Rewriting app command — `add-rule-clause-command`.

use semio_s_artifact_trinity_jack::PropertyValue;
use crate::rewriting_snapshot_mutations;
use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::{ParameterKind, Rhs};
use crate::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

fn add_rule_clause(state: &mut RewritingSnapshot, clause_kind: &str) -> bool {
    let Ok(mut lhs) = pack::from_json_str::<schema::Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) else {
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
            rhs.create.push(schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "merge" => {
            rhs.merge.push(schema::PatternJson { left_var: "n".into(), left_kind: "Piece".into(), edge_var: None, edge_kind: None, right_var: None, right_kind: None });
            true
        }
        "set" => {
            rhs.set.push(schema::AssignmentJson { var: left_var, prop: "label".into(), value: PropertyValue::String(String::new()) });
            true
        }
        "delete" => {
            rhs.delete.push(left_var);
            true
        }
        "parameter" => {
            let name = format!("param{}", rhs.parameters.len());
            state.parameter_bindings.insert(name.clone(), PropertyValue::String(String::new()));
            rhs.parameters.push(schema::ParameterSpec { name, kind: ParameterKind::String, default: PropertyValue::String(String::new()) });
            true
        }
        _ => false,
    };
    if changed {
        state.lhs_json = pack::to_json_string(&lhs);
        state.rhs_json = pack::to_json_string(&rhs);
    }
    changed
}
pub(crate) fn add_rule_clause_command(state: &RewritingSnapshot, kind: &str) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    let mut next = state.clone();
    if add_rule_clause(&mut next, kind) {
        Emit::mutations(rewriting_snapshot_mutations(state, &next))
    } else {
        Emit::default()
    }
}
