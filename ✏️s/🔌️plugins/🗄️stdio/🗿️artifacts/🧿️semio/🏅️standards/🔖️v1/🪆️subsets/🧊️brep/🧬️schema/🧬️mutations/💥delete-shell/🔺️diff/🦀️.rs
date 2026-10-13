//! 🔺️ Diff for `DeleteShell`.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioBrepDiff;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteShell, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if !base.shells.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Shell \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { shells: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }), ..Default::default() })
}
//#endregion 🔖️Diff
