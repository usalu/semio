//! ✏️ `mp3 edit` — localized revision-bound native media transport.

use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::standards::mpeg1_layer3::subsets::any::schema::inferences::duration::compute_mp3_duration;
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

/// 🎬️ Projects inferred timing and exact live identity for the registered resumable MP3 producer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &Mp3Snapshot, locale: Locale, resource: Option<MediaResource>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let duration_ms = semio_s_artifact_stdio_contract::media_duration_ms(compute_mp3_duration(snapshot).duration_seconds);
    let revision = resource.as_ref().map_or_else(|| "0".to_string(), |resource| resource.revision.clone());
    let ready = resource.is_some();
    let reason = match locale { Locale::En => "Playback waits for a revision-bound document resource.", Locale::De => "Die Wiedergabe wartet auf eine revisionsgebundene Dokumentressource." };
    MediaWindowKit::render(&MediaView {
        duration_ms,
        position_ms: 0,
        selection_start_ms: None,
        selection_end_ms: None,
        kind: MediaKind::Audio,
        media_type: "audio/mpeg".into(),
        revision,
        locale,
        resource,
        capability: if ready { MediaCapabilityStatus::Ready } else { MediaCapabilityStatus::Unsupported },
        capability_reason: (!ready).then(|| reason.into()),
        host_content_height: 88.0,
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
