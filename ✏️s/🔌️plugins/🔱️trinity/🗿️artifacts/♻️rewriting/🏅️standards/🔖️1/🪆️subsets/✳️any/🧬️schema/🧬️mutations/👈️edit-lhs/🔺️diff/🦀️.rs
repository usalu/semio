//! 🔺️ Sparse diff builder for `EditLhs` — names only the pattern and where-clause fields the new left-hand side changes.
use crate::standards::v1::subsets::any::schema::diff::{LhsPatch, PatternPatch, RewritingDiff};
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditLhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let (next, current) = (&payload.new_lhs, &base.lhs);
    let patch = LhsPatch {
        pattern: Some(PatternPatch {
            left_var: (next.pattern.left_var != current.pattern.left_var).then(|| next.pattern.left_var.clone()),
            left_kind: (next.pattern.left_kind != current.pattern.left_kind).then(|| next.pattern.left_kind.clone()),
            edge_var: (next.pattern.edge_var != current.pattern.edge_var).then(|| next.pattern.edge_var.clone()),
            edge_kind: (next.pattern.edge_kind != current.pattern.edge_kind).then(|| next.pattern.edge_kind.clone()),
            right_var: (next.pattern.right_var != current.pattern.right_var).then(|| next.pattern.right_var.clone()),
            right_kind: (next.pattern.right_kind != current.pattern.right_kind).then(|| next.pattern.right_kind.clone()),
        })
        .filter(|pattern| !pattern.is_empty()),
        where_clause: (next.where_clause != current.where_clause).then(|| next.where_clause.clone()),
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Lhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { lhs: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
