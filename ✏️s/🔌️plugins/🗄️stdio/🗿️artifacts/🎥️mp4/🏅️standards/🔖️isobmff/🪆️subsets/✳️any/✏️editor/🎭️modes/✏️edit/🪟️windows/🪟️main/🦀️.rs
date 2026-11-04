//! ✏️ `mp4 edit` — localized revision-bound native media transport.

use crate::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot;
use crate::standards::isobmff::subsets::any::schema::inferences::duration::compute_mp4_duration;
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

/// 🎬️ The media transport view: inferred timing plus exact live identity, ready only with a revision-bound playback resource.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn view(snapshot: &Mp4Snapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> MediaView {
    semio_s_artifact_stdio_contract::media_transport_view(compute_mp4_duration(snapshot).duration_seconds, MediaKind::Video, "video/mp4", 360.0, locale, resource, trim(snapshot, revision))
}

/// ✂️ The sample-range cut this movie declares to the transport: `cut-frames` addressed in whole samples of the first track, at that track's mean sample rate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn trim(snapshot: &Mp4Snapshot, revision: Option<String>) -> Option<MediaTrim> {
    let track = snapshot.tracks.first()?;
    let samples = track.samples.len() as u64;
    let ticks: u64 = track.samples.iter().map(|sample| u64::from(sample.duration)).sum();
    (samples > 0 && ticks > 0 && track.timescale > 0).then(|| MediaTrim {
        action_id: "cut-frames".into(),
        revision: revision.unwrap_or_default(),
        unit: MediaTrimUnit::Frame,
        units_per_second_numerator: samples.saturating_mul(u64::from(track.timescale)),
        units_per_second_denominator: ticks,
        total_units: samples,
    }).filter(|trim| !trim.revision.is_empty())
}

/// 🎬️ Renders [`view`] through the media window kit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &Mp4Snapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    MediaWindowKit::render(&view(snapshot, locale, resource, revision))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
