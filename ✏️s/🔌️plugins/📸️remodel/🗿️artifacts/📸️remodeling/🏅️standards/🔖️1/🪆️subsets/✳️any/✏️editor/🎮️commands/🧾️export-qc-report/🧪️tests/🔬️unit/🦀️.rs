use semio_s_artifact_remodel_remodeling_cmd_workflow::editor::remodeling::commands::export_qc_report;
use semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingCommand;
use crate::editor::remodeling::unit_tests::context::{app, dispatch};

/// 📤️ Exporting a report the document does not have yet is a no-op, not an error.
#[semio_framework_async_macros::async_test]
async fn export_qc_report_is_a_no_op_without_a_report() {
    let mut app = app().await;
    let result = dispatch(&mut app, RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {})).await;
    assert!(!result.edited_document(), "{:?}", result.lanes);
    assert!(result.requested_effects.is_empty());
}

/// 🧾️ Portable QC export cases preserve report JSON and never edit the document.
#[test]
fn export_qc_report_matches_the_portable_download_contract() {
    use semio_plugin::{ArtifactView, ConfigView, Effect, HistoryView, NoConfig};
    use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️cases/🔣️.json")).expect("QC export cases");
    for case in fixture["cases"].as_array().expect("cases") {
        let mut scene = RemodelingSnapshot::default();
        scene.results.qc = if case["report"].is_null() { None } else { Some(serde_json::from_value(case["report"].clone()).expect("QC report")) };
        let before = scene.clone();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&scene, &history);
        let emit = export_qc_report::handle(&export_qc_report::ExportQcReport {}, &doc, &ConfigView { snapshot: &NoConfig, window: None }).expect("QC export succeeds, including absent reports");
        assert!(emit.artifact_mutations.is_empty());
        assert!(emit.config_mutations.is_empty());
        assert!(emit.window_config_mutations.is_empty());
        assert!(emit.draft_mutations.is_empty());
        assert_eq!(scene, before);
        assert_eq!(emit.effects.len(), case["effects"].as_u64().expect("effect count") as usize, "{}", case["id"]);
        if let [Effect::DownloadMediaExport { filename, mime_type, data, encoding }] = emit.effects.as_slice() {
            assert_eq!(filename, fixture["filename"].as_str().expect("filename"));
            assert_eq!(mime_type, fixture["mimeType"].as_str().expect("MIME"));
            assert!(encoding.is_none());
            assert_eq!(serde_json::from_str::<serde_json::Value>(data).expect("downloaded JSON"), case["report"]);
        } else {
            assert_eq!(case["effects"], 0);
        }
    }
}
