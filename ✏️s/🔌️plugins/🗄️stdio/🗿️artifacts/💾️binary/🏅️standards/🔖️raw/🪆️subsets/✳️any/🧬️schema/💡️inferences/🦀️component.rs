//! 💡️ BinaryInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📏extent/`). Deliberately NOT a
//! `🗃entries` census like `🎒️zip`'s: `BinarySnapshot` is a single opaque `bytes: Vec<u8>` blob —
//! genuinely the most honest, minimal container in this entire family, with no entry/chunk/box
//! structure of any kind to census. Forcing an "entries" shape onto it would fabricate structure
//! this format doesn't have; this facet instead reports exactly what an opaque byte blob honestly
//! supports — its real extent (byte length, emptiness) plus a real content digest.

use crate::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::extent::{compute_binary_extent, BinaryExtent};

//#region 🔖️Inference
/// 💡️ Everything inferable from a binary snapshot. One field per named inference under
/// `💡️inferences/` (currently: `extent`, backed by the `📏extent/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.binary.inference")]
pub struct BinaryInference {
    #[derived]
    pub extent: BinaryExtent,
}

impl protocol::Inference<BinarySnapshot> for BinaryInference {
    fn infer(snapshot: &BinarySnapshot) -> Self {
        Self { extent: compute_binary_extent(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `BinarySnapshot::default()`'s `bytes` ever stop being empty.
impl Default for BinaryInference {
    fn default() -> Self {
        <Self as protocol::Inference<BinarySnapshot>>::infer(&BinarySnapshot::default())
    }
}

impl protocol::InferenceSpec<BinarySnapshot> for BinaryInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.binary.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.binary.inference.extent", reads: &["bytes"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `extent` is a single length read plus a fold over `bytes`,
/// already O(n) in byte count with no honest per-entity incremental decomposition (there is no
/// "entity" to decompose in an opaque byte blob) — the default `infer_cached` passthrough is
/// exact.
impl ArtifactInferrer for crate::artifacts::binary::standards::v_raw::subsets::any::schema::BinaryBuilder {
    type Snapshot = BinarySnapshot;
    type Inference = BinaryInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.binary.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `binary_artifact_schema_descriptor`'s registration.
pub fn binary_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.binary.inference",
        inference: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use protocol::Inference;
    use crate::artifacts::binary::standards::v_raw::subsets::any::schema::{empty_binary_snapshot, demo_binary_snapshot};
    use crate::artifacts::binary::STDIO_BINARY_DOCUMENT_SCHEMA;

    #[test]
    fn inference_determinism_law() {
        let snapshot = BinarySnapshot::default();
        assert_eq!(BinaryInference::infer(&snapshot), BinaryInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(BinaryInference::infer(&BinarySnapshot::default()), BinaryInference::default());
    }

    //#region 🦑️DissolvedEngineTests
    /// 🦑 Moved out of the former `⚙️engine`'s own test module (ticket 26/08/12/ENGINELESS-
    /// ARTIFACTS-AND-APP-STATE-MACHINES) — conformance laws, field sweeps, and pure
    /// snapshot-round-trip tests, kept together per that ticket's own destination rule.
    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_binary_snapshot();
        assert_eq!(snapshot.schema, STDIO_BINARY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ `codec_retention_law`: decode→encode is byte-preserving on real fixtures, incl. bytes
    /// that are themselves invalid UTF-8 (the hex DSL layer never interprets payload bytes as
    /// text, so this is a real test of the hex codec, not just the binary-pack envelope).
    #[test]
    fn codec_retention_law() {
        for bytes in [
            vec![],
            vec![0x00, 0x01, 0xFF, 0xFE],
            (0u8..=255).collect::<Vec<u8>>(),
        ] {
            let snap = BinarySnapshot { bytes: bytes.clone(), ..Default::default() };
            let dsl_text = store::ArtifactDsl::print_dsl(&snap);
            let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
            assert_eq!(parsed, snap, "dsl round-trip mismatch for {bytes:?}");
            let packed = store::ArtifactPack::encode_pack(&snap);
            let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode");
            assert_eq!(decoded, snap, "pack round-trip mismatch for {bytes:?}");
        }
    }

    //#region 🔖️FieldSweep
    /// 🧹 Canonical "every mutable field differs" snapshot A.
    fn sweep_a() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }
    }

    /// 🧹 Canonical "every mutable field differs" snapshot B: an insert-only region (bytes 100
    /// inserted mid-buffer), a pure-removal region (bytes 3,4 dropped), and a pure-replacement
    /// region (byte 8 → 88) -- one splice can't express all three at once, exercising the
    /// splice mechanism's full range per the artifact's own field-sweep note.
    fn sweep_b() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 100, 5, 6, 7, 88] }
    }

    /// 🧪️ `field_sweep`: THE acceptance criterion. `between` round-trips both directions, the
    /// splice list is non-empty (the only "field" a splice-list diff has), and `between(a,a)`
    /// is empty.
    #[test]
    fn field_sweep_covers_every_byte_level_change() {
        use protocol::os_spr::command::DiffAlgebra;
        use protocol::MutationDiff;
        use crate::artifacts::binary::standards::v_raw::subsets::any::schema::diff::BinaryDiff;
        let a = sweep_a();
        let b = sweep_b();

        let ab = BinaryDiff::between(&a, &b);
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) must equal b");
        let ba = BinaryDiff::between(&b, &a);
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(!ab.splices.is_empty(), "sweep diff must carry at least one splice");

        // 🔬️ Exercise insert/remove/replace explicitly via hand-built splices (not just the
        // minimal `between` form) to prove the mechanism itself, not just this one pair.
        let hand_built = BinaryDiff {
            splices: vec![
                crate::artifacts::binary::standards::v_raw::subsets::any::schema::diff::ByteSplice { offset: 2, remove_len: 2, insert: vec![100] }, // replace+shrink
                crate::artifacts::binary::standards::v_raw::subsets::any::schema::diff::ByteSplice { offset: 7, remove_len: 1, insert: vec![88] },  // pure replace
            ],
        };
        assert_eq!(hand_built.apply(&a), b);

        assert!(BinaryDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️FieldSweep

    #[test]
    fn demo_snapshot_round_trip() {
        let snap = demo_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🦑️DissolvedEngineTests

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P3: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::binary::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an
        /// analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_binary_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");

            // 🔬️ Also the empty-bytes case (`BinarySnapshot::default()`), exercising the `hex`
            // macro's zero-width match.
            let empty_text = store::ArtifactDsl::print_dsl(&empty_binary_snapshot());
            let (empty_envelope, empty_body) = store::semio_format::split_text_preamble(&empty_text).expect("split preamble");
            let empty_reconstructed = format!("{}\n{empty_body}", empty_envelope.envelope_id());
            assert!(recognizer.recognize(&empty_reconstructed).expect("recognize"), "grammar did not recognize empty dsl body:\n{empty_reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `BinaryMutation` variant (`mutations::demo_mutation_cases()`).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `BinaryDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty (no-splices) diff and a multi-splice diff with a zero-length no-op splice.
        #[test]
        fn diff_grammar_conformance_law() {
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
        /// bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_binary_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_binary_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_binary_snapshot();

            let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_binary_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_binary_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_binary_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_binary_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
