//! ↩️ Inverse for `CreateStream` — `delete-stream` of the id it created. A duplicate create (id
//! already present in `base`) was itself a no-op, so its inverse is also `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::delete_stream::DeleteStream;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::create_stream::component::CreateStream;

//#region 🔖️Inverse
pub fn inverse<Op: From<DeleteStream>>(payload: &crate::standards::v1::subsets::any::schema::mutations::create_stream::component::CreateStream, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.streams.iter().any(|stream| stream.id == payload.stream.id) {
        return Vec::new();
    }
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_stream::delete_stream(payload.stream.id.clone())]

    })())
}
//#endregion 🔖️Inverse
