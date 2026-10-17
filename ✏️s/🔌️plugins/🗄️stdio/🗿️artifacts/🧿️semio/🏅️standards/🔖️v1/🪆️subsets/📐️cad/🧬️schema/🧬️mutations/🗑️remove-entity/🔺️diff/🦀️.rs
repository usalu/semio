//! 🔺️ Diff for `RemoveEntity`.

#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_entity;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveEntity, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_entity(base, &payload.handle).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Entity \"{}\" does not exist.", payload.handle), [payload.handle.clone()]);
    }
    let super::RemoveEntity { handle } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: None, blocks: None, entities: Some(NamedTripleDiff { removed: vec![handle.clone()], modified: Vec::new(), added: Vec::new() }) })
}
//#endregion 🔖️Diff
