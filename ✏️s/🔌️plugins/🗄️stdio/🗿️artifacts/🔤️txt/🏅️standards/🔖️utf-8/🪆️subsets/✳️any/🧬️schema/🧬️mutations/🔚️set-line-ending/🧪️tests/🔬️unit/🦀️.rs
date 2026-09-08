
use super::super::{TxtMutation, apply_txt_mutation};
use super::*;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical set-line-ending descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<SetLineEndingMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <SetLineEndingMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetLineEndingMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "set-line-ending");
}

#[test]
fn inverse_and_root_codecs_restore_a_visible_crlf_style() {
    let base = TxtSnapshot { lines: vec!["a".into(), "b".into()], ..Default::default() };
    let mutation = TxtMutation::SetLineEnding(SetLineEndingMutation { value: LineEnding::CrLf });
    let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base);
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
fn refuses_an_invisible_crlf_choice_and_unknown_fields() {
    let base = TxtSnapshot { lines: vec!["a".into()], ..Default::default() };
    let mutation = SetLineEndingMutation { value: LineEnding::CrLf };
    assert!(!<SetLineEndingMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
    assert!(dsl::json::from_json_str::<SetLineEndingMutation>(r#"{"value":"lf","unknown":true}"#).is_err());
}
