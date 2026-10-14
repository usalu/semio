//! 🧪️ Facet conformance laws of the glTF snapshot, mutation and diff representations: every committed grammar and
//! protocol parses, recognizes real printed output, walks real encoded bytes, and the shipped demo fixtures are honest.
//! They span the model and the mutation vocabulary, so they live with the crate that depends on both.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAlphaMode;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::{diff, snapshot};
use crate::standards::v2_0::subsets::any::io::text::mutations as text_mutations;
use crate::standards::v2_0::subsets::any::io::binary::mutations as binary_mutations;
use crate::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::{ChangeMaterialAlphaModeMutation, GltfChangeMaterialAlphaModePayload};
use crate::standards::v2_0::subsets::any::schema::mutations::GltfMutation;
use protocol::{DiffBinary,DiffCodec,DiffText, Mutation, OpBinary, OpText};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn alpha_mode_mutation() -> GltfMutation {
    GltfMutation::ChangeMaterialAlphaMode(ChangeMaterialAlphaModeMutation::Apply(GltfChangeMaterialAlphaModePayload { material: 0, alpha_mode: GltfAlphaMode::Mask }))
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn material_snapshot() -> GltfSnapshot {
    let mut snapshot = GltfSnapshot::default();
    snapshot.document.materials.push(Default::default());
    snapshot
}

/// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
/// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
/// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
#[semio_framework_async_macros::async_test]
async fn committed_facet_files_parse() {
    for (label, text) in [("snapshot grammar", semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", text_mutations::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO)] {
        let grammar = semio_framework_dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
        assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
    }
    for (label, text) in [("snapshot protocol", semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", binary_mutations::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO)] {
        semio_framework_dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
    }
}

/// ✅️ The authored snapshot grammar recognizes the actual canonical envelope and body.
#[semio_framework_async_macros::async_test]
async fn grammar_conformance_law() {
    let grammar = semio_framework_dsl::parse_grammar(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
    let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
    let text = pack::snapshot::ArtifactDsl::print_dsl(&semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::demo_gltf_snapshot());
    let (_, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
    for input in [text.as_str(), body] {
        assert!(recognizer.recognize(input).expect("recognize"), "grammar did not recognize current owned snapshot:\n{input}");
    }
}

/// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes the canonical generic
/// envelope, independently of the registered command payload shape.
#[semio_framework_async_macros::async_test]
async fn ops_grammar_conformance_law() {
    let grammar = semio_framework_dsl::parse_grammar(text_mutations::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
    let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
    let forward = alpha_mode_mutation();
    let inverse = forward.inverse(&material_snapshot()).expect("valid retained mutation inverse fixture").pop().expect("inverse envelope");
    for mutation in [forward, inverse] {
        let printed = mutation.print_op();
        assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn aggregate_transport_laws() {
    let mutation = alpha_mode_mutation();
    let text = mutation.print_op();
    assert_eq!(GltfMutation::parse_op(&text).expect("text round trip"), mutation);
    let binary = mutation.encode_op().expect("binary encode");
    assert_eq!(GltfMutation::decode_op(&binary).expect("binary round trip"), mutation);
    let mut trailing = binary;
    trailing.push(0);
    assert!(GltfMutation::decode_op(&trailing).is_err(), "binary envelope accepted trailing data");
}

/// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
/// for every representative `GltfDiff` (`diff::demo_diff_cases()`), incl. the empty
/// (all-`None`) diff and the fully-populated rich diff.
#[semio_framework_async_macros::async_test]
async fn diff_grammar_conformance_law() {
    let grammar = semio_framework_dsl::parse_grammar(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
    let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
    for d in diff::demo_diff_cases() {
        let printed = d.print_diff();
        assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
    }
}

/// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
/// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
/// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), a generic mutation
/// envelope's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
#[semio_framework_async_macros::async_test]
async fn protocol_walk_law() {
    let pack_spec = semio_framework_dsl::parse_protocol(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
    let packed = pack::snapshot::ArtifactPack::encode_pack(&semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::demo_gltf_snapshot());
    let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
    let trace = semio_framework_dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
    assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

    let op_spec = semio_framework_dsl::parse_protocol(binary_mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
    let forward = alpha_mode_mutation();
    let inverse = forward.inverse(&material_snapshot()).expect("valid retained mutation inverse fixture").pop().expect("inverse envelope");
    for mutation in [forward, inverse] {
        let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
        let trace = semio_framework_dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
    }

    let diff_spec = semio_framework_dsl::parse_protocol(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
    for d in diff::demo_diff_cases() {
        let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
        let trace = semio_framework_dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
        assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
    }
}

/// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
/// `print_dsl`/`encode_pack` output of `demo_gltf_snapshot()` — `parse_dsl(fixture) ==
/// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
/// fixtures can never silently drift back to a fake (the pre-FG3 `{"hello":"stdio.gltf",
/// "n":1}` stub this program's own recipe explicitly calls out as the wrong shape).
#[semio_framework_async_macros::async_test]
async fn fixture_honesty_law() {
    const FIXTURE_DSL: &str = include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");
    const FIXTURE_PACK: &[u8] = include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio");

    let demo = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::demo_gltf_snapshot();

    let parsed = <GltfSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
    assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_gltf_snapshot()");
    assert_eq!(pack::snapshot::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_gltf_snapshot()) drifted from the shipped .dsl.semio fixture");

    let decoded = <GltfSnapshot as pack::snapshot::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
    assert_eq!(decoded.schema, demo.schema, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s schema");
    assert_eq!(decoded.document, demo.document, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s document");
    assert_eq!(decoded.buffers, demo.buffers, "shipped .pack.semio fixture does not decode back to demo_gltf_snapshot()'s buffers");
    assert_eq!(decoded.source_form, demo.source_form, "owned pack must preserve source form");
    assert_eq!(pack::snapshot::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_gltf_snapshot()) drifted from the shipped .pack.semio fixture");
}
