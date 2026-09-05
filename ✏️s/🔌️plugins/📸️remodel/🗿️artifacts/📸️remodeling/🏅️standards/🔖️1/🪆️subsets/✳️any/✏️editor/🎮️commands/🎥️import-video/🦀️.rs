//! 🐚️ 🐚️ Remodeling play app commands command — `import-video`.

use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Constants
/// 📥️ The drop zone's accepted extensions: still-image formats plus every container the `video` engine
/// topic file can probe (decode is attempted in-process; an undecodable codec still records provenance).
pub const REMODELING_MEDIA_ACCEPT: &str = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";
pub const REMODELING_VIDEO_ACCEPT: &str = "video/mp4,video/quicktime,video/webm,video/x-msvideo,.mp4,.mov,.webm,.avi";
//#endregion 🔖️Constants

//#region 🔖️ImportFrames
//#endregion 🔖️ImportFrames

//#region 🔖️ImportVideo
//#endregion 🔖️ImportVideo

//#region 🔖️ExportQcReport
//#endregion 🔖️ExportQcReport

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-video")]
pub struct ImportVideo {}

/// 🎞️ Asks the host to decode and sample the picked video, using the document's own ingest params;
/// `fallback_action` hands the raw container back when the host cannot decode it.
pub async fn handle(_payload: &ImportVideo, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    let ingest = &doc.snapshot.params.ingest;
    Ok(Emit::effect(Effect::RequestMediaFrames {
        req: semio_framework_plugin::RequestId(118),
        accept: REMODELING_VIDEO_ACCEPT.into(),
        frame_action: "importVideoFramePayload".into(),
        done_action: "importVideoDone".into(),
        fallback_action: "importVideoBytesPayload".into(),
        sample_stride: ingest.frame_sample_stride,
        max_frames: ingest.max_frames,
        max_long_edge_px: ingest.downscale_long_edge_px,
        fps_hint: 0.0,
        payload: None,
        args: None,
    }))
}
