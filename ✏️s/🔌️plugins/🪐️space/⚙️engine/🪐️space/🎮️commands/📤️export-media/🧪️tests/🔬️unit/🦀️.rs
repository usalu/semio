
use super::*;
use crate::demo_space_projection;
use crate::engine::space::SpaceCommand;
use crate::engine::space::unit_tests::context::{apply_config, studio_emit};
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn space_command_op_text_round_trips_every_variant() {
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportMedia(ExportMedia { node_id: "n1".into(), format: "dwg".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMedia(crate::engine::space::commands::import_media::ImportMedia { node_id: "n1".into(), format: "dwg".into() }));
    store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportMediaPayload(crate::engine::space::commands::import_media_payload::ImportMediaPayload { payload: "data:...".into() }));
}

/// 🪪️ A neutral format id keeps this command law about typed effects rather than linking an
/// unrelated full Stdio codec catalog into Space's four-family Home-I/O build.
const DWG_FORMAT_ID: &str = "s.stdio.dwg.standard.ac1018.representation.document";

#[semio_framework_async_macros::async_test]
async fn export_media_emits_download_effect_and_import_requests_file_open() {
    crate::engine::space::unit_tests::context::seed_draw_plugin().await;
    semio_framework::register_format_descriptors([semio_framework::FormatDescriptor {
        kind_id: DWG_FORMAT_ID.into(),
        short_id: DWG_FORMAT_ID.into(),
        aliases: Vec::new(),
        mimes: vec!["image/vnd.dwg".into()],
        extensions: vec![".dwg".into()],
        name: "Drawing exchange".into(),
        full_name: "Drawing exchange test carrier".into(),
        neutral: false,
        dir_name: "dwg".into(),
        is_binary: true,
    }])
    .await
    .expect("register neutral format descriptor");
    semio_framework_os::workflow::register_os_media_export_handler_kind("2d.drawing", DWG_FORMAT_ID, |_doc| {
        Ok(semio_framework_os::OsMediaExportResult { data: base64_codec::base64_standard_encode(b"space-home-io-test"), mime_type: "image/vnd.dwg".into(), file_name: "draw.dwg".into(), encoding: Some("base64".into()) })
    });
    // 🚪️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1:
    // `register_dwg_import_handler` is deleted (see host `🦀️.rs`'s `media_export_raster`
    // module) -- it was a thin, always-`"dwg"` wrapper over `register_os_media_import_handler_kind`
    // (which stays; it is a domain-neutral format-agnostic primitive, not one of this wave's five
    // targeted functions). This test's `"2d.drawing"` kind is a synthetic test-only stand-in for
    // the real `🖍️draw` artifact (never a registered `ArtifactKindSpec`), so it has no real
    // `ComposerEntry` to migrate to -- inlined here rather than deleted, since the test below
    // exercises `SpaceCommand::ImportMedia`'s effect-producing behaviour, not the DWG bridge itself.
    semio_framework_os::workflow::register_os_media_import_handler_kind("2d.drawing", DWG_FORMAT_ID, |bytes| {
        if bytes != b"space-home-io-test" {
            return Err("unexpected neutral carrier bytes".into());
        }
        Ok(json!({ "schema": "draw.document", "imported": true }))
    });

    let projection = demo_space_projection().await;
    let node = projection.graph.nodes.iter().find(|node| node.plugin_id == "draw").expect("draw node").clone();
    let config = SpaceConfig::default();

    let export = studio_emit(&projection, &config, &SpaceCommand::ExportMedia(ExportMedia { node_id: node.id.clone(), format: DWG_FORMAT_ID.into() })).await.expect("handle");
    let (data, encoding) = export
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::DownloadMediaExport { data, encoding, .. } => Some((data.clone(), encoding.clone())),
            _ => None,
        })
        .expect("DownloadMediaExport effect");
    assert!(!data.is_empty());
    assert_eq!(encoding.as_deref(), Some("base64"));

    let import = studio_emit(&projection, &config, &SpaceCommand::ImportMedia(crate::engine::space::commands::import_media::ImportMedia { node_id: node.id.clone(), format: DWG_FORMAT_ID.into() })).await.expect("handle");
    assert!(import.effects.iter().any(|effect| matches!(effect, Effect::RequestFileOpen { import_action, accept, .. } if import_action == "importMediaPayload" && accept.contains(".dwg"))));
    assert_eq!(import.config_mutations, vec![SpaceConfigMutation::SetPendingImport { node_id: Some(node.id), format: Some(DWG_FORMAT_ID.into()) }]);

    let pending_config = apply_config(&config, &import.config_mutations).await;
    let payload = studio_emit(&projection, &pending_config, &SpaceCommand::ImportMediaPayload(crate::engine::space::commands::import_media_payload::ImportMediaPayload { payload: format!("data:image/vnd.dwg;base64,{data}") })).await.expect("handle");
    assert!(payload.artifact_mutations.is_empty());
}
