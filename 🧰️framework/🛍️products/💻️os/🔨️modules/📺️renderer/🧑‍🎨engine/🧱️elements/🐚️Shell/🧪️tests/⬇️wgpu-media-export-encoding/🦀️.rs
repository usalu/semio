//! ⬇️ The wgpu shell's half of the media-export encoding law, driven by the SAME language-agnostic
//! fixture the kernel and the TypeScript twin drive (`🎠️kernel/🧫️fixtures/⬇️media-export-encoding/🔣️.json`).
//!
//! Two defects are pinned here, both measured on ticket 26/09/09/PROCEDURAL-3D-END-TO-END:
//!   1. `queue_host_effects` had no `DownloadMediaExport` arm at all, so a plugin export fell into the
//!      funnel's loud drop and this renderer could not export anything.
//!   2. the browser half of `download_media_export` took `_encoding` and blobbed `data` verbatim, so a
//!      binary export would have reached the user as its own base64 TEXT.
//!
//! The download itself ends in a native file dialog or a browser anchor click, neither of which a
//! headless test can present — so the law drives the byte contract both halves now call, and reads
//! the two call sites out of this module's own source so a future edit cannot quietly drop them again.

use super::*;

#[derive(serde::Deserialize)]
struct EncodingCase {
    id: String,
    encoding: Option<String>,
    data: String,
    bytes: Vec<u8>,
}

#[derive(serde::Deserialize)]
struct EncodingFixture {
    cases: Vec<EncodingCase>,
}

fn fixture() -> EncodingFixture {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/⬇️media-export-encoding/🔣️.json")).expect("media-export-encoding fixture JSON")
}

const WGPU_SHELL_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");

/// ⬇️ Every fixture export becomes exactly its declared bytes through the contract BOTH halves of this
/// shell now call — the same rows the React shell and the kernel drive.
#[test]
fn the_wgpu_download_contract_decodes_every_fixture_export() {
    let fixture = fixture();
    assert!(fixture.cases.len() >= 8, "fixture rows: {}", fixture.cases.len());
    for case in &fixture.cases {
        let bytes = semio_framework::kernel::media_export_bytes(&case.data, case.encoding.as_deref()).unwrap_or_else(|error| panic!("{} must decode: {error}", case.id));
        assert_eq!(bytes, case.bytes, "{}", case.id);
        if case.encoding.as_deref() == Some(semio_framework::kernel::MEDIA_EXPORT_BASE64_ENCODING) && !case.bytes.is_empty() {
            assert_ne!(bytes, case.data.as_bytes(), "{} was saved as its own base64 text", case.id);
        }
    }
}

/// 📥️ The host-effect funnel names the export effect. Without this arm every plugin export on this
/// renderer was a `[DEBUG] wgpu-shell effect dropped` line and nothing else.
#[test]
fn the_host_effect_funnel_owns_download_media_export() {
    assert!(WGPU_SHELL_SOURCE.contains("Effect::DownloadMediaExport { filename, mime_type, data, encoding } => {"), "queue_host_effects must own DownloadMediaExport");
    assert!(WGPU_SHELL_SOURCE.contains("download_media_export(&filename, &mime_type, &data, encoding.as_deref());"), "the funnel must route the export to the download door");
}

/// 🕸️ The browser half must READ `encoding`. A `_encoding` parameter here is the exact shape of the
/// defect: a binary export blobbed as its own base64 text.
#[test]
fn neither_download_half_ignores_the_encoding() {
    assert!(!WGPU_SHELL_SOURCE.contains("_encoding: Option<&str>"), "a download half that ignores `encoding` saves base64 text under a binary file name");
    assert_eq!(WGPU_SHELL_SOURCE.matches("semio_framework::kernel::media_export_bytes(data, encoding)").count(), 2, "both the browser and the native half must answer through the kernel contract");
}

/// 🚫️ A malformed or unknown encoding refuses instead of writing the envelope's text to disk.
#[test]
fn a_refused_export_never_falls_back_to_the_envelope_text() {
    assert!(semio_framework::kernel::media_export_bytes("QQ", Some("base64")).is_err());
    assert!(semio_framework::kernel::media_export_bytes("4142", Some("hex")).is_err());
}
