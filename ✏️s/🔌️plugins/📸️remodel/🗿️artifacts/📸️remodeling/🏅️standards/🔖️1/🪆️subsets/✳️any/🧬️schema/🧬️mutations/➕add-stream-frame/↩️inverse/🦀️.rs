//! ↩️ Inverse for `AddStreamFrame` — `remove-stream-frame` targeting the canonical `(index, asset_id)`
//! position the frame is inserted at, computed from BASE the same way the diff computes it. A step the
//! forward verb refuses or warns off (unknown stream, declared media kind the stream does not have, a
//! frame the stream already holds) moved nothing, so its inverse is `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::remove_stream_frame::RemoveStreamFrame;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<RemoveStreamFrame>>(payload: &crate::standards::v1::subsets::any::schema::mutations::add_stream_frame::component::AddStreamFrame, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    if payload.kind != stream.kind || stream.frames.contains(&payload.frame) {
        return Vec::new();
    }
    let at = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ordered_index(&stream.frames, &(payload.frame.index, payload.frame.asset_id.clone()), |frame| (frame.index, frame.asset_id.clone()));
    vec![crate::standards::v1::subsets::any::schema::mutations::remove_stream_frame::remove_stream_frame(payload.id.clone(), at as u32)]

    })())
}
//#endregion 🔖️Inverse
