//! 🐚️ 🐚️ Remodeling play app commands command — `import-frames`.

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
#[dsl(keyword = "import-frames")]
pub struct ImportFrames {}

pub async fn handle(_payload: &ImportFrames, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(117), accept: REMODELING_MEDIA_ACCEPT.into(), read_as: Some("dataUrl".into()), import_action: "importFramePayload".into(), multiple: true }))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::commands::{export_qc_report, import_video};
    use crate::editor::remodeling::testkit::{app, dispatch};
    use crate::editor::remodeling::RemodelingCommand;

    #[semio_framework_async_macros::async_test]
    async fn import_pickers_emit_a_host_effect_and_no_operations() {
        let mut app = app();
        for command in [RemodelingCommand::ImportFrames(ImportFrames {}), RemodelingCommand::ImportVideo(import_video::ImportVideo {})] {
            let result = dispatch(&mut app, command);
            assert!(result.mutations.is_empty(), "a shell picker never mutates the document");
            assert_eq!(result.requested_effects.len(), 1);
        }
    }

    /// 📤️ Exporting a report the document does not have yet is a no-op, not an error.
    #[semio_framework_async_macros::async_test]
    async fn export_qc_report_is_a_no_op_without_a_report() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {}));
        assert!(result.mutations.is_empty());
        assert!(result.requested_effects.is_empty());
    }
}
//#endregion 🧪️Tests
