//! ✏️ `mp4 edit` — localized revision-bound native media transport.

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::standards::isobmff::subsets::any::schema::inferences::duration::compute_mp4_duration;
use semio_framework_plugin::app::{MediaCapabilityStatus, MediaKind, MediaResource, MediaView, MediaWindowKit};
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::Locale;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowKit;

pub const WINDOW_KIND_ID: &str = MediaWindowKit::KIND_ID;
pub const BODY_KEY: &str = MediaWindowKit::KIND_ID;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> WindowKindDefinition {
    MediaWindowKit::window_kind()
}

/// 🎬️ Projects inferred timing and exact live identity without claiming an unregistered encoded-byte producer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &Mp4Snapshot, locale: Locale, resource: Option<MediaResource>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let duration_ms = semio_s_artifact_stdio_contract::media_duration_ms(compute_mp4_duration(snapshot).duration_seconds);
    let revision = resource.as_ref().map_or_else(|| "0".to_string(), |resource| resource.revision.clone());
    let reason = match locale { Locale::En => "Encoded playback export is not registered for this app.", Locale::De => "Der kodierte Wiedergabeexport ist für diese App nicht registriert." };
    MediaWindowKit::render(&MediaView {
        duration_ms,
        position_ms: 0,
        selection_start_ms: None,
        selection_end_ms: None,
        kind: MediaKind::Video,
        media_type: "video/mp4".into(),
        revision,
        locale,
        resource,
        capability: MediaCapabilityStatus::Unsupported,
        capability_reason: Some(reason.into()),
        host_content_height: 360.0,
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
