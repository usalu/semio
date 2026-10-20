use semio_s_artifact_stdio_semio_table_mutations::io::text::mutations::*;
use protocol::OpText;
#[cfg(test)] use semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation;

#[semio_framework_async_macros::async_test]
async fn op_text_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = <SemioTableMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");
    }
}
