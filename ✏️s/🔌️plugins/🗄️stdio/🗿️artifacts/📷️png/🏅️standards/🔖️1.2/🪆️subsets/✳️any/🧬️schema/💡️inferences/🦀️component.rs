//! 💡️ Png inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `📐dimensions/`).

use crate::artifacts::png::PngSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

use super::dimensions::{compute_png_dimensions, PngDimensions};

//#region 🔖️Inference
/// 💡️ Everything inferable from a png snapshot. One field per named inference under
/// `💡️inferences/` (currently: `dimensions`, backed by the `📐dimensions/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.inference")]
pub struct PngInference {
    #[derived]
    pub dimensions: PngDimensions,
}

impl protocol::Inference<PngSnapshot> for PngInference {
    fn infer(snapshot: &PngSnapshot) -> Self {
        Self { dimensions: compute_png_dimensions(snapshot) }
    }
}

/// 🌱 Hand-fixed to agree with `infer(&PngSnapshot::default())` rather than a naive
/// `#[derive(Default)]` — `PngSnapshot::default()`'s `bitDepth: 8`/`colorType: Rgba` disagree with
/// a structurally-derived all-zero `PngDimensions`, the same "match `infer` of the real default,
/// don't derive structurally" trick as `AddInference`'s hand-written `Default` in
/// `📡️spr/🎮️command/🦀️component.rs`.
impl Default for PngInference {
    fn default() -> Self {
        <Self as protocol::Inference<PngSnapshot>>::infer(&PngSnapshot::default())
    }
}

impl protocol::InferenceSpec<PngSnapshot> for PngInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.png.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.png.inference.dimensions", reads: &["width", "height", "bitDepth", "colorType"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here (an IHDR-field read is already O(1)) — the default `infer_cached`
/// passthrough (`ArtifactInferrer::infer_cached`) is exact.
impl semio_framework_plugin::ArtifactInferrer for crate::artifacts::png::standards::v1_2::subsets::any::schema::PngBuilder {
    type Snapshot = PngSnapshot;
    type Inference = PngInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.png.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `png_artifact_schema_descriptor`'s registration.
pub fn png_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.png.inference",
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

    #[test]
    fn inference_determinism_law() {
        let snapshot = PngSnapshot::default();
        assert_eq!(PngInference::infer(&snapshot), PngInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(PngInference::infer(&PngSnapshot::default()), PngInference::default());
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P2: per-artifact conformance laws (item 6 of the deliverable list) — grammar/
    /// protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
    /// `encode_diff` bytes, and the fixture-honesty round-trip. Relocated verbatim from
    /// `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
    /// rule 4: conformance laws live in `🧬️schema/💡️inferences/`) — none of these depend on the
    /// real byte codec (`encode_png`/`decode_png`, now in `🚪️io/`), only on `ArtifactDsl`/
    /// `ArtifactPack`/grammar-protocol parsing, so the split is clean.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::png::schema::{demo_png_snapshot, diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio`
        /// files parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a
        /// clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — PNG has no
        /// textual syntax of its own, see that file's own doc comment) recognizes real
        /// `print_dsl` output for the demo snapshot — same preamble-stripped body
        /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture`
        /// uses, so this is a direct proof this artifact will pass that harness once
        /// graduated, not merely an analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_png_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `PngMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// output for every representative `PngDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty diff and every tri-state/collection-triple shape.
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
        /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed
        /// == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_png_snapshot());
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

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are
        /// GENUINE `print_dsl`/`encode_pack` output of `demo_png_snapshot()` —
        /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and
        /// the pack twin — so the fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_png_snapshot();

            let parsed = <PngSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_png_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_png_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PngSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_png_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_png_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
