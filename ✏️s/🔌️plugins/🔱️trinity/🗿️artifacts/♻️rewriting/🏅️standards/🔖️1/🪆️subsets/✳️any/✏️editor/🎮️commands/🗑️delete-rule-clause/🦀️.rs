//! 📜️ Trinity Rewriting app command — `delete-rule-clause`.

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::mutations::{edit_lhs, edit_rhs, remove_parameter_binding, remove_rule_layout_point};
use crate::standards::v1::subsets::any::schema::{self, Rhs};
use crate::RewritingSnapshot;

/// 🧭️ One addressable rule-clause node in the LHS/RHS semantic graphs (`lhs-where`, `rhs-create-N`,
/// `rhs-merge-N`, `rhs-set-N`, `rhs-delete-N`, `rhs-parameter-N`) — parsed back from its synthetic
/// node id by `parse_clause_ref`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

fn remove_at<T>(items: &mut Vec<T>, index: usize) -> Option<T> {
    (index < items.len()).then(|| items.remove(index))
}

/// 🗑️ The leaves that drop the rule clauses `node_ids` address: ONE `edit-lhs` and/or `edit-rhs` carrying each side without
/// them, a `remove-parameter-binding` per dropped parameter clause that had a binding, and a `remove-rule-layout-point` per
/// dropped clause that had a layout point. Clauses of one list are dropped from the highest index down, so every id names the
/// clause it named in the committed rule; ids that name no clause, and an undecodable rule, drop nothing.
pub(crate) fn delete_rule_clauses(state: &RewritingSnapshot, node_ids: &[String]) -> Vec<RewriteRuleMutation> {
    let mut lhs=state.lhs.clone();
    let mut rhs=state.rhs.clone();
    let mut clauses: Vec<(RuleClauseRef, &String)> = node_ids.iter().filter_map(|id| parse_clause_ref(id).map(|clause| (clause, id))).collect();
    clauses.sort_by(|left, right| right.0.cmp(&left.0));
    clauses.dedup_by(|left, right| left.0 == right.0);
    let (mut lhs_changed, mut rhs_changed, mut removed) = (false, false, Vec::new());
    for (clause, id) in clauses {
        let dropped = match clause {
            RuleClauseRef::LhsWhere => {
                lhs_changed |= lhs.where_clause.take().is_some();
                lhs_changed
            }
            RuleClauseRef::RhsCreate(index) => remove_at(&mut rhs.create, index).is_some(),
            RuleClauseRef::RhsMerge(index) => remove_at(&mut rhs.merge, index).is_some(),
            RuleClauseRef::RhsSet(index) => remove_at(&mut rhs.set, index).is_some(),
            RuleClauseRef::RhsDelete(index) => remove_at(&mut rhs.delete, index).is_some(),
            RuleClauseRef::RhsParameter(index) => match remove_at(&mut rhs.parameters, index) {
                Some(parameter) => {
                    removed.extend(state.parameter_bindings.contains_key(&parameter.name).then(|| remove_parameter_binding(parameter.name)));
                    true
                }
                None => false,
            },
        };
        rhs_changed |= dropped && clause != RuleClauseRef::LhsWhere;
        removed.extend((dropped && state.rule_layout.contains_key(id)).then(|| remove_rule_layout_point(id.clone())));
    }
    let sides = [lhs_changed.then(|| edit_lhs(lhs)), rhs_changed.then(|| edit_rhs(rhs))];
    sides.into_iter().flatten().chain(removed).collect()
}
