//! ↩️ Inverse for `ReplaceStreamSource` — the OLD `source` looked up from BASE.
//! Missing target ⇒ `Vec::new()`.
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::replace_stream_source::component::ReplaceStreamSource;
use crate::standards::v1::subsets::any::schema::mutations::replace_stream_source::component::replace_stream_source;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceStreamSource>>(payload: &crate::standards::v1::subsets::any::schema::mutations::replace_stream_source::component::ReplaceStreamSource, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.streams.iter().find(|stream| stream.id == payload.id) {
        Some(stream) => vec![super::replace_stream_source(payload.id.clone(), stream.source.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
