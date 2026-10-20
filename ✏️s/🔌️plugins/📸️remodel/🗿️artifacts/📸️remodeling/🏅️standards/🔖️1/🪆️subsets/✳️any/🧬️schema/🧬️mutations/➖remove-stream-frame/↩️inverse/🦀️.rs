//! ↩️ Inverse for `RemoveStreamFrame` — re-`add-stream-frame`s the captured BASE frame, which lands
//! at its canonical `(index, asset_id)` position, i.e. exactly the one it was removed from. `kind` is
//! the stream's own BASE kind, which is what the forward verb now asserts rather than rewrites.
//! Missing target/index ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::add_stream_frame::AddStreamFrame;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AddStreamFrame>>(payload: &crate::standards::v1::subsets::any::schema::mutations::remove_stream_frame::component::RemoveStreamFrame, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(stream) = base.streams.iter().find(|stream| stream.id == payload.id) else {
        return Vec::new();
    };
    let Some(frame) = stream.frames.get(payload.frame_index as usize) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::add_stream_frame::add_stream_frame(payload.id.clone(), frame.clone(), stream.kind)]

    })())
}
//#endregion 🔖️Inverse
