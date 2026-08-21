//! 💡️ Csv inference schema — the fourth schema family alongside snapshot/diff/mutations
//! (ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING). Directory
//! shape mirrors `🧬️mutations/`: this file is the family-root assembly (never mod's/includes the
//! slug dirs directly — `📦️glue.rs` is the sole mounting mechanism, same as mutations); each named
//! inference gets its own `<emoji><slug>/` child (currently: `🧾outline/`).

use crate::artifacts::csv::CsvSnapshot;
use schema::ArtifactSchema;
use semio_framework_plugin::ArtifactInferrer;
use serde::{Deserialize, Serialize};

use super::outline::CsvOutline;

//#region 🔖️Inference
/// 💡️ Everything inferable from a csv snapshot. One field per named inference under
/// `💡️inferences/` (currently: `outline`, backed by the `🧾outline/` slug dir).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv.inference")]
pub struct CsvInference {
    #[derived]
    pub outline: CsvOutline,
}

impl protocol::Inference<CsvSnapshot> for CsvInference {
    async fn infer(snapshot: &CsvSnapshot) -> Self {
        Self { outline: CsvOutline::compute(snapshot) }
    }
}

impl protocol::InferenceSpec<CsvSnapshot> for CsvInference {
    async fn inference_schema_id() -> &'static str {
        "s.stdio.csv.inference"
    }
    async fn schema_version() -> u32 {
        1
    }
    async fn fields() -> &'static [protocol::InferenceFieldSpec] {
        &[protocol::InferenceFieldSpec { id: "s.stdio.csv.inference.outline", reads: &["records", "hasHeader"] }]
    }
}
//#endregion 🔖️Inference

//#region 🔖️ArtifactInferrer
impl ArtifactInferrer for crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::CsvBuilder {
    type Snapshot = CsvSnapshot;
    type Inference = CsvInference;
}
//#endregion 🔖️ArtifactInferrer

//#region 🔖️Descriptor
/// 💡️ Registers `s.stdio.csv.inference`'s facet leaves into the OS-wide inference catalog — call
/// once at plugin init, alongside `csv_artifact_schema_descriptor`'s registration.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn csv_artifact_inference_descriptor() -> schema::ArtifactInferenceDescriptor {
    schema::ArtifactInferenceDescriptor {
        id: "s.stdio.csv.inference",
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

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snapshot = CsvSnapshot::default();
        assert_eq!(CsvInference::infer(&snapshot), CsvInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(CsvInference::infer(&CsvSnapshot::default()), CsvInference::default());
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-P1: grammar/protocol parseability, `Recognizer` against a real fixture, `walk_protocol`
    /// against real `encode_pack`/`encode_op`/`encode_diff` bytes, and fixture honesty. Dissolved
    /// out of the former `⚙️engine`'s own test region (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    mod conformance_laws {
        
        use crate::artifacts::csv::schema::snapshot::{self, CsvField, CsvRecord};
        use crate::artifacts::csv::{CsvDiff, CsvMutation};
        use protocol::{DiffCodec, OpBinary};

        /// 🧪️ P2-P1: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against the
        /// REAL fixture body — the snapshot text facet's own real RFC 4180 grammar recognizes the
        /// genuine `print_dsl` output (envelope-id-normalized, matching how
        /// `dsl::fixture_sweep::m5_handcrafted_grammar_conformance::dsl_body_from_fixture` feeds the
        /// Recognizer, mirrored here so this law does not depend on the framework's own harness).
        #[semio_framework_async_macros::async_test]
        async fn grammar_conformance_law() {
            let grammar_text = snapshot::text::COMPONENT_GRAMMAR_SEMIO;
            let grammar = dsl::parse_grammar(grammar_text).expect("parse snapshot grammar");
            assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar);
            let recognizer = dsl::Recognizer::compile(&grammar);
            let fixture = crate::artifacts::csv::examples::demo::PRIMARY_TEXT;
            let (envelope, body) = store::semio_format::split_text_preamble(fixture).expect("real preamble");
            let normalized = format!("{}\n{body}", envelope.envelope_id());
            let ok = recognizer.recognize(&normalized).expect("recognize should not error");
            assert!(ok, "snapshot grammar must recognize the real demo fixture body");
        }

        /// 🧪️ P2-P1: `dsl::parse_protocol` + `dsl::walk_protocol` against REAL bytes for all three
        /// binary facets (Pack/Spr/Diff), asserting `consumed == bytes.len()` exactly (the walker's
        /// own law) — snapshot's Pack facet walks the post-`unwrap_binary` payload of a genuine
        /// `encode_pack` call; mutations' Spr facet walks a genuine `encode_op` frame; diff's own
        /// protocol facet walks a genuine `encode_diff` frame.
        #[semio_framework_async_macros::async_test]
        async fn protocol_walk_law() {
            // Pack (snapshot binary facet).
            let snap = snapshot::demo_csv_snapshot();
            let pack_bytes = <snapshot::CsvSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let (_, payload) = store::semio_format::unwrap_binary(&pack_bytes).expect("unwrap_binary");
            let pack_protocol = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let trace = dsl::walk_protocol(&pack_protocol, &payload).expect("walk snapshot protocol");
            assert_eq!(trace.consumed, payload.len(), "snapshot protocol must consume the whole post-envelope payload");

            // Spr (mutations binary facet) — a real, non-trivial mutation.
            let mutation = CsvMutation::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } };
            let op_bytes = <CsvMutation as OpBinary>::encode_op(&mutation).expect("encode_op");
            let spr_protocol = dsl::parse_protocol(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            let trace = dsl::walk_protocol(&spr_protocol, &op_bytes).expect("walk mutations protocol");
            assert_eq!(trace.consumed, op_bytes.len(), "mutations protocol must consume the whole op frame");

            // Diff binary facet.
            let mut before = snap.clone();
            let diff = crate::artifacts::csv::schema::mutations::apply_csv_mutation(&mut before, &mutation);
            let diff_bytes = <CsvDiff as DiffCodec>::encode_diff(diff.diff()).expect("encode_diff");
            let diff_protocol = dsl::parse_protocol(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            let trace = dsl::walk_protocol(&diff_protocol, &diff_bytes).expect("walk diff protocol");
            assert_eq!(trace.consumed, diff_bytes.len(), "diff protocol must consume the whole diff frame");
        }

        /// 🧪️ P2-P1 item 5: fixture honesty — the committed `.dsl.semio`/`.pack.semio` fixtures are
        /// genuinely `print_dsl`/`encode_pack` output of the SAME demo snapshot, round-tripping both
        /// ways (never allowed to silently drift again).
        #[semio_framework_async_macros::async_test]
        async fn fixture_honesty_law() {
            let demo = snapshot::demo_csv_snapshot();
            assert_eq!(<snapshot::CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::artifacts::csv::examples::demo::PRIMARY_TEXT).unwrap(), demo);
            assert_eq!(<snapshot::CsvSnapshot as store::ArtifactDsl>::print_dsl(&demo), crate::artifacts::csv::examples::demo::PRIMARY_TEXT);

            assert_eq!(<snapshot::CsvSnapshot as store::ArtifactPack>::decode_pack(crate::artifacts::csv::examples::demo::PACK_BYTES).unwrap(), demo);
            assert_eq!(<snapshot::CsvSnapshot as store::ArtifactPack>::encode_pack(&demo), crate::artifacts::csv::examples::demo::PACK_BYTES.to_vec());
        }

        /// 🧪️ P2-P1 item 6: every committed grammar/protocol file for this standard genuinely
        /// parses under `dsl::parse_grammar`/`dsl::parse_protocol` — this artifact's own early
        /// warning, independent of the eventual repo-wide policy gate.
        #[semio_framework_async_macros::async_test]
        async fn committed_grammar_and_protocol_files_parse() {
            let g1 = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO);
            assert!(g1.is_ok(), "snapshot grammar must parse: {g1:?}");
            let g2 = dsl::parse_grammar(crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO);
            assert!(g2.is_ok(), "mutations grammar must parse: {g2:?}");
            let g3 = dsl::parse_grammar(crate::artifacts::csv::schema::diff::text::COMPONENT_GRAMMAR_SEMIO);
            assert!(g3.is_ok(), "diff grammar must parse: {g3:?}");
            let p1 = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO);
            assert!(p1.is_ok(), "snapshot protocol must parse: {p1:?}");
            let p2 = dsl::parse_protocol(crate::artifacts::csv::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO);
            assert!(p2.is_ok(), "mutations protocol must parse: {p2:?}");
            let p3 = dsl::parse_protocol(crate::artifacts::csv::schema::diff::binary::COMPONENT_PROTOCOL_SEMIO);
            assert!(p3.is_ok(), "diff protocol must parse: {p3:?}");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
