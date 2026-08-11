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
    /// 🧹 Canonical "every mutable field differs" snapshot A. Deliberately SHORTER than
    /// `sweep_b` (2 lines vs. 3) — see the `field_sweep_covers_every_mutable_field` doc comment
    /// for why a flat, unkeyed `Vec<String>` collection needs an asymmetric length to exercise
    /// `removed`/`added` at all.
    fn sweep_a() -> TxtSnapshot {
        TxtSnapshot {
            schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
            lines: vec!["keep-me".into(), "modify-me".into()],
            trailing_newline: false,
            line_ending: LineEnding::Lf,
        }
    }

    /// 🧹 Canonical "every mutable field differs" snapshot B: one line unchanged (`keep-me`),
    /// one modified in place (`modify-me` → `modified!`), one genuinely new tail line
    /// (`added!`) that only exists because `sweep_b` is longer than `sweep_a`;
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
    ///
    /// 🧩 `TxtLinesDiff::between`'s own algorithm (pairwise-compare `0..min(len)`, then
    /// "whichever side is longer supplies the tail" — the exact shape the recipe specifies) can
    /// structurally only ever produce a `removed`-tail XOR an `added`-tail from a single
    /// `between()` call, never both at once, since the two tails are complementary by
    /// construction — there is no field-count-mismatch escape hatch here the way there is for
    /// csv's per-record sub-structure, and no name-keying the way there is for xml's attributes
    /// (see those artifacts' own `field_sweep` tests/reports for the same structural note).
    /// `sweep_a`/`sweep_b` are deliberately different lengths so `ab = between(a, b)` exercises
    /// `modified` + `added` (`b` is longer) and `ba = between(b, a)` exercises `modified` +
    /// `removed` (`a` is now the "longer" side) — between the two directions every kind of line
    /// change the diff type can express is proven, exactly matching what `between_roundtrip_law`
    /// already checks in both directions anyway.
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        use protocol::os_spr::command::DiffAlgebra;
        use protocol::MutationDiff;
        let a = sweep_a();
        let b = sweep_b();

        let ab = TxtDiff::between(&a, &b);
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) must equal b");
        let ba = TxtDiff::between(&b, &a);
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) must equal a");

        assert!(ab.trailing_newline.is_some(), "trailing_newline must be Some in a sweep diff");
        assert!(ab.line_ending.is_some(), "line_ending must be Some in a sweep diff");

        let ab_lines = ab.lines.as_ref().expect("lines diff must be Some in a sweep diff");
        assert!(!ab_lines.modified.is_empty(), "a->b sweep must exercise a modified line");
        assert!(!ab_lines.added.is_empty(), "a->b sweep must exercise an added line (b is longer)");

        let ba_lines = ba.lines.as_ref().expect("reverse lines diff must be Some in a sweep diff");
        assert!(!ba_lines.modified.is_empty(), "b->a sweep must exercise a modified line");
        assert!(!ba_lines.removed.is_empty(), "b->a sweep must exercise a removed line (a is shorter)");

        assert!(TxtDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
