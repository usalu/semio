use super::super::{apply_txt_mutation, TxtMutation};
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};
#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<SetTrailingNewlineMutation as MutationLeaf>::DESCRIPTOR))).unwrap(), expected);
    let provenance = <SetTrailingNewlineMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️set-trailing-newline");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️set-trailing-newline/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↩️set-trailing-newline/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}
#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetTrailingNewlineMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "set-trailing-newline");
}

#[test]
fn inverse_and_root_codecs_restore_a_visible_terminator() {
    let base = TxtSnapshot { lines: vec!["a".into()], trailing_newline: false, line_ending: LineEnding::Lf, ..Default::default() };
    let mutation = TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: true });
    let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse, vec![TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: false })]);
    let mut after = base.clone();
    assert!(apply_txt_mutation(&mut after, &mutation).messages().is_empty());
    assert_eq!(TxtSnapshot::from_body(&after.to_body()), after);
    for step in inverse {
        assert_eq!(TxtMutation::parse_op(&step.print_op()).unwrap(), step);
        assert_eq!(TxtMutation::decode_op(&step.encode_op().unwrap()).unwrap(), step);
        assert!(apply_txt_mutation(&mut after, &step).messages().is_empty());
    }
    assert_eq!(TxtSnapshot::from_body(&after.to_body()), after);
    assert_eq!(after, base);
}

#[test]
fn rejects_losing_the_only_visible_crlf_terminator_and_unknown_fields() {
    let base = TxtSnapshot { lines: vec!["a".into()], trailing_newline: true, line_ending: LineEnding::CrLf, ..Default::default() };
    let mutation = SetTrailingNewlineMutation { value: false };
    assert!(!<SetTrailingNewlineMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
    assert!(dsl::json::from_json_str::<SetTrailingNewlineMutation>(r#"{"value":true,"unknown":true}"#).is_err());
}
