//! 🗂️ Remodeling play app panel — the Media tab: an import drop zone plus a summary line per imported
//! stream/asset.

use crate::artifacts::remodeling::{MediaKind, RemodelingSnapshot};
use crate::editor::remodeling::commands::import_frames::REMODELING_MEDIA_ACCEPT;
use crate::editor::remodeling::remodeling_action;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{tree_item, tree_item_desc, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult};

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
pub fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiAssemblyResult<BuiltNode> {
    let mut rows: Vec<UiAssemblyResult<BuiltNode>> = vec![
        tree_item_desc("remodeling-media-drop", labels.panel_media.as_str(), Some(format!("{} ({REMODELING_MEDIA_ACCEPT})", labels.no_streams.as_str()))),
        tree_item("remodeling-media.summary", format!("{}: {} - {}: {}", labels.streams.as_str(), scene.streams.len(), labels.assets.as_str(), scene.assets.len())),
    ];
    for stream in &scene.streams {
        let kind_label = match stream.kind {
            MediaKind::Video => labels.stream_kind_video,
            MediaKind::ImageSequence => labels.stream_kind_image_sequence,
        };
        let description = stream.source.as_ref().map(|source| format!("{:?} {}x{} {:.0}ms", source.codec, source.width, source.height, source.duration_ms));
        rows.push(tree_item_desc(
            format!("remodeling-media.stream.{}", stream.id),
            format!("{} ({}, {} {}, {}: {:.1}ms)", stream.name, kind_label.as_str(), stream.frames.len(), labels.frames.as_str(), labels.sync_offset.as_str(), stream.sync_offset_ms),
            description,
        ));
    }
    let rows = crate::editor::remodeling::ui_node_list(rows)?;
    let (drop_action, _) = remodeling_action("importFramePayload", None)?;
    PanelTreeBuilder::new("remodeling-media")?.section("remodeling-media.streams", Some(crate::editor::remodeling::ui_label(labels.streams.as_str())?), true, rows)?.drop_action(drop_action).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::remodeling::commands::import_frame_payload::testkit_import_checker_stream;
    use crate::editor::remodeling::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_media_panel_lists_every_imported_stream() {
        let mut app = app().await;
        testkit_import_checker_stream(&mut app, 2);
        let body = render_body(&mut app, REMODELING_PLAY_BODY_MEDIA).await;
        assert!(body.contains("frame-0.png"), "the stream's name is listed: {body}");
        assert!(body.contains("remodeling-media-drop"));
    }
}
//#endregion 🧪️Tests
