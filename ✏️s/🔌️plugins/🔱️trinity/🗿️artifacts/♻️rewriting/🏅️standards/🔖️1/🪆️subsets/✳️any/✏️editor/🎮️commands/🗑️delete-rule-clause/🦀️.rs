//! 📜️ Trinity Rewriting app command — `delete-rule-clause`.

use crate::standards::v1::subsets::any::schema::{self, Rhs};
use crate::RewritingSnapshot;

/// 🧭️ One addressable rule-clause node in the LHS/RHS semantic graphs (`lhs-where`, `rhs-create-N`,
/// `rhs-merge-N`, `rhs-set-N`, `rhs-delete-N`, `rhs-parameter-N`) — parsed back from its synthetic
/// node id by `parse_clause_ref`.
enum RuleClauseRef {
    LhsWhere,
    RhsCreate(usize),
    RhsMerge(usize),
    RhsSet(usize),
    RhsDelete(usize),
    RhsParameter(usize),
}

fn parse_clause_ref(node_id: &str) -> Option<RuleClauseRef> {
    if node_id == "lhs-where" {
        return Some(RuleClauseRef::LhsWhere);
    }
    let (prefix, index) = node_id.rsplit_once('-')?;
    let index: usize = index.parse().ok()?;
    match prefix {
        "rhs-create" => Some(RuleClauseRef::RhsCreate(index)),
        "rhs-merge" => Some(RuleClauseRef::RhsMerge(index)),
        "rhs-set" => Some(RuleClauseRef::RhsSet(index)),
        "rhs-delete" => Some(RuleClauseRef::RhsDelete(index)),
        "rhs-parameter" => Some(RuleClauseRef::RhsParameter(index)),
        _ => None,
    }
}
fn remove_at<T>(items: &mut Vec<T>, index: usize) -> bool {
    if index < items.len() {
        items.remove(index);
        true
    } else {
        false
    }
}

/// 🗑️ Drops the rule clause addressed by `node_id` from the LHS/RHS JSON carried by `state`,
/// together with its layout point and — for a parameter clause — its binding.
pub(crate) fn delete_rule_clause(state: &mut RewritingSnapshot, node_id: &str) -> bool {
    let Some(clause_ref) = parse_clause_ref(node_id) else {
        return false;
    };
    let Ok(mut lhs) = pack::from_json_str::<schema::Lhs>(&state.lhs_json) else {
        return false;
    };
    let Ok(mut rhs) = pack::from_json_str::<Rhs>(&state.rhs_json) else {
        return false;
    };
    let changed = match clause_ref {
        RuleClauseRef::LhsWhere => {
            let had = lhs.where_clause.is_some();
            lhs.where_clause = None;
            had
        }
        RuleClauseRef::RhsCreate(index) => remove_at(&mut rhs.create, index),
        RuleClauseRef::RhsMerge(index) => remove_at(&mut rhs.merge, index),
        RuleClauseRef::RhsSet(index) => remove_at(&mut rhs.set, index),
        RuleClauseRef::RhsDelete(index) => remove_at(&mut rhs.delete, index),
        RuleClauseRef::RhsParameter(index) => {
            if index < rhs.parameters.len() {
                let removed = rhs.parameters.remove(index);
                state.parameter_bindings.remove(&removed.name);
                true
            } else {
                false
            }
        }
    };
    if changed {
        state.lhs_json = pack::to_json_string(&lhs);
        state.rhs_json = pack::to_json_string(&rhs);
        state.rule_layout.remove(node_id);
    }
    changed
}
