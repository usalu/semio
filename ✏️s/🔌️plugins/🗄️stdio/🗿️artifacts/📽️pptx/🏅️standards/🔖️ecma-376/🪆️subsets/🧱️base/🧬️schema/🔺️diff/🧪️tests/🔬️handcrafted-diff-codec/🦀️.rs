use semio_s_artifact_stdio_pptx_base_model::standards::v_ecma_376::subsets::base::schema::diff::*;
use protocol::{command::DiffAlgebra, DiffBinary,DiffCodec,DiffText, MutationDiff};

#[test]
fn canonical_diff_cases_roundtrip_text_and_binary() {
    for diff in demo_diff_cases() {
        assert_eq!(PptxDiff::parse_diff(&diff.print_diff()).unwrap(), diff);
        assert_eq!(PptxDiff::decode_diff(&diff.encode_diff().unwrap()).unwrap(), diff);
    }
}
