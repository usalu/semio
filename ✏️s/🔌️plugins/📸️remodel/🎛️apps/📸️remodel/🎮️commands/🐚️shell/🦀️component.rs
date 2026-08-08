//! 🐚️ Remodel play app commands — host shell effects: the two import pickers and the QC report export.
//! None of these mutate the document; each returns exactly one `HostEffect`.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigMutation};
use crate::artifacts::remodel::op::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 📥️ The drop zone's accepted extensions: still-image formats plus every container the `video` engine
/// topic file can probe (decode is attempted in-process; an undecodable codec still records provenance).
pub const REMODEL_MEDIA_ACCEPT: &str = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";
pub const REMODEL_VIDEO_ACCEPT: &str = "video/mp4,video/quicktime,video/webm,video/x-msvideo,.mp4,.mov,.webm,.avi";
//#endregion 🔖️Constants

//#region 🔖️ImportFrames
pub mod import_frames {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-frames")]
    pub struct ImportFrames {}

    pub fn handle(_payload: &ImportFrames, _doc: &DocumentView<'_, RemodelProjection>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: REMODEL_MEDIA_ACCEPT.into(), read_as: Some("dataUrl".into()), import_action: "importFramePayload".into(), multiple: true }))
    }
}
//#endregion 🔖️ImportFrames

//#region 🔖️ImportVideo
pub mod import_video {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-video")]
    pub struct ImportVideo {}

    /// 🎞️ Asks the host to decode and sample the picked video, using the document's own ingest params;
    /// `fallback_action` hands the raw container back when the host cannot decode it.
    pub fn handle(_payload: &ImportVideo, doc: &DocumentView<'_, RemodelProjection>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        let ingest = &doc.projection.params.ingest;
        Ok(Emit::effect(HostEffect::RequestMediaFrames {
            accept: REMODEL_VIDEO_ACCEPT.into(),
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
}
//#endregion 🔖️ImportVideo

//#region 🔖️ExportQcReport
pub mod export_qc_report {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-qc-report")]
    pub struct ExportQcReport {}

    pub fn handle(_payload: &ExportQcReport, doc: &DocumentView<'_, RemodelProjection>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelMutation, RemodelConfigMutation>, Fault> {
        match &doc.projection.results.qc {
            Some(qc) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: "remodel-qc-report.ops".into(), mime_type: "text/plain".into(), data: serde_json::to_string_pretty(qc).unwrap_or_default(), encoding: None })),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportQcReport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;

    #[test]
    fn import_pickers_emit_a_host_effect_and_no_operations() {
        let mut app = app();
        for command in [RemodelCommand::ImportFrames(import_frames::ImportFrames {}), RemodelCommand::ImportVideo(import_video::ImportVideo {})] {
            let result = dispatch(&mut app, command);
            assert!(result.mutations.is_empty(), "a shell picker never mutates the document");
            assert_eq!(result.requested_effects.len(), 1);
        }
    }

    /// 📤️ Exporting a report the document does not have yet is a no-op, not an error.
    #[test]
    fn export_qc_report_is_a_no_op_without_a_report() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::ExportQcReport(export_qc_report::ExportQcReport {}));
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    }
}
//#endregion 🧪️Tests
