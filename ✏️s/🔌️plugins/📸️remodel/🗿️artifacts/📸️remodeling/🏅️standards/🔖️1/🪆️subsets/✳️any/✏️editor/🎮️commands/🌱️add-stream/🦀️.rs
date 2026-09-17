//! 📥️ 📥️ Remodeling play app commands command — `add-stream`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::create_stream;
use crate::op::RemodelingMutation;
use crate::schema::next_remodeling_id;
use crate::{MediaKind, MediaStream, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-stream")]
pub struct AddStream {
    pub name: String,
    pub kind: String,
    pub camera_id: String,
}

/// 🌱️ An empty `camera_id` is an uncalibrated stream; a non-empty one must name a calibrated camera
/// of the open document, because `CreateStream::diff` answers a FATAL `mutation.invariant` for an
/// unknown camera — and a fatal outcome never reaches the ledger, so it is refused here, where the
/// shell shows the refusal, instead of inside the bounded publication.
pub fn handle(payload: &AddStream, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let kind = if payload.kind == "video" { MediaKind::Video } else { MediaKind::ImageSequence };
    let camera_id = match payload.camera_id.trim() {
        "" => None,
        camera_id if doc.snapshot.calibration.cameras.iter().any(|camera| camera.id == camera_id) => Some(camera_id.to_string()),
        camera_id => return Err(Fault::new(FaultOrigin::App, FaultCode::new("remodeling.stream.unknown-camera"), format!("no calibrated camera \"{camera_id}\" to bind the new stream to"))),
    };
    let id = next_remodeling_id("stream");
    let stream = MediaStream { id, name: payload.name.clone(), kind, camera_id, sync_offset_ms: 0.0, fps_hint: 30.0, frames: Vec::new(), source: None };
    Ok(Emit::mutations(vec![create_stream(stream)]))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
