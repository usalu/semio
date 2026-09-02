//! 🗂️ Remodeling play app panel — the Media tab: an import drop zone plus a summary line per imported
//! stream/asset.

use crate::artifacts::remodeling::{MediaKind, RemodelingSnapshot};
use crate::editor::remodeling::commands::import_frames::REMODELING_MEDIA_ACCEPT;
use crate::editor::remodeling::remodeling_action;
use crate::editor::remodeling::terminology::RemodelingLabels;
use semio_framework_plugin::{ui_import_drop_zone, ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode};

//#region 🔖️Constants
pub const REMODELING_PANEL_MEDIA_ID: &str = "remodeling.media";
pub const REMODELING_PLAY_BODY_MEDIA: &str = "remodeling.play.media";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(REMODELING_PANEL_MEDIA_ID.into()), label: LocalizedLabel::native("Media", "Medien"), group: PanelGroup::Workbench, body_key: Some(REMODELING_PLAY_BODY_MEDIA.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(scene: &RemodelingSnapshot, labels: &RemodelingLabels) -> UiNode {
    let mut lines = vec![
        ui_import_drop_zone("remodeling-media-drop", labels.panel_media.into(), labels.no_streams.into(), Some(REMODELING_MEDIA_ACCEPT), remodeling_action("importFramePayload", None)),
        ui_text(Label::data(format!("{}: {} - {}: {}", labels.streams.as_str(), scene.streams.len(), labels.assets.as_str(), scene.assets.len()))),
    ];
    for stream in &scene.streams {
        let kind_label = match stream.kind {
            MediaKind::Video => labels.stream_kind_video,
            MediaKind::ImageSequence => labels.stream_kind_image_sequence,
        };
        lines.push(ui_text(Label::data(format!("{} ({}, {} {}, {}: {:.1}ms)", stream.name, kind_label.as_str(), stream.frames.len(), labels.frames.as_str(), labels.sync_offset.as_str(), stream.sync_offset_ms))));
        if let Some(source) = &stream.source {
            lines.push(ui_text(Label::data(format!("  {:?} {}x{} {:.0}ms", source.codec, source.width, source.height, source.duration_ms))));
        }
    }
    ui_stack_vertical(lines)
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
        let mut app = app();
        testkit_import_checker_stream(&mut app, 2);
        let body = render_body(&mut app, REMODELING_PLAY_BODY_MEDIA);
        assert!(body.contains("frame-0.png"), "the stream's name is listed: {body}");
        assert!(body.contains("remodeling-media-drop"));
    }
}
//#endregion 🧪️Tests
