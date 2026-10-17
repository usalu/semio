#[allow(unused_imports)]
use crate::SvgSnapshot;
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use protocol::command::DiffAlgebra;
#[allow(unused_imports)]
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
#[allow(unused_imports)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype;
#[allow(unused_imports)]
use semio_s_artifact_stdio_xml::schema::snapshot::XmlQuote;
use crate::standards::v1_1::subsets::base::schema::diff::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

/// 🧪️ `DiffCodec` round-trip laws over the hand-rolled `SvgDiff` grammar — exercises the
/// recursive enum tree (`Element`/`Text`/`Replace` `SvgNodeDiff` variants), both top-level
/// tri-states, attribute add/remove/modify, and nested child add/remove/modify. Reuses
/// `demo_diff_cases()` (the single prolog of truth also consumed by `⚙️engine/🦀️.rs`'s
/// `diff_grammar_conformance_law`/`protocol_walk_law`).
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    for d in demo_diff_cases() {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SvgDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SvgDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
