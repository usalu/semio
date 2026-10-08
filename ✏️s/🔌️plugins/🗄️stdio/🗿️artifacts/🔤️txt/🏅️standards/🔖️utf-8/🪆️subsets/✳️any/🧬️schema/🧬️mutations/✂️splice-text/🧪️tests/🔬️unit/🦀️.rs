use crate::apply_mutation;
use super::super::{TxtMutation};
use super::*;
use crate::schema::snapshot::LineEnding;
use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

#[test]
fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
    let expected: serde_json::Value = serde_json::from_str(include_str!("../../🔣️.json")).expect("valid canonical splice-text descriptor");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(<SpliceTextMutation as MutationLeaf>::DESCRIPTOR))).expect("serializable descriptor"), expected);
    let provenance = <SpliceTextMutation as MutationLeaf>::PROVENANCE;
    assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
    assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️splice-text");
    assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️splice-text/🦀️.rs");
    assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️splice-text/🔣️.json");
    assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🔣️mutation-authority.json");
    assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
}

#[test]
fn semantic_identity_matches_descriptor() {
    assert_eq!(<SpliceTextMutation as MutationKind<TxtSnapshot, TxtMutation>>::SEMANTICS.kind, "splice-text");
}

#[test]
fn a_range_set_rewrites_only_the_lines_it_touches_and_its_inverse_restores_the_text() {
    let base = TxtSnapshot::from_body("alpha\nbeta\ngamma\n");
    let mutation = TxtMutation::SpliceText(SpliceTextMutation { splices: vec![TextSplice { offset: 6, delete: 4, insert: "BETA".into() }, TextSplice { offset: 17, delete: 0, insert: "delta".into() }] });
    let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture");
    let outcome = <TxtMutation as Mutation<TxtSnapshot>>::diff(&mutation, &base);
    assert!(outcome.messages().is_empty());
    let diff = outcome.diff();
    assert_eq!(diff.trailing_newline, Some(false));
    let lines = diff.lines.as_ref().expect("sparse line rows");
    assert_eq!((lines.modified.len(), lines.added.len(), lines.removed.len()), (1, 1, 0));
    let mut after = base.clone();
    assert!(apply_mutation(&mut after, &mutation).messages().is_empty());
    assert_eq!(after.to_body(), "alpha\nBETA\ngamma\ndelta");
    for step in inverse.iter().rev() {
        assert_eq!(TxtMutation::parse_op(&step.print_op()).unwrap(), *step);
        assert_eq!(TxtMutation::decode_op(&step.encode_op().unwrap()).unwrap(), *step);
        assert!(apply_mutation(&mut after, step).messages().is_empty());
    }
    assert_eq!(after, base);
}

#[test]
fn a_change_that_touches_nothing_answers_an_empty_diff_and_no_inverse() {
    let base = TxtSnapshot::from_body("a\nb\n");
    let mutation = SpliceTextMutation { splices: vec![TextSplice { offset: 0, delete: 1, insert: "a".into() }] };
    assert_eq!(<SpliceTextMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).diff(), &crate::schema::diff::TxtDiff::default());
    assert!(<TxtMutation as Mutation<TxtSnapshot>>::inverse(&TxtMutation::SpliceText(mutation), &base).unwrap().is_empty());
}

#[test]
fn refuses_ranges_that_leave_the_text_overlap_or_leave_the_native_shape() {
    let base = TxtSnapshot { lines: vec!["a".into(), "b".into()], trailing_newline: true, line_ending: LineEnding::Lf, ..Default::default() };
    for splices in [
        vec![TextSplice { offset: 9, delete: 0, insert: "x".into() }],
        vec![TextSplice { offset: 0, delete: 2, insert: "x".into() }, TextSplice { offset: 1, delete: 0, insert: "y".into() }],
        vec![TextSplice { offset: 1, delete: 0, insert: "\r".into() }],
    ] {
        let mutation = SpliceTextMutation { splices };
        assert!(!<SpliceTextMutation as MutationKind<TxtSnapshot, TxtMutation>>::diff(&mutation, &base).messages().is_empty());
        assert!(<TxtMutation as Mutation<TxtSnapshot>>::inverse(&TxtMutation::SpliceText(mutation), &base).unwrap().is_empty());
    }
    assert!(semio_framework_pack_json::from_json_str::<SpliceTextMutation>(r#"{"splices":[],"unknown":true}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<SpliceTextMutation>(r#"{"splices":[{"offset":0,"delete":0,"insert":"x","unknown":1}]}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}

/// 🔬️ The shared corpus (`🧫️fixtures/✂️splice-text`, oracle `🧪️tests/✂️splice-text/🟦️.ts`): every row means the corpus text, agrees with the
/// contract's own scalar splice application, and its inverse restores the base.
#[test]
fn the_corpus_rows_mean_the_corpus_text() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/✂️splice-text/🔣️.json")).expect("splice corpus");
    for row in corpus["cases"].as_array().expect("cases") {
        let id = row["id"].as_str().unwrap();
        let before = row["before"].as_str().unwrap();
        let splices: Vec<TextSplice> = row["splices"].as_array().unwrap().iter().map(|splice| TextSplice { offset: splice["offset"].as_u64().unwrap() as u32, delete: splice["delete"].as_u64().unwrap() as u32, insert: splice["insert"].as_str().unwrap().to_string() }).collect();
        let base = TxtSnapshot::from_body(before);
        let mutation = TxtMutation::SpliceText(SpliceTextMutation { splices: splices.clone() });
        let outcome = <TxtMutation as Mutation<TxtSnapshot>>::diff(&mutation, &base);
        match row["after"].as_str() {
            None => assert!(!outcome.messages().is_empty(), "{id}"),
            Some(after) => {
                let contract = semio_s_artifact_stdio_contract::apply_draft_splices(before, &splices.iter().map(|splice| semio_s_artifact_stdio_contract::DraftSplice { offset: splice.offset as usize, delete: splice.delete as usize, insert: splice.insert.clone() }).collect::<Vec<_>>());
                assert_eq!(contract.as_deref(), Some(after), "{id}: the contract applies the ranges to the same text");
                let mut state = base.clone();
                assert!(apply_mutation(&mut state, &mutation).messages().is_empty(), "{id}");
                assert_eq!(state.to_body(), after, "{id}");
                let inverse = <TxtMutation as Mutation<TxtSnapshot>>::inverse(&mutation, &base).unwrap();
                for step in inverse.iter().rev() {
                    assert!(apply_mutation(&mut state, step).messages().is_empty(), "{id}");
                }
                assert_eq!(state.to_body(), before, "{id}");
            }
        }
    }
}

/// ⚖️ `mutation_inverse_sum_law`: the inverse diffs sum to the negative forward diff for ranges that join, split, append and drop lines.
#[semio_framework_async_macros::async_test]
async fn splice_text_inverse_sum_law_holds_for_every_shape() {
    let base = TxtSnapshot::from_body("alpha\nbeta\ngamma\n");
    for splices in [
        vec![TextSplice { offset: 6, delete: 4, insert: "BETA".into() }],
        vec![TextSplice { offset: 8, delete: 0, insert: "\n".into() }],
        vec![TextSplice { offset: 5, delete: 1, insert: String::new() }],
        vec![TextSplice { offset: 17, delete: 0, insert: "delta".into() }],
        vec![TextSplice { offset: 16, delete: 1, insert: String::new() }],
        vec![TextSplice { offset: 0, delete: 1, insert: "A".into() }, TextSplice { offset: 11, delete: 5, insert: "G".into() }],
        vec![TextSplice { offset: 0, delete: 17, insert: String::new() }],
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&TxtMutation::SpliceText(SpliceTextMutation { splices }), &base).await;
    }
}
