//! 🔺️ Diff for `RemoveLayer`.

#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
#[allow(unused_imports)]
use crate::schema::diff::SemioCadDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_layer;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveLayer, base: &SemioCadSnapshot) -> protocol::MutationOutcome<SemioCadDiff> {
    if find_layer(base, &payload.name).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Layer \"{}\" does not exist.", payload.name), [payload.name.clone()]);
    }
    let super::RemoveLayer { name } = payload;
    protocol::MutationOutcome::new(SemioCadDiff { layers: Some(NamedTripleDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() }), blocks: None, entities: None })
}
//#endregion 🔖️Diff
