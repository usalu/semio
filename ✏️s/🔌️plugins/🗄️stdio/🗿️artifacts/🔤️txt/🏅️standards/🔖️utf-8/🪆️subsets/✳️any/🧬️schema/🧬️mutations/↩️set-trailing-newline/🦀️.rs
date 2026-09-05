//! 🧬️ Direct set-trailing-newline mutation owner.
//#region 🔖️Payload
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::TxtDiff;
use crate::artifacts::txt::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error};

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetTrailingNewlineMutation {
    pub value: bool,
}

pub type SetTrailingNewlinePayload = SetTrailingNewlineMutation;

pub fn decode_set_trailing_newline_payload(value: &dsl::DslValue) -> Result<SetTrailingNewlinePayload, String> {
    let fields = crate::artifacts::txt::schema::mutation_support::txt_required_object(value, &["value"])?;
    let value = fields[0].1.as_bool().ok_or_else(|| "payload field `value` must be boolean".to_string())?;
    Ok(SetTrailingNewlinePayload { value })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetTrailingNewlineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "trailing-newline", kind: "set-trailing-newline", record: "SetTrailingNewline" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        if let Some(reason) = native_snapshot_error(base) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if let Some(reason) = native_shape_error(base.lines.len(), base.lines.last().is_some_and(|line| line.is_empty()), self.value, base.line_ending) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if let Some(reason) = base.lines.last().and_then(|line| native_text_error(line, base.line_ending, self.value)) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        protocol::MutationOutcome::new(if base.trailing_newline == self.value { TxtDiff::default() } else { TxtDiff { trailing_newline: Some(self.value), ..Default::default() } })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        let outcome = self.diff(base);
        if !outcome.messages().is_empty() || outcome.diff().trailing_newline.is_none() {
            return Vec::new();
        }
        vec![super::TxtMutation::SetTrailingNewline(Self { value: base.trailing_newline })]
    }

    fn label(&self) -> String {
        "Set Trailing Newline".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["set-trailing-newline".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::{TxtMutation, apply_txt_mutation};
    use super::*;
    use crate::artifacts::txt::schema::snapshot::LineEnding;
    use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};
    #[test]
    fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
        let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
        assert_eq!(serde_json::to_value(<SetTrailingNewlineMutation as MutationLeaf>::DESCRIPTOR).unwrap(), expected);
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
        assert_eq!(<SetTrailingNewlineMutation as protocol::MutationKind<TxtSnapshot, super::super::TxtMutation>>::SEMANTICS.kind, "set-trailing-newline");
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
        assert!(serde_json::from_str::<SetTrailingNewlineMutation>(r#"{"value":true,"unknown":true}"#).is_err());
    }
}
//#endregion 🧪️Tests
