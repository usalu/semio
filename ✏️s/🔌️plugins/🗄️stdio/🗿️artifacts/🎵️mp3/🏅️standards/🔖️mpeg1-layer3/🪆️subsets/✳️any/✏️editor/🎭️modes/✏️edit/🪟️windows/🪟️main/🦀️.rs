//! ✏️ `mp3 edit` — localized revision-bound native media transport.

use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot;
use crate::standards::mpeg1_layer3::subsets::any::schema::inferences::duration::compute_mp3_duration;
use crate::standards::mpeg1_layer3::subsets::any::schema::sample_rate_hz;
use semio_framework_plugin::app::{MediaKind, MediaResource, MediaTrim, MediaTrimUnit, MediaView, MediaWindowKit};
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

/// 🎬️ The media transport view: inferred timing plus exact live identity for the registered resumable MP3 producer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn view(snapshot: &Mp3Snapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> MediaView {
    semio_s_artifact_stdio_contract::media_transport_view(compute_mp3_duration(snapshot).duration_seconds, MediaKind::Audio, "audio/mpeg", 88.0, locale, resource, trim(snapshot, revision))
}

/// ✂️ The frame-range cut this stream declares to the transport: `cut-frames` addressed in whole MPEG frames, which last 1152 (MPEG-1) or 576 (MPEG-2/2.5) samples at the stream's sample rate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn trim(snapshot: &Mp3Snapshot, revision: Option<String>) -> Option<MediaTrim> {
    let header = &snapshot.frames.first()?.header;
    let rate = sample_rate_hz(header.mpeg_version_id, header.sample_rate_index)?;
    Some(MediaTrim {
        action_id: "cut-frames".into(),
        revision: revision?,
        unit: MediaTrimUnit::Frame,
        units_per_second_numerator: u64::from(rate),
        units_per_second_denominator: if header.mpeg_version_id == 3 { 1152 } else { 576 },
        total_units: snapshot.frames.len() as u64,
    })
}

/// 🎬️ Renders [`view`] through the media window kit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &Mp3Snapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    MediaWindowKit::render(&view(snapshot, locale, resource, revision))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
