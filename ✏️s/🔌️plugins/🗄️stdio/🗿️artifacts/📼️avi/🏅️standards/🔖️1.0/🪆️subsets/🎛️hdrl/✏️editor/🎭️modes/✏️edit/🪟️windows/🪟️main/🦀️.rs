//! ✏️ `avi edit` — localized revision-bound native media transport.

use crate::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot;
use crate::standards::v1_0::subsets::any::schema::inferences::duration::compute_avi_duration;
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
pub fn view(snapshot: &AviSnapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> MediaView {
    semio_s_artifact_stdio_contract::media_transport_view(compute_avi_duration(snapshot).duration_seconds, MediaKind::Video, "video/x-msvideo", 360.0, locale, resource, trim(snapshot, revision))
}

/// ✂️ The chunk-range cut this movie declares to the transport: `cut-frames` addressed in whole chunks of the first stream at that stream's `rate / scale`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn trim(snapshot: &AviSnapshot, revision: Option<String>) -> Option<MediaTrim> {
    let stream = snapshot.streams.first()?;
    let chunks = stream.chunks.len() as u64;
    (chunks > 0 && stream.strh.rate > 0 && stream.strh.scale > 0).then(|| MediaTrim {
        action_id: "cut-frames".into(),
        revision: revision.unwrap_or_default(),
        unit: MediaTrimUnit::Frame,
        units_per_second_numerator: u64::from(stream.strh.rate),
        units_per_second_denominator: u64::from(stream.strh.scale),
        total_units: chunks,
    }).filter(|trim| !trim.revision.is_empty())
}

/// 🎬️ Renders [`view`] through the media window kit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render(snapshot: &AviSnapshot, locale: Locale, resource: Option<MediaResource>, revision: Option<String>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    MediaWindowKit::render(&view(snapshot, locale, resource, revision))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
