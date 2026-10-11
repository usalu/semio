//! ✅️ Remodeling play app commands — `import-video-done`: the host finished sampling a video; its provenance write commits
//! the importing window's streamed transaction (design §15).

use crate::editor::remodeling::commands::import_video_frame_payload::import_transaction;
use crate::editor::remodeling::transient::RemodelingWindowTransient;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::replace_stream_source;
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use crate::schema::video_codec_from_label;
use crate::{RemodelingSnapshot, VideoSource};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-video-done")]
pub struct ImportVideoDone {
    pub name: String,
    pub duration_ms: f64,
    pub frame_count: u32,
    pub width: u32,
    pub height: u32,
    pub codec: String,
}

/// ✅️ Host-decoded video import finished, through the importing window's tool state `window`: writes `VideoSource`
/// provenance on the stream the import built and COMMITS its streamed transaction, so the whole import (every accepted
/// frame plus this final metadata write) is one edit, one history row, one undo step. An import that accepted no frame
/// ends with nothing to commit; a done for an import that already ended (aborted, or its stream gone) is dropped.
pub fn handle_in_window(payload: &ImportVideoDone, doc: &ArtifactView<'_, RemodelingSnapshot>, window: &RemodelingWindowTransient) -> Result<(Emit<RemodelingMutation, NoConfigMutation>, RemodelingWindowTransient), Fault> {
    let Some(import) = window.import.as_ref() else { return Ok((Emit::default(), window.clone())) };
    let Some(stream_id) = import.stream_id.clone().filter(|stream_id| doc.snapshot.streams.iter().any(|stream| stream.id == *stream_id)) else {
        return Ok((Emit::default(), RemodelingWindowTransient::default()));
    };
    let source = VideoSource { name: payload.name.clone(), container: "unknown".into(), codec: video_codec_from_label(&payload.codec), duration_ms: payload.duration_ms, frame_count: payload.frame_count, width: payload.width, height: payload.height };
    Ok((Emit::commit_transaction(import_transaction(&stream_id), vec![replace_stream_source(stream_id, Some(source))]), RemodelingWindowTransient::default()))
}

/// ✅️ A done dispatched without a window names no import: nothing to commit.
pub fn handle(payload: &ImportVideoDone, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    handle_in_window(payload, doc, &RemodelingWindowTransient::default()).map(|(emit, _)| emit)
}
