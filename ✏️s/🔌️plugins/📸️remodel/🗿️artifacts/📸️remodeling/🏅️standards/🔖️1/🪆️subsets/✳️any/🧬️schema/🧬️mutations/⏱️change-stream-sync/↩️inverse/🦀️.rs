//! ↩️ Inverse for `ChangeStreamSync` — the OLD `sync_offset_ms` looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::change_stream_sync::ChangeStreamSync;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeStreamSync>>(payload: &super::ChangeStreamSync, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![super::change_stream_sync(payload.id.clone(), stream.sync_offset_ms)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
