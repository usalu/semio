//! 💡️ DeflateInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🪟window/`). Deliberately NOT a
//! `🗃entries` census like `🎒️zip`'s: RFC1950 wraps exactly one deflate-compressed member, not a
//! multi-entry container — forcing an "entries" shape onto a single-stream zlib payload would be
//! dishonest, so this facet instead derives real RFC1950 zlib HEADER semantics (CMF window size,
//! FLG.FLEVEL, FDICT) that zip has no equivalent of at all.

use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::window::{compute_deflate_window, DeflateWindow};

//#region 🔖️Inference
/// 💡️ Everything inferable from a deflate snapshot. One field per named inference under
/// `💡️inferences/` (currently: `window`, backed by the `🪟window/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.deflate.inference")]
pub struct DeflateInference {
    #[derived]
    pub window: DeflateWindow,
}

impl protocol::Inference<DeflateSnapshot> for DeflateInference {
    fn infer(snapshot: &DeflateSnapshot) -> Self {
        Self { window: compute_deflate_window(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — `DeflateSnapshot::default()` is a real RFC1950
/// normal form (`compression_method: 8`, `window_bits: 7`), not a zeroed struct, so a derived
/// all-zero `Default` would disagree with the honest compute and break the law.
impl Default for DeflateInference {
    fn default() -> Self {
        <Self as protocol::Inference<DeflateSnapshot>>::infer(&DeflateSnapshot::default())
    }
}

impl protocol::InferenceSpec<DeflateSnapshot> for DeflateInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.deflate.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec {
            id: "s.stdio.deflate.inference.window",
            reads: &["windowBits", "compressionLevelHint", "dictId", "payload"],
        }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `window` is a fixed-field header read plus a single fold over
/// `payload`, already O(n) in payload size with no honest per-entity incremental decomposition (a
/// merkle dep-chain over one flat `Vec<u8>` payload costs more than the fold it would cache) —
/// the default `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::DeflateBuilder {
    type Snapshot = DeflateSnapshot;
    type Inference = DeflateInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.deflate.inference`'s facet leaves into the OS-wide inference catalog —
/// call once at plugin init, alongside `deflate_artifact_schema_descriptor`'s registration.
pub fn deflate_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.deflate.inference",
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
    use crate::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::{empty_deflate_snapshot, demo_deflate_snapshot};

    #[test]
    fn inference_determinism_law() {
        let snapshot = DeflateSnapshot::default();
        assert_eq!(DeflateInference::infer(&snapshot), DeflateInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(DeflateInference::infer(&DeflateSnapshot::default()), DeflateInference::default());
    }

    //#region 🔖️ConformanceLaws
    /// 🦑 Moved out of the former `⚙️engine`'s own test module (ticket 26/08/12/ENGINELESS-
    /// ARTIFACTS-AND-APP-STATE-MACHINES) — per-artifact conformance laws (recipe §4 deliverable
    /// item 6) — grammar/protocol parseability, `Recognizer` against real fixtures AND real
    /// `print_op`/`print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
    /// `encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::deflate::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below.
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
        /// for the demo snapshot (a non-empty payload + a real preset-dictionary id).
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_deflate_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `DeflateMutation` demo case (`mutations::demo_mutation_cases()`).
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
        /// output for every representative `DeflateDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty-line diff and both `dict_id` tri-state directions.
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
        /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
        /// bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_deflate_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_deflate_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_deflate_snapshot();

            let parsed = <DeflateSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_deflate_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_deflate_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <DeflateSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_deflate_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_deflate_snapshot()) drifted from the shipped .pack.semio fixture");
        }

        /// ✅️ `schema_spec_registration_resolves`: `register_schema_specs` genuinely resolves the
        /// snapshot schema id through `dsl::registry::full_resolver()` once called (real
        /// `DeflateSnapshot::__dsl_spec`, not fabricated — see that fn's own doc comment for why
        /// the diff id is deliberately NOT registered).
        #[test]
        #[cfg(not(target_arch = "wasm32"))]
        fn schema_spec_registration_resolves() {
            use dsl::os_pack::cli::SchemaResolver;
            crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::register_schema_specs();
            let resolver = dsl::registry::full_resolver();
            assert!(resolver.resolve("stdio.deflate").is_some(), "stdio.deflate must resolve");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
