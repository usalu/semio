//! 🔺️ `delete-shell` — sparse diff construction; removes the shell with this `id` from
//! `shells`. No cascade into any other collection (see the payload leaf's doc comment for
//! why). Absent `id` in `base` is `mutation.target-missing` (Error, empty diff — id-keyed
//! collections are unordered sets; a spurious `removed` entry for an absent id would make
//! `SemioBrepDiff::is_empty()` lie).

use super::mutation::DeleteShell;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &DeleteShell, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if !base.shells.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shell \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { shells: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }), ..Default::default() })
}
//#endregion 🔖️Diff
