
use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_snapshot_matches_schema() {
    let snapshot = crate::engine::empty_obj_snapshot();
    assert_eq!(snapshot.schema, STDIO_OBJ_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn codec_round_trip() {
    let snap = crate::engine::empty_obj_snapshot();
    let text = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <ObjSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(parsed.schema, snap.schema);
    let bytes = store::ArtifactPack::encode_pack(&snap);
    let decoded = <ObjSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(decoded, snap);
}

#[semio_framework_async_macros::async_test]
async fn negative_indices_resolve_correctly() {
    let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf -3 -2 -1\n";
    let snap = decode_obj(text).expect("parse");
    assert_eq!(snap.vertices.len(), 3);
    let idxs: Vec<u32> = snap.faces[0].vertices.iter().map(|fv| fv.vertex).collect();
    assert_eq!(idxs, vec![0, 1, 2]);
}

#[semio_framework_async_macros::async_test]
async fn out_of_range_negative_index_is_error() {
    let text = "v 0 0 0\nf -2 1 1\n";
    let err = decode_obj(text).unwrap_err();
    assert!(err.contains("out of range"), "unexpected error: {err}");
}

#[semio_framework_async_macros::async_test]
async fn face_index_forms_all_supported() {
    let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nvt 0 0\nvt 1 0\nvt 0 1\nvn 0 0 1\n\
                     f 1/1/1 2/2/1 3/3/1\nf 1//1 2//1 3//1\nf 1/1 2/2 3/3\nf 1 2 3\n";
    let snap = decode_obj(text).expect("parse");
    assert_eq!(snap.faces.len(), 4);
    assert_eq!(snap.faces[0].vertices[0].texcoord, Some(0));
    assert_eq!(snap.faces[0].vertices[0].normal, Some(0));
    assert_eq!(snap.faces[1].vertices[0].texcoord, None);
    assert_eq!(snap.faces[1].vertices[0].normal, Some(0));
    assert_eq!(snap.faces[2].vertices[0].texcoord, Some(0));
    assert_eq!(snap.faces[2].vertices[0].normal, None);
    assert_eq!(snap.faces[3].vertices[0].texcoord, None);
    assert_eq!(snap.faces[3].vertices[0].normal, None);
}

#[semio_framework_async_macros::async_test]
async fn multi_group_multi_material_negative_index_round_trip() {
    let text = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nv 0 0 1\nv 1 0 1\n\
                     vt 0 0\nvt 1 0\nvt 1 1\nvn 0 0 1\nvn 0 0 -1\n\
                     o Cube\ng Front\nusemtl Red\ns 1\n\
                     f 1/1/1 2/2/1 3/3/1\n\
                     f -4/-3/-2 -3/-2/-2 1/1/2\n\
                     g Back\nusemtl Blue\ns off\n\
                     f 4 3 2\n\
                     f -6 -5 -4\n";
    let snap = decode_obj(text).expect("parse");
    assert_eq!(snap.vertices.len(), 6);
    assert_eq!(snap.texcoords.len(), 3);
    assert_eq!(snap.normals.len(), 2);
    assert_eq!(snap.faces.len(), 4);

    assert_eq!(snap.objects.len(), 1);
    assert_eq!(snap.objects[0].name, "Cube");
    assert_eq!(snap.objects[0].faces, vec![0, 1, 2, 3]);

    assert_eq!(snap.groups.len(), 2);
    assert_eq!(snap.groups[0], ObjGroup { name: "Front".into(), faces: vec![0, 1] });
    assert_eq!(snap.groups[1], ObjGroup { name: "Back".into(), faces: vec![2, 3] });

    assert_eq!(snap.usemtl, vec![ObjUsemtlRange { face_index_from: 0, material: "Red".into() }, ObjUsemtlRange { face_index_from: 2, material: "Blue".into() },]);
    assert_eq!(snap.smoothing_groups, vec![ObjSmoothingRange { face_index_from: 0, group: Some(1) }, ObjSmoothingRange { face_index_from: 2, group: None },]);

    assert_eq!(snap.faces[1].vertices[0].vertex, 2);
    assert_eq!(snap.faces[1].vertices[0].texcoord, Some(0));
    assert_eq!(snap.faces[1].vertices[0].normal, Some(0));
    assert_eq!(snap.faces[3].vertices[0].vertex, 0);

    let text2 = encode_obj(&snap);
    let snap2 = decode_obj(&text2).expect("re-parse");
    assert_eq!(snap2, snap, "round trip through encode/decode must be lossless");
}

#[semio_framework_async_macros::async_test]
async fn optional_w_components_retained() {
    let text = "v 0 0 0 1.5\nv 1 0 0\nvt 0.1 0.2 0.3\nvt 0.5 0.5\nvn 0 0 1\nf 1/1/1 2/2/1 1/1/1\n";
    let snap = decode_obj(text).expect("parse");
    assert_eq!(snap.vertices[0].w, Some(1.5));
    assert_eq!(snap.vertices[1].w, None);
    assert_eq!(snap.texcoords[0].w, Some(0.3));
    assert_eq!(snap.texcoords[1].w, None);
}

#[semio_framework_async_macros::async_test]
async fn mtllib_last_occurrence_wins() {
    let text = "mtllib a.mtl\nv 0 0 0\nv 1 0 0\nv 0 1 0\nmtllib b.mtl c.mtl\nf 1 2 3\n";
    let snap = decode_obj(text).expect("parse");
    assert_eq!(snap.mtllib.as_deref(), Some("b.mtl c.mtl"));
}

//#region 🔖️CodecRetentionLaw
/// 🔁️ decode→encode retains every field (geometry, group/object/usemtl/smoothing
/// membership, mtllib, and unknown-statement content+relative order); per this module's
/// documented normal form, `unknown_statements[].line_index` is renumbered on re-encode
/// (comments/unrecognized lines move into a trailer), so from the SECOND generation onward
/// decode/encode is a true fixed point.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let fixture = "# leading comment\nmtllib materials.mtl\n\
                        v 0 0 0\nv 1 0 0\nv 0 1 0 1\n\
                        vt 0 0\nvt 1 0 0.5\nvn 0 0 1\n\
                        o Cube\ng Front\nusemtl Red\ns 1\n\
                        f 1/1/1 2/2/1 3/1/1\n\
                        # trailing comment\nweird_directive foo bar\n";
    let snap1 = decode_obj(fixture).expect("decode");
    assert_eq!(snap1.mtllib.as_deref(), Some("materials.mtl"));
    assert_eq!(snap1.vertices[2].w, Some(1.0));
    assert_eq!(snap1.texcoords[1].w, Some(0.5));
    assert_eq!(snap1.unknown_statements.len(), 3, "leading comment + trailing comment + weird_directive");

    let text2 = encode_obj(&snap1);
    let snap2 = decode_obj(&text2).expect("re-decode");

    assert_eq!(snap1.vertices, snap2.vertices);
    assert_eq!(snap1.texcoords, snap2.texcoords);
    assert_eq!(snap1.normals, snap2.normals);
    assert_eq!(snap1.faces, snap2.faces);
    assert_eq!(snap1.groups, snap2.groups);
    assert_eq!(snap1.objects, snap2.objects);
    assert_eq!(snap1.mtllib, snap2.mtllib);
    assert_eq!(snap1.usemtl, snap2.usemtl);
    assert_eq!(snap1.smoothing_groups, snap2.smoothing_groups);
    assert_eq!(snap1.unknown_statements.iter().map(|u| u.raw.clone()).collect::<Vec<_>>(), snap2.unknown_statements.iter().map(|u| u.raw.clone()).collect::<Vec<_>>(), "unknown-statement content and relative order must be retained");

    // 🔁 second-generation stability: a true fixed point from here on.
    let text3 = encode_obj(&snap2);
    let snap3 = decode_obj(&text3).expect("re-decode 2");
    assert_eq!(snap2, snap3, "decode/encode must be a fixed point from the second generation onward");
}

/// 🏷️ An `o` run that ends is closed with a bare `o`, so an object over a STRICT SUBSET of the
/// faces survives the text. Before the terminator existed the encoder had no way to express
/// "no object from here on" and the sticky `o Shell` ran to end-of-file, so re-decoding handed
/// back the object over ALL faces — narrowing an object's membership was a silent no-op in the
/// document even though the snapshot carried it (ticket
/// `26/08/23/END-TO-END-TESTING-REFACTOR`, `mutate-obj-3-0::mutate-set-object`).
#[test]
fn an_object_run_that_ends_is_closed_with_a_bare_o() {
    let fixture = "v 0 0 0\nv 1 0 0\nv 0 1 0\no Shell\nf 1 2 3\nf 2 3 1\nf 3 1 2\n";
    let mut snap = decode_obj(fixture).expect("decode");
    assert_eq!(snap.objects[0].faces, vec![0, 1, 2], "the fixture's own o run covers every face");

    snap.objects[0].faces = vec![0];
    let text = encode_obj(&snap);
    assert!(text.contains("\no\n"), "the end of the o run must be written as a bare `o`: {text}");
    let reread = decode_obj(&text).expect("re-decode");
    assert_eq!(reread.objects.len(), 1, "no second object may be invented: {:?}", reread.objects);
    assert_eq!(reread.objects[0].faces, vec![0], "the narrowed membership must survive the text");
}
//#endregion 🔖️CodecRetentionLaw

//#region 🔖️ConformanceLaws
/// 🧪️ P2-FG1: per-artifact conformance laws (recipe §4 item 6) — grammar/protocol
/// parseability, `Recognizer` against real fixtures AND real `print_op`/`print_diff` output,
/// `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff` bytes, and the
/// fixture-honesty round-trip. Dissolved out of `⚙️engine`'s own test region (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — same convention every prior pilot
/// wave (json/csv/zip/png/txt/binary) established.
mod conformance_laws {
    use super::*;
    use crate::schema::{diff, mutations, snapshot};
    use protocol::{DiffCodec, OpBinary, OpText};

    /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
    /// parse under the real dialect — independent of, and cheaper than, the two
    /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
    /// message).
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
    /// for the demo mesh — same preamble-stripped body reconstruction
    /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
    /// direct proof this artifact will pass that harness once graduated, not merely an
    /// analogue.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        let text = store::ArtifactDsl::print_dsl(&crate::engine::demo_obj_snapshot());
        let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
        let reconstructed = format!("{}\n{body}", envelope.envelope_id());
        assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
    }

    /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
    /// output for every representative `ObjMutation` variant (`mutations::demo_mutation_cases()`),
    /// including `SetSnapshot`'s whole nested `ObjSnapshot` tree, precisely field-by-field
    /// (this artifact's own leaf collections are all flat records, no `REST` fallback
    /// needed).
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);
        for mutation in mutations::demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
        }
    }

    /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
    /// output for every representative `ObjDiff` (`diff::demo_diff_cases()`), incl. the empty
    /// diff and a two-directional `between()` result exercising every index-/name-keyed
    /// collection triple and both tri-states.
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
    /// snapshot pack (`encode_pack`, envelope-unwrapped, matching how
    /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
    /// mutation's `encode_op`, and every demo diff's `encode_diff`. All three facets are
    /// plain `framing record` payloads (no `backward`/`jump`), so the ordinary
    /// `consumed == bytes.len()` law holds for all of them.
    #[semio_framework_async_macros::async_test]
    async fn protocol_walk_law() {
        let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let packed = store::ArtifactPack::encode_pack(&crate::engine::demo_obj_snapshot());
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
    /// `print_dsl`/`encode_pack` output of `demo_obj_snapshot()` — `parse_dsl(fixture) ==
    /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
    /// fixtures can never silently drift back to a fake again.
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
        const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

        let demo = crate::engine::demo_obj_snapshot();

        let parsed = <ObjSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
        assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_obj_snapshot()");
        assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_obj_snapshot()) drifted from the shipped .dsl.semio fixture");

        let decoded = <ObjSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
        assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_obj_snapshot()");
        assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_obj_snapshot()) drifted from the shipped .pack.semio fixture");
    }
}
//#endregion 🔖️ConformanceLaws
