use super::super::TestSnapshot;
use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
#[test]
fn direct_leaves_preserve_generic_document_codecs_and_laws() {
    let base = TestSnapshot { count: 3, label: "before".into() };
    for mutation in [TestMutation::SetCount(SetCount { value: -4 }), TestMutation::SetLabel(SetLabel { value: "quote \" newline\n☃".into() })] {
        assert_eq!(TestMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(TestMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        let after = mutation.diff(&base).diff().apply(&base).unwrap();
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(inverse[0].diff(&after).diff().apply(&after).unwrap(), base);
    }
}
