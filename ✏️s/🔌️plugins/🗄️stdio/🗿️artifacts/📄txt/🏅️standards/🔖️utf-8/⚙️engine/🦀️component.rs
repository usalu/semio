//! ⚙️ TxtEngine — owns a real `TxtArtifact`.

use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::{TxtArtifact, TxtDiff, TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_txt_snapshot() -> TxtSnapshot {
    TxtSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::txt::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt",
        extension: Some("txt"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.txt`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::txt::schema::txt_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.txt` artifact engine.
pub struct TxtEngine {
    artifact_state: TxtArtifact,
    snapshot_state: TxtSnapshot,
}

impl TxtEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: TxtSnapshot) -> Self {
        let artifact_state = TxtArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_txt_snapshot();
        assert_eq!(snapshot.schema, STDIO_TXT_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_txt_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn nontrivial_multiline_unicode_round_trip() {
        let body = "Hello, \u{4e16}\u{754c}!\nLine two with an emoji \u{1f389}.\nTab\there.\n".to_string();
        let snap = TxtSnapshot::from_body(&body);
        assert_eq!(snap.to_body(), body);
        let dsl_text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
        assert_eq!(parsed.to_body(), body);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.to_body(), body);
    }

    /// 🧪️ `codec_retention_law`: decode→encode is byte-preserving on real fixtures — CRLF, no
    /// trailing newline, and a fully empty document all round-trip exactly at the `to_body`/
    /// `from_body` and binary-pack layers (pack has no preamble-trimming quirk). The DSL-text
    /// layer additionally round-trips for bodies that don't open with a blank line: the shared
    /// `store::semio_format::wrap_text` (outside this artifact's ownership boundary) unwraps
    /// via `body.trim_start()`, which is documented-lossy for a body starting with its own
    /// newline -- pre-existing framework behavior, not something this diff/mutation wave owns.
    #[test]
    fn codec_retention_law() {
        for body in ["a\nb\nc\n", "a\r\nb\r\nc", "", "\n", "just one line, no newline"] {
            let snap = TxtSnapshot::from_body(body);
            assert_eq!(snap.to_body(), body, "to_body/from_body mismatch for {body:?}");
            let bytes = store::ArtifactPack::encode_pack(&snap);
            let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
            assert_eq!(decoded, snap, "pack round-trip mismatch for {body:?}");
        }
        for body in ["a\nb\nc\n", "a\r\nb\r\nc", "just one line, no newline"] {
            let snap = TxtSnapshot::from_body(body);
            let dsl_text = store::ArtifactDsl::print_dsl(&snap);
            let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
            assert_eq!(parsed, snap, "dsl round-trip mismatch for {body:?}");
        }
    }

    //#region 🔖️FieldSweep
    /// 🧹 Canonical "every mutable field differs" snapshot A.
    fn sweep_a() -> TxtSnapshot {
        TxtSnapshot {
            schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
            lines: vec!["keep-me".into(), "remove-me".into(), "modify-me".into()],
            trailing_newline: false,
            line_ending: LineEnding::Lf,
        }
    }

    /// 🧹 Canonical "every mutable field differs" snapshot B: `lines` exercises one removed
    /// (`remove-me`), one modified-in-place (`modify-me` → `modified!`), one added (`added!`);
    /// `trailing_newline`/`line_ending` both flip.
    fn sweep_b() -> TxtSnapshot {
        TxtSnapshot {
            schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
            lines: vec!["keep-me".into(), "modified!".into(), "added!".into()],
            trailing_newline: true,
            line_ending: LineEnding::CrLf,
        }
    }

    /// 🧪️ `field_sweep`: THE acceptance criterion. `between` round-trips both directions, every
    /// diff field is populated (`is_some()`), and `between(a,a)` is empty.
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        use protocol::DiffAlgebra;
        let a = sweep_a();
        let b = sweep_b();

        let ab = TxtDiff::between(&a, &b);
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) must equal b");
        let ba = TxtDiff::between(&b, &a);
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) must equal a");

        assert!(ab.trailing_newline.is_some(), "trailing_newline must be Some in a sweep diff");
        assert!(ab.line_ending.is_some(), "line_ending must be Some in a sweep diff");
        let lines_diff = ab.lines.as_ref().expect("lines diff must be Some in a sweep diff");
        assert!(!lines_diff.removed.is_empty(), "sweep must exercise a removed line");
        assert!(!lines_diff.modified.is_empty(), "sweep must exercise a modified line");
        assert!(!lines_diff.added.is_empty(), "sweep must exercise an added line");

        assert!(TxtDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
