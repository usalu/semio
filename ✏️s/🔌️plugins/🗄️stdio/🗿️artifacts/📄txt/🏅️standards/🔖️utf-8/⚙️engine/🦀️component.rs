//! ⚙️ TxtEngine — owns a real `TxtArtifact`.

use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::{TxtArtifact, TxtDiff, TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_txt_snapshot() -> TxtSnapshot {
    TxtSnapshot::default()
}

/// 📄️ The `demo` example, parsed once from `examples::demo::PRIMARY_TEXT` — the single source
/// of truth `🗣️example.dsl.semio` is genuinely `print_dsl` of (P2-P3 `fixture_honesty_law`),
/// same pattern as `note::semio_example_snapshot`/`csv::demo_csv_snapshot`.
pub fn demo_txt_snapshot() -> TxtSnapshot {
    <TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::txt::examples::demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_txt_snapshot())
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::txt::io_registry::register();
    register_artifact_schema();
    register_artifact_inferences();
    register_pilot_languages();
    register_schema_specs();
    store::register_document_codec(store::ArtifactCodec::of::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA));
}

/// 📇️ P2-P3: `dsl::registry::register_schema_spec` (P2-M3's `FullResolver` insertion API) — real,
/// non-fabricated calls (unlike json/csv, `TxtSnapshot`/`TxtDiff` DO carry genuine derived
/// `RecordSpec` constructors: `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslDiff)]` emit
/// `__dsl_spec`/`__dsl_diff_spec` respectively, see ../🪆️subsets/✳️any/🧬️schema/📸️snapshot and
/// 🔺️diff's own doc comments). Covers both the document's own schema id and its `"<doc>#diff"`
/// diff schema id, per design ruling B-R4. `#[cfg]`-gated to match `os_dsl::registry`'s own
/// `#[cfg(not(target_arch = "wasm32"))]` (📇️registry/🦀️component.rs) -- the registry simply does
/// not exist as a compiled item on `wasm32`.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.txt", TxtSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.txt#diff", TxtDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) — 5-role
/// `LanguageSpec` set (Document/Ops/Diff/Pack/Spr), following `note`'s exemplar pattern exactly
/// (`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`), same as
/// the sibling `stdio.csv`/`stdio.json` P2 pilots.
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
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.txt.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.txt.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.txt`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::txt::schema::txt_artifact_schema_descriptor());
}

/// 💡️ Registers `s.stdio.txt.inference`'s facet leaves into the OS-wide inference catalog —
/// sibling to `register_artifact_schema()` (separate registry, ticket
/// 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
pub fn register_artifact_inferences() {
    ::schema::register_artifact_inference_descriptor(crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::inferences::txt_artifact_inference_descriptor());
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

    //#region 🔖️P2P3GrammarProtocolFixtureLaws
    /// 🧪️ P2-P3: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against the
    /// REAL fixture body — the snapshot text facet's own real grammar (preamble + `REST`-captured
    /// whole body) recognizes the genuine `print_dsl` output, envelope-id-normalized the same way
    /// `dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dsl_body_from_fixture` feeds the
    /// Recognizer (mirrored here so this law does not depend on the framework's own harness).
    #[test]
    fn grammar_conformance_law() {
        let grammar_text = crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse snapshot grammar");
        assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar);
        let recognizer = dsl::Recognizer::compile(&grammar);
        let fixture = crate::artifacts::txt::examples::demo::PRIMARY_TEXT;
        let (envelope, body) = store::semio_format::split_text_preamble(fixture).expect("real preamble");
        let normalized = format!("{}\n{body}", envelope.envelope_id());
        let ok = recognizer.recognize(&normalized).expect("recognize should not error");
        assert!(ok, "snapshot grammar must recognize the real demo fixture body");
    }

    /// 🧪️ P2-P3: `dsl::parse_protocol` + `dsl::walk_protocol` against REAL bytes for all three
    /// binary facets (Pack/Spr/Diff), asserting `consumed == bytes.len()` exactly (the walker's
    /// own law) — snapshot's Pack facet walks the post-`unwrap_binary` payload of a genuine
    /// `encode_pack` call; mutations' Spr facet walks a genuine `encode_op` frame; diff's own
    /// protocol facet walks a genuine `encode_diff` frame.
    #[test]
    fn protocol_walk_law() {
        // Pack (snapshot binary facet).
        let snap = demo_txt_snapshot();
        let pack_bytes = <TxtSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let (_, payload) = store::semio_format::unwrap_binary(&pack_bytes).expect("unwrap_binary");
        let pack_protocol = dsl::parse_protocol(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let trace = dsl::walk_protocol(&pack_protocol, &payload).expect("walk snapshot protocol");
        assert_eq!(trace.consumed, payload.len(), "snapshot protocol must consume the whole post-envelope payload");

        // Spr (mutations binary facet) — a real, non-trivial mutation.
        let mutation = TxtMutation::InsertLine { index: 1, text: "x".into() };
        let op_bytes = <TxtMutation as protocol::OpBinary>::encode_op(&mutation).expect("encode_op");
        let spr_protocol = dsl::parse_protocol(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        let trace = dsl::walk_protocol(&spr_protocol, &op_bytes).expect("walk mutations protocol");
        assert_eq!(trace.consumed, op_bytes.len(), "mutations protocol must consume the whole op frame");

        // Diff binary facet.
        let mut before = snap.clone();
        let diff = crate::artifacts::txt::schema::mutations::apply_txt_mutation(&mut before, &mutation);
        let diff_bytes = <TxtDiff as protocol::DiffCodec>::encode_diff(&diff).expect("encode_diff");
        let diff_protocol = dsl::parse_protocol(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        let trace = dsl::walk_protocol(&diff_protocol, &diff_bytes).expect("walk diff protocol");
        assert_eq!(trace.consumed, diff_bytes.len(), "diff protocol must consume the whole diff frame (32-byte header + opaque .spk tail)");
    }

    /// 🧪️ P2-P3: fixture honesty — the committed `.dsl.semio`/`.pack.semio` fixtures are
    /// genuinely `print_dsl`/`encode_pack` output of the SAME demo snapshot, round-tripping both
    /// ways (never allowed to silently drift again).
    #[test]
    fn fixture_honesty_law() {
        let demo = demo_txt_snapshot();
        assert_eq!(<TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::txt::examples::demo::PRIMARY_TEXT).unwrap(), demo);
        assert_eq!(<TxtSnapshot as store::ArtifactDsl>::print_dsl(&demo), crate::artifacts::txt::examples::demo::PRIMARY_TEXT);

        assert_eq!(<TxtSnapshot as store::ArtifactPack>::decode_pack(crate::artifacts::txt::examples::demo::PACK_BYTES).unwrap(), demo);
        assert_eq!(<TxtSnapshot as store::ArtifactPack>::encode_pack(&demo), crate::artifacts::txt::examples::demo::PACK_BYTES.to_vec());

        let mutation = TxtMutation::InsertLine { index: 1, text: "x".into() };
        assert_eq!(<TxtMutation as protocol::OpBinary>::encode_op(&mutation).unwrap(), crate::artifacts::txt::examples::demo::SPR_BYTES.to_vec());
        assert_eq!(<TxtMutation as protocol::OpBinary>::decode_op(crate::artifacts::txt::examples::demo::SPR_BYTES).unwrap(), mutation);
    }

    /// 🧪️ P2-P3: every committed grammar/protocol file for this standard genuinely parses under
    /// `dsl::parse_grammar`/`dsl::parse_protocol` — this artifact's own early warning, independent
    /// of the eventual repo-wide policy gate.
    #[test]
    fn committed_grammar_and_protocol_files_parse() {
        let g1 = dsl::parse_grammar(crate::artifacts::txt::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g1.is_ok(), "snapshot grammar must parse: {g1:?}");
        let g2 = dsl::parse_grammar(crate::artifacts::txt::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g2.is_ok(), "mutations grammar must parse: {g2:?}");
        let g3 = dsl::parse_grammar(crate::artifacts::txt::schema::diff::text::COMPONENT_GRAMMAR_SEMIO);
        assert!(g3.is_ok(), "diff grammar must parse: {g3:?}");
        let p1 = dsl::parse_protocol(crate::artifacts::txt::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p1.is_ok(), "snapshot protocol must parse: {p1:?}");
        let p2 = dsl::parse_protocol(crate::artifacts::txt::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p2.is_ok(), "mutations protocol must parse: {p2:?}");
        let p3 = dsl::parse_protocol(crate::artifacts::txt::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO);
        assert!(p3.is_ok(), "diff protocol must parse: {p3:?}");
    }

    /// 🧪️ P2-P3: `register_schema_spec` genuinely resolves both the document and diff schema ids
    /// through `dsl::registry::full_resolver()` once `register()` has run.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn schema_spec_registration_resolves() {
        use dsl::os_pack::cli::SchemaResolver;
        register_schema_specs();
        let resolver = dsl::registry::full_resolver();
        assert!(resolver.resolve("stdio.txt").is_some(), "stdio.txt must resolve");
        assert!(resolver.resolve("stdio.txt#diff").is_some(), "stdio.txt#diff must resolve");
    }
    //#endregion 🔖️P2P3GrammarProtocolFixtureLaws
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::txt::standards::v_utf_8::subsets::any::schema::TxtComposer as TxtRawAnyComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<TxtRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
