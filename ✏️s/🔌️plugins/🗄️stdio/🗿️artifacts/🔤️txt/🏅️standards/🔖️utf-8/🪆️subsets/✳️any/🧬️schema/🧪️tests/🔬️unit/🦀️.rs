use super::*;
use crate::schema::mutations::InsertLineMutation;
use crate::{TxtDiff, TxtMutation, STDIO_TXT_DOCUMENT_SCHEMA};

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = empty_txt_snapshot();
    assert_eq!(snapshot.schema, STDIO_TXT_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = empty_txt_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <TxtSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <TxtSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

#[semio_framework_async_macros::async_test]
async fn nontrivial_multiline_unicode_round_trip() {
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
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
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
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> TxtSnapshot {
    TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["keep-me".into(), "modify-me".into()], trailing_newline: false, line_ending: LineEnding::Lf }
}

/// 🧹 Canonical "every mutable field differs" snapshot B: one line unchanged (`keep-me`),
/// one modified in place (`modify-me` → `modified!`), one genuinely new tail line
/// (`added!`) that only exists because `sweep_b` is longer than `sweep_a`;
/// `trailing_newline`/`line_ending` both flip.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> TxtSnapshot {
    TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["keep-me".into(), "modified!".into(), "added!".into()], trailing_newline: true, line_ending: LineEnding::CrLf }
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
#[semio_framework_async_macros::async_test]
async fn field_sweep_covers_every_mutable_field() {
    use protocol::os_spr::command::DiffAlgebra;
    use protocol::MutationDiff;
    let a = sweep_a();
    let b = sweep_b();

    let ab = TxtDiff::between(&a, &b);
    assert_eq!(ab.apply(&a).unwrap(), b, "between(a,b).apply(a) must equal b");
    let ba = TxtDiff::between(&b, &a);
    assert_eq!(ba.apply(&b).unwrap(), a, "between(b,a).apply(b) must equal a");

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
#[semio_framework_async_macros::async_test]
async fn grammar_conformance_law() {
    let grammar_text = crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO;
    let grammar = dsl::parse_grammar(grammar_text).expect("parse snapshot grammar");
    assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar);
    let recognizer = dsl::Recognizer::compile(&grammar);
    // 🧬️ CARRIER LAW: `PRIMARY_TEXT` is now the raw body itself (no preamble to split off —
    // `parse_dsl`/`print_dsl` no longer wrap one). The grammar's own synthetic conformance
    // input shape (`"<envelope-id>\n<body>"`) is a framework-level testing convention
    // unrelated to the real codec, so it is built directly here.
    let body = crate::examples::demo::PRIMARY_TEXT;
    let normalized = format!("{}\n{body}", <TxtSnapshot as store::ArtifactDsl>::envelope_id());
    let ok = recognizer.recognize(&normalized).expect("recognize should not error");
    assert!(ok, "snapshot grammar must recognize the real demo fixture body");
}

/// 🧪️ P2-P3: `dsl::parse_protocol` + `dsl::walk_protocol` against REAL bytes for all three
/// binary facets (Pack/Spr/Diff), asserting `consumed == bytes.len()` exactly (the walker's
/// own law) — snapshot's Pack facet walks the post-`unwrap_binary` payload of a genuine
/// `encode_pack` call; mutations' Spr facet walks a genuine `encode_op` frame; diff's own
/// protocol facet walks a genuine `encode_diff` frame.
#[semio_framework_async_macros::async_test]
async fn protocol_walk_law() {
    // Pack (snapshot binary facet).
    let snap = demo_txt_snapshot();
    let pack_bytes = <TxtSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let (_, payload) = store::semio_format::unwrap_binary(&pack_bytes).expect("unwrap_binary");
    let pack_protocol = dsl::parse_protocol(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
    let trace = dsl::walk_protocol(&pack_protocol, &payload).expect("walk snapshot protocol");
    assert_eq!(trace.consumed, payload.len(), "snapshot protocol must consume the whole post-envelope payload");

    // Spr (mutations binary facet) — a real, non-trivial mutation.
    let mutation = TxtMutation::InsertLine(InsertLineMutation { index: 1, text: "x".into() });
    let op_bytes = <TxtMutation as protocol::OpBinary>::encode_op(&mutation).expect("encode_op");
    let spr_protocol = dsl::parse_protocol(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
    let trace = dsl::walk_protocol(&spr_protocol, &op_bytes).expect("walk mutations protocol");
    assert_eq!(trace.consumed, op_bytes.len(), "mutations protocol must consume the whole op frame");

    // Diff binary facet.
    let mut before = snap.clone();
    let diff = crate::schema::mutations::apply_txt_mutation(&mut before, &mutation);
    let diff_bytes = <TxtDiff as protocol::DiffCodec>::encode_diff(diff.diff()).expect("encode_diff");
    let diff_protocol = dsl::parse_protocol(crate::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
    let trace = dsl::walk_protocol(&diff_protocol, &diff_bytes).expect("walk diff protocol");
    assert_eq!(trace.consumed, diff_bytes.len(), "diff protocol must consume the whole diff frame (32-byte header + opaque .spk tail)");
}

/// 🧪️ P2-P3: fixture honesty — the committed `.dsl.semio`/`.pack.semio` fixtures are
/// genuinely `print_dsl`/`encode_pack` output of the SAME demo snapshot, round-tripping both
/// ways (never allowed to silently drift again).
#[semio_framework_async_macros::async_test]
async fn fixture_honesty_law() {
    let demo = demo_txt_snapshot();
    assert_eq!(<TxtSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap(), demo);
    assert_eq!(<TxtSnapshot as store::ArtifactDsl>::print_dsl(&demo), crate::examples::demo::PRIMARY_TEXT);

    assert_eq!(<TxtSnapshot as store::ArtifactPack>::decode_pack(crate::examples::demo::PACK_BYTES).unwrap(), demo);
    assert_eq!(<TxtSnapshot as store::ArtifactPack>::encode_pack(&demo), crate::examples::demo::PACK_BYTES.to_vec());

    let mutation = TxtMutation::InsertLine(InsertLineMutation { index: 1, text: "x".into() });
    let bytes = <TxtMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();
    assert_eq!(bytes.first(), Some(&3));
    assert_eq!(<TxtMutation as protocol::OpBinary>::decode_op(&bytes).unwrap(), mutation);
}

/// 🧪️ P2-P3: every committed grammar/protocol file for this standard genuinely parses under
/// `dsl::parse_grammar`/`dsl::parse_protocol` — this artifact's own early warning, independent
/// of the eventual repo-wide policy gate.
#[semio_framework_async_macros::async_test]
async fn committed_grammar_and_protocol_files_parse() {
    let g1 = dsl::parse_grammar(crate::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO);
    assert!(g1.is_ok(), "snapshot grammar must parse: {g1:?}");
    let g2 = dsl::parse_grammar(crate::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO);
    assert!(g2.is_ok(), "mutations grammar must parse: {g2:?}");
    let g3 = dsl::parse_grammar(crate::schema::diff::text::COMPONENT_GRAMMAR_SEMIO);
    assert!(g3.is_ok(), "diff grammar must parse: {g3:?}");
    let p1 = dsl::parse_protocol(crate::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO);
    assert!(p1.is_ok(), "snapshot protocol must parse: {p1:?}");
    let p2 = dsl::parse_protocol(crate::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO);
    assert!(p2.is_ok(), "mutations protocol must parse: {p2:?}");
    let p3 = dsl::parse_protocol(crate::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO);
    assert!(p3.is_ok(), "diff protocol must parse: {p3:?}");
}

/// 🧪️ P2-P3: `register_schema_spec` genuinely resolves both the document and diff schema ids
/// through `dsl::registry::full_resolver().await` once `register()` has run.
#[semio_framework_async_macros::async_test]
#[cfg(not(target_arch = "wasm32"))]
async fn schema_spec_registration_resolves() {
    use dsl::os_pack::cli::SchemaResolver;
    crate::standards::v_utf_8::subsets::any::io::register_schema_specs();
    let resolver = dsl::registry::full_resolver().await;
    assert!(resolver.resolve("stdio.txt").await.is_some(), "stdio.txt must resolve");
    assert!(resolver.resolve("stdio.txt#diff").await.is_some(), "stdio.txt#diff must resolve");
}
//#endregion 🔖️P2P3GrammarProtocolFixtureLaws
