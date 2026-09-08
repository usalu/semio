
use super::*;
use crate::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::commands::{export_qc_report, import_video};
use crate::editor::remodeling::testkit::{app, dispatch};

#[semio_framework_async_macros::async_test]
async fn import_pickers_emit_a_host_effect_and_no_operations() {
    let mut app = app().await;
    for command in [RemodelingCommand::ImportFrames(ImportFrames {}), RemodelingCommand::ImportVideo(import_video::ImportVideo {})] {
        let result = dispatch(&mut app, command).await;
        assert!(result.mutations.is_empty(), "a shell picker never mutates the document");
        assert_eq!(result.requested_effects.len(), 1);
    }
}

/// 📤️ Exporting a report the document does not have yet is a no-op, not an error.
#[semio_framework_async_macros::async_test]
async fn export_qc_report_is_a_no_op_without_a_report() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {})).await;
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}
