//! 💡️ ZipInference — the fourth schema family alongside snapshot/diff/mutations (ticket
//! 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory shape
//! mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the slug
//! dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🗃entries/`, a real census over the
//! archive's decompressed `entries` — the natural container-level facet a ZIP central directory
//! already exists to answer).

use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::entries::{compute_zip_entries, ZipEntries};

//#region 🔖️Inference
/// 💡️ Everything inferable from a zip snapshot. One field per named inference under
/// `💡️inferences/` (currently: `entries`, backed by the `🗃entries/` slug dir).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.inference")]
pub struct ZipInference {
    #[derived]
    pub entries: ZipEntries,
}

impl protocol::Inference<ZipSnapshot> for ZipInference {
    fn infer(snapshot: &ZipSnapshot) -> Self {
        Self { entries: compute_zip_entries(snapshot) }
    }
}

/// 🌱 Defined in terms of `infer` (not derived) — keeps the law correct regardless of whether
/// `ZipSnapshot::default()`'s `entries` ever stop being empty.
impl Default for ZipInference {
    fn default() -> Self {
        <Self as protocol::Inference<ZipSnapshot>>::infer(&ZipSnapshot::default())
    }
}

impl protocol::InferenceSpec<ZipSnapshot> for ZipInference {
    fn inference_schema_id() -> &'static str {
        "s.stdio.zip.inference"
    }
    fn schema_version() -> u32 {
        1
    }
    fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.zip.inference.entries", reads: &["entries"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
/// 💡️ No `InferredField`s here — `entries` is a single fold over `entries` (count, byte-size sum,
/// content digest), already O(n) in entry count with no honest per-entity incremental
/// decomposition worth a merkle dep-chain over one flat `Vec<ZipEntry>` — the default
/// `infer_cached` passthrough is exact.
impl ArtifactInferrer for crate::artifacts::zip::standards::v2_0::subsets::any::schema::ZipBuilder {
    type Snapshot = ZipSnapshot;
    type Inference = ZipInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.zip.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `zip_artifact_schema_descriptor`'s registration.
pub fn zip_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.zip.inference",
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
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::demo_zip_snapshot;

    #[test]
    fn inference_determinism_law() {
        let snapshot = ZipSnapshot::default();
        assert_eq!(ZipInference::infer(&snapshot), ZipInference::infer(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(ZipInference::infer(&ZipSnapshot::default()), ZipInference::default());
    }

    //#region 🔖️ConformanceLaws
    /// 🦑 Moved out of the former `⚙️engine`'s own test module (ticket 26/08/12/ENGINELESS-
    /// ARTIFACTS-AND-APP-STATE-MACHINES) — per-artifact conformance laws (item 6 of the deliverable
    /// list) — grammar/protocol parseability, `Recognizer` against real fixtures AND real
    /// `print_op`/`print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/
    /// `encode_diff` bytes, and the fixture-honesty round-trip.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::zip::schema::{diff, mutations, snapshot};
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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar (a hex-dump grammar — `stdio.zip` is
        /// binary-native) recognizes real `print_dsl` output for the demo archive — same
        /// preamble-stripped body reconstruction `m5_handcrafted_grammar_conformance`'s own
        /// `dsl_body_from_fixture` uses, so this is a direct proof this artifact will pass that
        /// harness once graduated, not merely an analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_zip_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every representative `ZipMutation` variant (`mutations::demo_mutation_cases()`),
        /// including the three genuinely-recursive-payload variants (`SetSnapshot`/`AddEntry`/
        /// `SetEntryExtra`), which the grammar honestly models via `REST` (see that file's own doc
        /// comment) — this law proves `REST` genuinely swallows their real nested-block/list output,
        /// not just that the simple scalar-only variants parse.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every representative `ZipDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
        /// a two-directional `between()` result exercising the full `entries` collection triple and
        /// the tri-state `unix_mtime`.
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
        /// snapshot pack (`encode_pack`, envelope-unwrapped, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`.
        ///
        /// The snapshot/pack case does NOT assert `consumed == bytes.len()` — per M2's own
        /// documented exception (`walk_protocol`'s doc comment, `📖️grammar/🦀️component.rs`), a
        /// protocol that performs a `backward`/`jump` (ours does, twice: EOCD backward-scan +
        /// central-directory jump) is no longer required to land on exactly EOF, since the bytes
        /// between the final block's landing point and EOF are validly described by AN EARLIER
        /// block the walk already visited (here: the `backward eocd` block, which already fully
        /// captured the EOCD's own fields before the final `central_directory` repeat's sentinel
        /// match re-touches its first 4 bytes only to terminate cleanly). The op/diff cases declare
        /// neither block, so the ordinary `consumed == bytes.len()` law holds for them exactly.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_zip_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range position: {} (len {})", trace.consumed, inner.len());

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
        /// `print_dsl`/`encode_pack` output of `demo_zip_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_zip_snapshot();

            let parsed = <ZipSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_zip_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_zip_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <ZipSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_zip_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_zip_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
