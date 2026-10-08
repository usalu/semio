use super::*;

#[test]
fn original_unique_demo_dsl_materializes_through_owned_codecs_and_independent_json_oracle() {
    let original = sources();
    assert_eq!(original[0].text, semio_s_artifact_draw_drawing::examples::demo::source().document_json());
    assert_eq!(original[1].text, semio_s_artifact_writer_writer::examples::demo::source().document_json());
    assert!(original.iter().all(|source| source.format == SpaceDocumentFormat::Dsl));
    let drawing_json = document_for_app("draw", "draw").unwrap();
    let drawing: semio_s_artifact_draw_drawing::DrawingSnapshot = semio_framework_pack_json::from_json_str(&drawing_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let drawing_dsl = <semio_s_artifact_draw_drawing::DrawingSnapshot as store::ArtifactDsl>::parse_dsl(&original[0].text).unwrap();
    assert_eq!(drawing, drawing_dsl);
    assert!(!drawing.layers.is_empty());
    let drawing_oracle: serde_json::Value = serde_json::from_str(&drawing_json).unwrap();
    assert_eq!(drawing_oracle["schema"], "drawing.document");
    assert_eq!(drawing_oracle["id"], "semio");
    assert_eq!(drawing_oracle["title"], "Semio Emblem");
    let writer_json = document_for_app("writer", "writer").unwrap();
    let writer: semio_s_artifact_writer_writer::WriterSnapshot = semio_framework_pack_json::from_json_str(&writer_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let writer_dsl = <semio_s_artifact_writer_writer::WriterSnapshot as store::ArtifactDsl>::parse_dsl(&original[1].text).unwrap();
    assert_eq!(writer, writer_dsl);
    let writer_oracle: serde_json::Value = serde_json::from_str(&writer_json).unwrap();
    assert_eq!(writer_oracle["schema"], "writer.document");
    assert_eq!(writer_oracle["id"], "jack");
    assert_eq!(writer_oracle["text"].as_str(), Some(writer.text.as_str()));
    assert!(writer.text.contains("MATCH (a:Piece)-[r:Connection]->(b:Piece)"));
    assert!(document_for_app("missing", "missing").is_err());
}

#[test]
fn malformed_original_source_and_unknown_codec_fail_before_any_admission() {
    let mut inputs = sources();
    inputs[1].text = "semio malformed".into();
    assert!(crate::prepare_space_document_sources(&inputs, &codecs()).is_err());
    let mut inputs = sources();
    inputs[0].codec = "missing.snapshot.v1".into();
    assert!(crate::prepare_space_document_sources(&inputs, &codecs()).is_err());
}

#[semio_framework_async_macros::async_test]
async fn actual_export_command_forwards_original_demo_content_to_the_media_handler() {
    use crate::engine::space::{commands::export_media::ExportMedia, config::SpaceConfig, SpaceCommand};
    use crate::engine::space::unit_tests::context::studio_emit;
    const FORMAT: &str = "test.space.fixture.source";
    const PROBE_KIND: &str = "test.space.fixture.document";
    directory::io::register_format_descriptors([directory::io::FormatDescriptor {
        kind_id: FORMAT.into(), short_id: FORMAT.into(), aliases: Vec::new(), mimes: vec!["application/vnd.semio.space-fixture-probe+json".into()], extensions: vec![".space-fixture-probe".into()], name: "Fixture source probe".into(), full_name: "Space fixture source probe".into(), neutral: true, dir_name: "fixture-probe".into(), is_binary: false,
    }]).await.unwrap();
    semio_framework_os::workflow::register_os_media_export_handler_kind(PROBE_KIND, FORMAT, |document| {
        let json = document.to_string();
        let independent: serde_json::Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
        match independent["schema"].as_str() {
            Some("drawing.document") if independent["id"] == "semio" && independent["layers"].as_array().is_some_and(|layers| !layers.is_empty()) => {},
            Some("writer.document") if independent["id"] == "jack" && independent["text"].as_str().is_some_and(|text| text.contains("MATCH (a:Piece)-[r:Connection]->(b:Piece)")) => {},
            _ => return Err("original admitted source fields did not reach the export handler".into()),
        }
        Ok(semio_framework_os::OsMediaExportResult { data: json, mime_type: "application/vnd.semio.space-fixture-probe+json".into(), file_name: "fixture.space-fixture-probe".into(), encoding: None })
    });
    let mut projection = crate::demo_space_projection().await;
    for node in projection.graph.nodes.iter_mut().filter(|node| node.plugin_id == "draw" || node.plugin_id == "writer") {
        node.yields = PROBE_KIND.into();
    }
    for node in projection.graph.nodes.iter().filter(|node| node.plugin_id == "draw" || node.plugin_id == "writer") {
        let export = studio_emit(&projection, &SpaceConfig::default(), &SpaceCommand::ExportMedia(ExportMedia { node_id: node.id.clone(), format: FORMAT.into(), document_json: document_for_app(&node.plugin_id, &node.app_id).unwrap() })).await.unwrap();
        let actual = export.effects.iter().find_map(|effect| match effect {
            semio_framework_plugin::Effect::DownloadMediaExport { data, .. } => Some(data),
            _ => None,
        }).expect("actual media effect");
        let actual: serde_json::Value = serde_json::from_str(actual).unwrap();
        let original: serde_json::Value = serde_json::from_str(&document_for_app(&node.plugin_id, &node.app_id).unwrap()).unwrap();
        assert_eq!(actual, original);
    }
}
