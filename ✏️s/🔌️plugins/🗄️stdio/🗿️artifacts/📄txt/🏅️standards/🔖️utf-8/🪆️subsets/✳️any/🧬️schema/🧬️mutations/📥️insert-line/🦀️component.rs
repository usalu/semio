//! 🧬️ Direct insert-line mutation owner.
//#region 🔖️Payload
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::{TxtDiff, TxtLineAdded, TxtLinesDiff};
use crate::artifacts::txt::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error};
use serde::{Deserialize, Serialize};

#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertLineMutation {
    pub index: usize,
    pub text: String,
}

pub type InsertLinePayload = InsertLineMutation;
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for InsertLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "line", kind: "insert-line", record: "InsertedLine" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        if let Some(reason) = native_snapshot_error(base) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        let at = self.index.min(base.lines.len());
        let last_empty = if at == base.lines.len() { self.text.is_empty() } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len() + 1, last_empty, base.trailing_newline, base.line_ending) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if let Some(reason) = native_text_error(&self.text, base.line_ending, at < base.lines.len() || base.trailing_newline) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if at == base.lines.len() {
            if let Some(reason) = base.lines.last().and_then(|line| native_text_error(line, base.line_ending, true)) {
                return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
            }
        }
        protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: at, text: self.text.clone() }] }), ..Default::default() })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        let outcome = self.diff(base);
        if !outcome.messages().is_empty() {
            return Vec::new();
        }
        vec![super::TxtMutation::RemoveLine(super::RemoveLineMutation { index: self.index.min(base.lines.len()) })]
    }

    fn label(&self) -> String {
        "Insert Line".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-line".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::{RemoveLineMutation, TxtMutation, apply_txt_mutation};
    use super::*;
    use crate::artifacts::txt::schema::snapshot::LineEnding;
    use protocol::{Mutation, MutationKind, OpBinary, OpText};
    #[test]
    fn semantic_identity_matches_descriptor() {
        assert_eq!(<InsertLineMutation as protocol::MutationKind<TxtSnapshot, super::super::TxtMutation>>::SEMANTICS.kind, "insert-line");
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
        assert!(serde_json::from_str::<InsertLineMutation>(r#"{"index":0,"text":"x","unknown":true}"#).is_err());
        assert!(TxtMutation::parse_op("txt-mutation insert-line payload=7b22696e646578223a302c2274657874223a2278222c22756e6b6e6f776e223a747275657d").is_err());
        assert!(TxtMutation::decode_op(&[vec![3], br#"{"index":0,"text":"x","unknown":true}"#.to_vec()].concat()).is_err());
    }
}
//#endregion 🧪️Tests
