
use super::super::{RemoveLineMutation, TxtMutation, apply_txt_mutation};
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical insert-line descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<InsertLineMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <InsertLineMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📥️insert-line/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<InsertLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "insert-line");
}

#[test]
fn clamped_insert_inverse_and_root_codecs_restore_the_native_snapshot() {
    let base = TxtSnapshot { lines: vec!["a".into()], line_ending: LineEnding::Lf, ..Default::default() };
    let mutation = TxtMutation::InsertLine(InsertLineMutation { index: 99, text: "b".into() });
    let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse, vec![TxtMutation::RemoveLine(RemoveLineMutation { index: 1 })]);
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
fn rejects_contextually_invalid_text_and_unknown_fields() {
    let base = TxtSnapshot { lines: vec!["a".into()], ..Default::default() };
    let mutation = InsertLineMutation { index: 1, text: "b\nc".into() };
    assert!(!<InsertLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
    assert!(dsl::json::from_json_str::<InsertLineMutation>(r#"{"index":0,"text":"x","unknown":true}"#).is_err());
    assert!(TxtMutation::parse_op("txt-mutation insert-line payload=7b22696e646578223a302c2274657874223a2278222c22756e6b6e6f776e223a747275657d").is_err());
    assert!(TxtMutation::decode_op(&[vec![3], br#"{"index":0,"text":"x","unknown":true}"#.to_vec()].concat()).is_err());
}
