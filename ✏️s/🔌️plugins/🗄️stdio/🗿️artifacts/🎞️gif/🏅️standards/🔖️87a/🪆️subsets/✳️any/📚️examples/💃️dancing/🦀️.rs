//! 📚️ Example "dancing" for stdio.gif — a real animated GIF89a fixture (54 frames, 800×800,
//! per-frame local color tables, NETSCAPE2.0 infinite loop). Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: unlike the `🎬️demo`
//! example (a stub hex string that never actually decodes), this one's `artifact_json` comes from
//! ACTUALLY decoding the real fixture bytes via the 89a engine's real LZW/GCE/loop codec and
//! serializing the resulting real snapshot.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "dancing";
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Dancing", "Dancing")
}
pub const ICON: &str = "file";

/// 🖼️ The real fixture bytes — confirmed via sniffing to be GIF89a, animated, multiple frames.
pub const DANCING_GIF_BYTES: &[u8] = include_bytes!("🖼️assets/🧪️dancing/🖼️.gif");

/// 📄️ Decodes [`DANCING_GIF_BYTES`] via the real 89a codec. Panics (at example-registration
/// time, not at runtime for end users) if the fixture ever stops decoding — that's a real
/// regression this example exists to catch, not something to paper over with a fallback.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decoded_snapshot() -> crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifSnapshot {
    crate::artifacts::gif::standards::v89a::engine::decode_gif(DANCING_GIF_BYTES).expect("dancing.gif fixture must decode via the real GIF89a codec")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn source() -> ExampleSource {
    let artifact_json = pack::to_json_string(&decoded_snapshot());
    ExampleSource::new(ID, label(), artifact_json, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn dancing_source_nonempty_and_decodes() {
        let src = source();
        assert!(!src.document_json().is_empty());
        let _ = decoded_snapshot();
    }
}
