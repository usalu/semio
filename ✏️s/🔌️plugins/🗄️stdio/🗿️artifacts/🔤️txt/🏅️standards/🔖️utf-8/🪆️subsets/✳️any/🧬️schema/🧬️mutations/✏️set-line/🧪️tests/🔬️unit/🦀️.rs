use super::super::{apply_txt_mutation, TxtMutation};
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical set-line descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<SetLineMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <SetLineMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️set-line/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SetLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "set-line");
}

#[test]
fn crlf_bare_lf_content_inverse_and_root_codecs_restore_the_native_snapshot() {
    let base = TxtSnapshot { lines: vec!["a\nb".into(), "c".into()], line_ending: LineEnding::CrLf, ..Default::default() };
    let mutation = TxtMutation::SetLine(SetLineMutation { index: 0, text: "x\ny".into() });
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
fn rejects_lf_separator_hazards_and_unknown_fields() {
    let base = TxtSnapshot { lines: vec!["a".into(), "b".into()], line_ending: LineEnding::Lf, ..Default::default() };
    let mutation = SetLineMutation { index: 0, text: "a\r".into() };
    assert!(!<SetLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
    assert!(dsl::json::from_json_str::<SetLineMutation>(r#"{"index":0,"text":"x","unknown":true}"#).is_err());
    assert!(TxtMutation::decode_op(&[vec![5], vec![255]].concat()).is_err());
    let value = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(TxtMutation::SetLine(SetLineMutation { index: 1, text: "a".into() })))).unwrap();
    assert_eq!(value["mutation"], "set-line");
    assert_eq!(value["payload"], serde_json::json!({ "index": 1, "text": "a" }));
}
