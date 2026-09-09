use super::super::{apply_txt_mutation, InsertLineMutation, TxtMutation};
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

fn snapshot(lines: &[&str], trailing_newline: bool, line_ending: LineEnding) -> TxtSnapshot {
    TxtSnapshot { lines: lines.iter().map(|line| (*line).to_string()).collect(), trailing_newline, line_ending, ..Default::default() }
}

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical remove-line descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&(<RemoveLineMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <RemoveLineMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<RemoveLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "remove-line");
}

#[test]
fn one_line_to_empty_round_trips_through_production_inverse_and_codecs() {
    let base = snapshot(&["a"], false, LineEnding::Lf);
    let mutation = TxtMutation::RemoveLine(RemoveLineMutation { index: 0 });
    let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base);
    assert_eq!(inverse, vec![TxtMutation::InsertLine(InsertLineMutation { index: 0, text: "a".into() })]);
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
fn removing_the_last_visible_crlf_separator_is_refused() {
    let base = snapshot(&["a", "b"], false, LineEnding::CrLf);
    let mutation = RemoveLineMutation { index: 0 };
    assert!(<RemoveLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::inverse(&mutation, &base).is_empty());
    assert!(!<RemoveLineMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
    assert!(dsl::json::from_json_str::<RemoveLineMutation>(r#"{"index":0,"unknown":true}"#).is_err());
    assert!(dsl::json::from_json_str::<TxtMutation>(r#"{"mutation":"remove-line","payload":{"index":0},"unknown":true}"#).is_err());
}
