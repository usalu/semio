//! 🔺️ Sparse diff builder for `EditRhs` — names only the rewrite-program statement lists the new right-hand side changes.
use crate::standards::v1::subsets::any::schema::diff::{RewritingDiff, RhsPatch};
use crate::RewritingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::EditRhs, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
    let (next, current) = (&payload.new_rhs, &base.rhs);
    let patch = RhsPatch {
        create: (next.create != current.create).then(|| next.create.clone()),
        delete: (next.delete != current.delete).then(|| next.delete.clone()),
        set: (next.set != current.set).then(|| next.set.clone()),
        merge: (next.merge != current.merge).then(|| next.merge.clone()),
        parameters: (next.parameters != current.parameters).then(|| next.parameters.clone()),
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Rhs is already up to date.");
    }
    protocol::MutationOutcome::new(RewritingDiff { rhs: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
