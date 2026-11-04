//! 👁️ `mp4 view` — localized revision-bound native media transport.

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::standards::isobmff::subsets::any::schema::inferences::duration::compute_mp4_duration;
use semio_framework_plugin::app::{MediaKind, MediaResource, MediaView, MediaWindowKit};
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

/// 🎬️ The media transport view: inferred timing plus exact live identity, ready only with a revision-bound playback resource.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn view(snapshot: &Mp4Snapshot, locale: Locale, resource: Option<MediaResource>) -> MediaView {
    semio_s_artifact_stdio_contract::media_transport_view(compute_mp4_duration(snapshot).duration_seconds, MediaKind::Video, "video/mp4", 360.0, locale, resource, None)
}

/// 🎬️ Renders [`view`] through the media window kit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &Mp4Snapshot, locale: Locale, resource: Option<MediaResource>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    MediaWindowKit::render(&view(snapshot, locale, resource))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
