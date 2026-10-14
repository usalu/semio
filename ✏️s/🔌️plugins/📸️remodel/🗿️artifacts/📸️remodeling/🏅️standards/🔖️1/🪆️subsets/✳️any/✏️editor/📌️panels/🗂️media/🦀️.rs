//! 🗂️ Remodeling play app panel — the Media tab: an import drop zone plus a summary line per imported
//! stream/asset.

use semio_s_artifact_remodel_remodeling_cmd_ingest::editor::remodeling::commands::import_frames::REMODELING_MEDIA_ACCEPT;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::remodeling_action;
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::terminology::RemodelingLabels;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MediaKind;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::tree_item;
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;

//#region 🔖️Constants
pub const REMODELING_PANEL_MEDIA_ID: &str = "remodeling.media";
pub const REMODELING_PLAY_BODY_MEDIA: &str = "remodeling.play.media";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_MEDIA_ID.into()), label: LocalizedLabel::native("Media", "Medien"), group: PanelGroup::Workbench, body_key: Some(REMODELING_PLAY_BODY_MEDIA.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🪟️ A multi-camera import brings in more streams than one node admits, so the stream list is its
/// own windowed section beside the fixed drop zone and counts line.
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let chrome = semio_framework_plugin::ui_node_list([
        tree_item_desc("remodeling-media-drop", labels.panel_media.as_str(), Some(format!("{} ({REMODELING_MEDIA_ACCEPT})", labels.no_streams.as_str()))),
        tree_item("remodeling-media.summary", format!("{}: {} - {}: {}", labels.streams.as_str(), scene.streams.len(), labels.assets.as_str(), scene.assets.len())),
    ])?;
    let (drop_action, _) = remodeling_action("importFramePayload", None)?;
    PanelTreeBuilder::new("remodeling-media")?
        .section("remodeling-media.streams", Some(semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::ui_label(labels.streams.as_str())?), true, chrome)?
        .window_section(windows, "remodeling-media.imported", Some(semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::kit::ui_label(labels.streams.as_str())?), true, &scene.streams, |stream| {
            let kind_label = match stream.kind {
                MediaKind::Video => labels.stream_kind_video,
                MediaKind::ImageSequence => labels.stream_kind_image_sequence,
            };
            let description = stream.source.as_ref().map(|source| format!("{:?} {}x{} {:.0}ms", source.codec, source.width, source.height, source.duration_ms));
            tree_item_desc(
                format!("remodeling-media.stream.{}", stream.id),
                format!("{} ({}, {} {}, {}: {:.1}ms)", stream.name, kind_label.as_str(), stream.frames.len(), labels.frames.as_str(), labels.sync_offset.as_str(), stream.sync_offset_ms),
                description,
            )
        })?
        .drop_action(drop_action)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
