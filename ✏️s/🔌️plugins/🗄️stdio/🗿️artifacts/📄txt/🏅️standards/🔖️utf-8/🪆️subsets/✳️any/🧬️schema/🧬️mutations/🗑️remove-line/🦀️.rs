//! 🧬️ Direct remove-line mutation owner.
//#region 🔖️Payload
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::{TxtDiff, TxtLinesDiff};
use crate::artifacts::txt::schema::mutation_support::{native_shape_error, native_snapshot_error, txt_u32_to_usize};
use serde::{Deserialize, Serialize};

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveLineMutation {
    pub index: u32,
}

pub type RemoveLinePayload = RemoveLineMutation;

pub fn decode_remove_line_payload(value: &dsl::DslValue) -> Result<RemoveLinePayload, String> {
    let fields = crate::artifacts::txt::schema::mutation_support::txt_required_object(value, &["index"])?;
    Ok(RemoveLinePayload { index: crate::artifacts::txt::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)? })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for RemoveLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "line", kind: "remove-line", record: "RemovedLine" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        if let Some(reason) = native_snapshot_error(base) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        let index = match txt_u32_to_usize(self.index) {
            Ok(index) => index,
            Err(_) => return protocol::MutationOutcome::new(TxtDiff::default()),
        };
        if index >= base.lines.len() {
            return protocol::MutationOutcome::new(TxtDiff::default());
        }
        let last_empty = if index == base.lines.len() - 1 { base.lines.len().checked_sub(2).and_then(|index| base.lines.get(index)).is_some_and(|line| line.is_empty()) } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len() - 1, last_empty, base.trailing_newline, base.line_ending) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        let outcome = self.diff(base);
        if !outcome.messages().is_empty() || outcome.diff().lines.is_none() {
            return Vec::new();
        }
        let index = txt_u32_to_usize(self.index).expect("a non-empty diff has a representable line index");
        vec![super::TxtMutation::InsertLine(super::InsertLineMutation { index: self.index, text: base.lines[index].clone() })]
    }

    fn label(&self) -> String {
        "Remove Line".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-line".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::{InsertLineMutation, TxtMutation, apply_txt_mutation};
    use super::*;
    use crate::artifacts::txt::schema::snapshot::LineEnding;
    use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

    fn snapshot(lines: &[&str], trailing_newline: bool, line_ending: LineEnding) -> TxtSnapshot {
        TxtSnapshot { lines: lines.iter().map(|line| (*line).to_string()).collect(), trailing_newline, line_ending, ..Default::default() }
    }

    #[test]
    fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
        let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("valid canonical remove-line descriptor");
        assert_eq!(serde_json::to_value(<RemoveLineMutation as MutationLeaf>::DESCRIPTOR).expect("serializable descriptor"), expected);
        let provenance = <RemoveLineMutation as MutationLeaf>::PROVENANCE;
        assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line");
        assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🦀️.rs");
        assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-line/🔣️.json");
        assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
        assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
    }

    #[test]
    fn semantic_identity_matches_descriptor() {
        assert_eq!(<RemoveLineMutation as protocol::MutationKind<TxtSnapshot, super::super::TxtMutation>>::SEMANTICS.kind, "remove-line");
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
        assert!(serde_json::from_str::<RemoveLineMutation>(r#"{"index":0,"unknown":true}"#).is_err());
        assert!(serde_json::from_str::<TxtMutation>(r#"{"mutation":"remove-line","payload":{"index":0},"unknown":true}"#).is_err());
    }
}
//#endregion 🧪️Tests
