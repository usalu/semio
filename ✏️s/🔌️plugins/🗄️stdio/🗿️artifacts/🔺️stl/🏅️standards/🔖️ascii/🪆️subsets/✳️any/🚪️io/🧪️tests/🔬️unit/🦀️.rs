use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = crate::engine::empty_stl_snapshot();
    assert_eq!(snapshot.schema, STDIO_STL_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = crate::engine::empty_stl_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <StlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <StlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

/// 🔺 A real (non-degenerate) 4-triangle tetrahedron — enough structure to catch an
/// off-by-one in facet/vertex slot tracking that a single-triangle fixture would miss. Each
/// facet gets a distinct, non-zero normal to exercise the persisted-normal round trip.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn tetrahedron() -> StlSnapshot {
    let corners = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let faces: [(usize, usize, usize, [f64; 3]); 4] = [(0, 1, 2, [0.0, 0.0, -1.0]), (0, 1, 3, [0.0, -1.0, 0.0]), (1, 2, 3, [1.0, 1.0, 1.0]), (0, 2, 3, [-1.0, 0.0, 0.0])];
    let triangles = faces.iter().map(|&(a, b, c, normal)| StlTriangle { normal, vertices: [corners[a], corners[b], corners[c]] }).collect();
    StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "tetrahedron".into(), triangles }
}

#[semio_framework_async_macros::async_test]
async fn ascii_tetrahedron_round_trip() {
    let snap = tetrahedron();
    let text = encode_stl_ascii(&snap);
    assert!(text.starts_with("solid tetrahedron"));
    assert!(text.trim_end().ends_with("endsolid tetrahedron"));
    assert_eq!(text.matches("facet normal").count(), 4);
    let decoded = decode_stl_ascii(&text).expect("decode");
    assert_eq!(decoded, snap);
}

#[semio_framework_async_macros::async_test]
async fn binary_tetrahedron_round_trip() {
    let snap = tetrahedron();
    let bytes = encode_stl_binary(&snap);
    assert_eq!(bytes.len(), 84 + 4 * 50);
    let decoded = decode_stl_binary(&bytes).expect("decode");
    assert_eq!(decoded.solid_name, snap.solid_name);
    assert_eq!(decoded.triangles.len(), snap.triangles.len());
    // f64 -> f32 -> f64 narrowing is lossy by spec; compare within tolerance.
    for (a, b) in decoded.triangles.iter().zip(snap.triangles.iter()) {
        for i in 0..3 {
            assert!((a.normal[i] - b.normal[i]).abs() < 1e-6);
            for j in 0..3 {
                assert!((a.vertices[i][j] - b.vertices[i][j]).abs() < 1e-6);
            }
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn auto_detect_dispatches_ascii_vs_binary() {
    let snap = tetrahedron();
    let ascii_bytes = encode_stl_ascii(&snap).into_bytes();
    let binary_bytes = encode_stl_binary(&snap);
    assert_eq!(decode_stl_auto(&ascii_bytes).expect("ascii").triangles.len(), 4);
    assert_eq!(decode_stl_auto(&binary_bytes).expect("binary").triangles.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn ascii_facet_normal_is_persisted_not_recomputed() {
    // A real-world "lazy writer" pattern: degenerate 0 0 0 facet normals that a naive
    // recompute-on-encode codec would silently overwrite. This codec must round-trip them
    // exactly as written.
    let text = "solid degenerate\n  facet normal 0 0 0\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid degenerate\n";
    let decoded = decode_stl_ascii(text).expect("decode");
    assert_eq!(decoded.triangles[0].normal, [0.0, 0.0, 0.0]);
    let reencoded = encode_stl_ascii(&decoded);
    let redecoded = decode_stl_ascii(&reencoded).expect("re-decode");
    assert_eq!(redecoded.triangles[0].normal, [0.0, 0.0, 0.0]);
}

#[semio_framework_async_macros::async_test]
async fn ascii_solid_name_round_trips_including_empty() {
    let text = "solid\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid\n";
    let decoded = decode_stl_ascii(text).expect("decode");
    assert_eq!(decoded.solid_name, "");
    let reencoded = encode_stl_ascii(&decoded);
    assert!(reencoded.starts_with("solid \n") || reencoded.starts_with("solid\n"));
}

//#region 🔖️ConformanceLaws
/// 🧪️ FG1: per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
/// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
/// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Dissolved
/// out of `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
    /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
    #[semio_framework_async_macros::async_test]
    async fn committed_facet_files_parse() {
        for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
            let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
        }
        for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
            dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
        }
    }

    /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
    /// for the demo snapshot — same preamble-stripped body reconstruction
    /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
    /// direct proof this artifact will pass that harness once graduated, not merely an
    /// analogue.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&crate::engine::demo_stl_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).unwrap_or(false), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every `StlMutation` variant (`mutations::demo_mutation_cases()`).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
    /// for every representative `StlDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
    /// a full removed+modified+added triple.
    #[semio_framework_async_macros::async_test]
    async fn diff_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for d in diff::demo_diff_cases() {
            let printed = d.print_diff();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
        }
    }

    /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
    /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
    /// bytes.len()`. Mutations/diff `encode_op`/`encode_diff` are the raw hand-rolled text
    /// bytes verbatim (no SEMIO envelope of their own — see those protocol files' own doc
    /// comments), so they're walked directly, unlike the snapshot pack facet.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&crate::engine::demo_stl_snapshot());
        let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
        let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

        let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        for mutation in mutations::demo_mutation_cases() {
            let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
            let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
        }

        let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        for d in diff::demo_diff_cases() {
            let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
            let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
        }
    }

    /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
    /// `print_dsl`/`encode_pack` output of `demo_stl_snapshot()` — `parse_dsl(fixture) ==
    /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
    /// fixtures can never silently drift back to a fake again.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = crate::engine::demo_stl_snapshot();

        let parsed = <StlSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_stl_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_stl_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <StlSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_stl_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_stl_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
