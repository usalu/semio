//! ⬇️ The `Effect::DownloadMediaExport` encoding law, driven by the language-agnostic fixture
//! `🧫️fixtures/⬇️media-export-encoding/🔣️.json` that `🎠️kernel/🟦️.ts`'s TypeScript twin drives too.
//!
//! The defect this pins: every binary export in the repo reached disk as base64 TEXT under a binary
//! file name, because the renderer dropped `encoding` on the guest→shell hop
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).

use super::*;

#[derive(serde::Deserialize)]
struct EncodingCase {
    id: String,
    encoding: Option<String>,
    data: String,
    bytes: Vec<u8>,
}

#[derive(serde::Deserialize)]
struct RefusalCase {
    id: String,
    encoding: Option<String>,
    data: String,
    error: String,
}

#[derive(serde::Deserialize)]
struct EncodingFixture {
    #[serde(rename = "baseEncoding")]
    base_encoding: String,
    #[serde(rename = "textEncoding")]
    text_encoding: String,
    cases: Vec<EncodingCase>,
    refusals: Vec<RefusalCase>,
}

fn fixture() -> EncodingFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/⬇️media-export-encoding/🔣️.json")).expect("media-export-encoding fixture JSON")
}

/// ⬇️ Every fixture export decodes to exactly the bytes the fixture declares.
#[test]
fn every_fixture_export_decodes_to_its_declared_bytes() {
    let fixture = fixture();
    assert_eq!(fixture.base_encoding, MEDIA_EXPORT_BASE64_ENCODING);
    assert_eq!(fixture.text_encoding, MEDIA_EXPORT_UTF8_ENCODING);
    assert!(fixture.cases.len() >= 8, "fixture must keep driving both lanes: {}", fixture.cases.len());
    for case in &fixture.cases {
        let decoded = media_export_bytes(&case.data, case.encoding.as_deref()).unwrap_or_else(|error| panic!("{} must decode: {error}", case.id));
        assert_eq!(decoded, case.bytes, "{} decoded to the wrong bytes", case.id);
    }
}

/// 📝️ A textual export is its own UTF-8 bytes, whether it leaves `encoding` absent or states
/// `utf-8` — the two spellings a live producer really uses, and they must answer identically.
#[test]
fn textual_exports_are_their_own_utf8_bytes_under_both_spellings() {
    let fixture = fixture();
    let textual: Vec<&EncodingCase> = fixture.cases.iter().filter(|case| case.encoding.as_deref() != Some(MEDIA_EXPORT_BASE64_ENCODING)).collect();
    assert!(textual.len() >= 3, "the fixture must keep both textual spellings");
    assert!(textual.iter().any(|case| case.encoding.is_none()) && textual.iter().any(|case| case.encoding.as_deref() == Some(MEDIA_EXPORT_UTF8_ENCODING)), "both spellings must be driven");
    for case in textual {
        assert_eq!(media_export_bytes(&case.data, None).expect("textual decode"), case.data.as_bytes(), "{}", case.id);
        assert_eq!(media_export_bytes(&case.data, Some(MEDIA_EXPORT_UTF8_ENCODING)).expect("declared textual decode"), case.data.as_bytes(), "{}", case.id);
    }
}

/// 🚨️ The whole point: a base64 export must NOT come back as the base64 text. This is the assertion
/// the broken shells passed, because they saved `data` verbatim.
#[test]
fn base64_exports_are_never_their_own_base64_text() {
    let fixture = fixture();
    let binary: Vec<&EncodingCase> = fixture.cases.iter().filter(|case| case.encoding.as_deref() == Some(MEDIA_EXPORT_BASE64_ENCODING) && !case.bytes.is_empty()).collect();
    assert!(!binary.is_empty(), "the fixture must keep a binary lane");
    for case in binary {
        let decoded = media_export_bytes(&case.data, Some(MEDIA_EXPORT_BASE64_ENCODING)).expect("base64 decode");
        assert_ne!(decoded, case.data.as_bytes(), "{} was saved as its own base64 text", case.id);
        assert_eq!(decoded, case.bytes, "{}", case.id);
    }
}

/// 🚫️ An encoding no shell reads, and malformed base64, are loud — never a silent text save.
#[test]
fn refusals_are_loud_and_typed() {
    for case in fixture().refusals {
        let error = media_export_bytes(&case.data, case.encoding.as_deref()).expect_err(&format!("{} must refuse", case.id));
        match (case.error.as_str(), &error) {
            ("unsupported", MediaExportEncodingError::Unsupported { .. }) => {}
            ("malformed", MediaExportEncodingError::Malformed { .. }) => {}
            (expected, actual) => panic!("{} expected {expected}, got {actual:?}", case.id),
        }
    }
}

/// 🔬️ Third-party oracle: the `base64` crate (a dev-only dependency, never a runtime one) must agree
/// with this contract on every base64 fixture row, in both directions.
#[test]
fn matches_third_party_base64_oracle_on_every_fixture_row() {
    use base64::Engine as _;
    let oracle = base64::engine::general_purpose::STANDARD;
    for case in fixture().cases.iter().filter(|case| case.encoding.as_deref() == Some(MEDIA_EXPORT_BASE64_ENCODING)) {
        assert_eq!(oracle.decode(&case.data).expect("oracle decode"), case.bytes, "oracle disagrees on {}", case.id);
        assert_eq!(oracle.encode(&case.bytes), case.data, "oracle re-encode disagrees on {}", case.id);
        assert_eq!(media_export_bytes(&case.data, Some(MEDIA_EXPORT_BASE64_ENCODING)).expect("ours"), oracle.decode(&case.data).expect("oracle"), "ours disagrees with the oracle on {}", case.id);
    }
}

/// 📦️ The effect carries the envelope the contract reads — the field really is `Option<String>` on
/// the kernel type, and a `Some("base64")` really does reach the bytes.
#[test]
fn the_effect_envelope_feeds_the_contract() {
    for case in fixture().cases {
        let effect = Effect::DownloadMediaExport { filename: "export.bin".into(), mime_type: "application/octet-stream".into(), data: case.data.clone(), encoding: case.encoding.clone() };
        let Effect::DownloadMediaExport { data, encoding, .. } = &effect else { panic!("variant") };
        assert_eq!(media_export_bytes(data, encoding.as_deref()).expect("envelope decode"), case.bytes, "{}", case.id);
    }
}
