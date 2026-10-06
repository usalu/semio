use super::*;
use protocol::{DiffBinary,DiffCodec,DiffText};

#[test]
fn canonical_storage_diff_text_and_binary_round_trip() {
    for diff in demo_diff_cases() {
        let printed = diff.print_diff();
        assert!(!printed.contains('\n'));
        assert_eq!(TiffDiff::parse_diff(&printed).expect("text decode"), diff);
        let encoded = diff.encode_diff().expect("binary encode");
        assert_eq!(TiffDiff::decode_diff(&encoded).expect("binary decode"), diff);
    }
}
