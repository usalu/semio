use super::*;
use protocol::Inference;

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = CsvSnapshot::default();
    assert_eq!(CsvInference::infer(&snapshot).expect("valid materialized inference fixture"), CsvInference::infer(&snapshot).expect("valid materialized inference fixture"));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(CsvInference::infer(&CsvSnapshot::default()).expect("valid materialized inference fixture"), CsvInference::default());
}

//#region 🔖️ConformanceLaws
/// 🧪️ P2-P1: grammar/protocol parseability, `Recognizer` against a real fixture, `walk_protocol`
/// against real `encode_pack`/`encode_op`/`encode_diff` bytes, and fixture honesty. Dissolved
/// out of the former `⚙️engine`'s own test region (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
mod conformance_laws {

    use crate::schema::snapshot::{self, CsvField, CsvRecord};
    use crate::{CsvDiff, CsvMutation};
    use protocol::{DiffBinary,DiffCodec,DiffText, OpBinary};

    /// 🧪️ P2-P1: `dsl::parse_grammar` + `dsl::Recognizer::compile` + `.recognize` against the
    /// real declared Record body after its independently checked envelope preamble.
    #[semio_framework_async_macros::async_test]
    async fn grammar_conformance_law() {
        let grammar_text = crate::standards::v_rfc4180::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO;
        let grammar = semio_framework_dsl::parse_grammar(grammar_text).expect("parse snapshot grammar");
        assert_eq!(grammar.dialect, semio_framework_dsl::SemioDialect::Grammar);
        let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
        let fixture = crate::examples::demo::PRIMARY_TEXT;
        let (_, body) = store::semio_format::split_text_preamble(fixture).expect("real preamble");
        let ok = recognizer.recognize(body).expect("recognize should not error");
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
        let snap = crate::standards::v_rfc4180::subsets::any::io::text::snapshot::demo_csv_snapshot();
        let pack_bytes = <snapshot::CsvSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let (_, payload) = store::semio_format::unwrap_binary(&pack_bytes).expect("unwrap_binary");
        let pack_protocol = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
        let trace = semio_framework_dsl::walk_protocol(&pack_protocol, &payload).expect("walk snapshot protocol");
        assert_eq!(trace.consumed, payload.len(), "snapshot protocol must consume the whole post-envelope payload");

        // Spr (mutations binary facet) — a real, non-trivial mutation.
        let mutation = CsvMutation::InsertRecord(crate::schema::mutations::insert_record::InsertRecord { index: 1, record: CsvRecord { fields: vec![CsvField { value: "brand-new".into(), quoted: true }] } });
        let op_bytes = <CsvMutation as OpBinary>::encode_op(&mutation).expect("encode_op");
        let spr_protocol = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
        let trace = semio_framework_dsl::walk_protocol(&spr_protocol, &op_bytes).expect("walk mutations protocol");
        assert_eq!(trace.consumed, op_bytes.len(), "mutations protocol must consume the whole op frame");

        // Diff binary facet.
        let mut before = snap.clone();
        let diff = crate::schema::mutations::apply_csv_mutation(&mut before, &mutation);
        let diff_bytes = <CsvDiff as DiffBinary>::encode_diff(diff.diff()).expect("encode_diff");
        let diff_protocol = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
        let trace = semio_framework_dsl::walk_protocol(&diff_protocol, &diff_bytes).expect("walk diff protocol");
        assert_eq!(trace.consumed, diff_bytes.len(), "diff protocol must consume the whole diff frame");
    }

    /// 🧪️ P2-P1 item 5: fixture honesty — the committed `.dsl.semio`/`.pack.semio` fixtures are
    /// genuinely `print_dsl`/`encode_pack` output of the SAME demo snapshot, round-tripping both
    /// ways (never allowed to silently drift again).
    #[semio_framework_async_macros::async_test]
    async fn fixture_honesty_law() {
        let demo = crate::standards::v_rfc4180::subsets::any::io::text::snapshot::demo_csv_snapshot();
        assert_eq!(<snapshot::CsvSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).unwrap(), demo);
        assert_eq!(<snapshot::CsvSnapshot as store::ArtifactDsl>::print_dsl(&demo), crate::examples::demo::PRIMARY_TEXT);

        assert_eq!(<snapshot::CsvSnapshot as store::ArtifactPack>::decode_pack(crate::examples::demo::PACK_BYTES).unwrap(), demo);
        assert_eq!(<snapshot::CsvSnapshot as store::ArtifactPack>::encode_pack(&demo), crate::examples::demo::PACK_BYTES.to_vec());
    }

    /// 🧪️ P2-P1 item 6: every committed grammar/protocol file for this standard genuinely
    /// parses under `dsl::parse_grammar`/`dsl::parse_protocol` — this artifact's own early
    /// warning, independent of the eventual repo-wide policy gate.
    #[semio_framework_async_macros::async_test]
    async fn committed_grammar_and_protocol_files_parse() {
        let g1 = semio_framework_dsl::parse_grammar(crate::standards::v_rfc4180::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO);
        assert!(g1.is_ok(), "snapshot grammar must parse: {g1:?}");
        let g2 = semio_framework_dsl::parse_grammar(crate::standards::v_rfc4180::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO);
        assert!(g2.is_ok(), "mutations grammar must parse: {g2:?}");
        let g3 = semio_framework_dsl::parse_grammar(crate::standards::v_rfc4180::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO);
        assert!(g3.is_ok(), "diff grammar must parse: {g3:?}");
        let p1 = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO);
        assert!(p1.is_ok(), "snapshot protocol must parse: {p1:?}");
        let p2 = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO);
        assert!(p2.is_ok(), "mutations protocol must parse: {p2:?}");
        let p3 = semio_framework_dsl::parse_protocol(crate::standards::v_rfc4180::subsets::any::io::binary::diff::COMPONENT_PROTOCOL_SEMIO);
        assert!(p3.is_ok(), "diff protocol must parse: {p3:?}");
    }
}
//#endregion 🔖️ConformanceLaws
