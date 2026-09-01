//! 🧬️ Direct set-line-ending mutation owner.
//#region 🔖️Payload
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::TxtDiff;
use crate::artifacts::txt::schema::mutation_support::{native_lines_error, native_snapshot_error};
use crate::artifacts::txt::schema::snapshot::LineEnding;

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetLineEndingMutation {
    pub value: LineEnding,
}

pub type SetLineEndingPayload = SetLineEndingMutation;

pub fn decode_set_line_ending_payload(value: &dsl::DslValue) -> Result<SetLineEndingPayload, String> {
    let fields = crate::artifacts::txt::schema::mutation_support::txt_required_object(value, &["value"])?;
    let value = match fields[0].1.as_str() {
        Some("lf") => LineEnding::Lf,
        Some("crLf") => LineEnding::CrLf,
        _ => return Err("payload field `value` must be `lf` or `crLf`".to_string()),
    };
    Ok(SetLineEndingPayload { value })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetLineEndingMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line-ending", kind: "set-line-ending", record: "SetLineEnding" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        if let Some(reason) = native_snapshot_error(base) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if let Some(reason) = native_lines_error(&base.lines, base.trailing_newline, self.value) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        protocol::MutationOutcome::new(if base.line_ending == self.value { TxtDiff::default() } else { TxtDiff { line_ending: Some(self.value), ..Default::default() } })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        if self.diff(base).diff().line_ending.is_none() {
            return Vec::new();
        }
        vec![super::TxtMutation::SetLineEnding(Self { value: base.line_ending })]
    }

    fn label(&self) -> String {
        "Set Line Ending".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["set-line-ending".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::{TxtMutation, apply_txt_mutation};
    use super::*;
    use protocol::{Mutation, MutationKind, MutationLeaf, OpBinary, OpText};

    #[test]
    fn canonical_leaf_metadata_matches_descriptor_and_provenance() {
        let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("valid canonical set-line-ending descriptor");
        assert_eq!(serde_json::to_value(<SetLineEndingMutation as MutationLeaf>::DESCRIPTOR).expect("serializable descriptor"), expected);
        let provenance = <SetLineEndingMutation as MutationLeaf>::PROVENANCE;
        assert_eq!(provenance.mutation_root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
        assert_eq!(provenance.owner, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending");
        assert_eq!(provenance.source_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🦀️.rs");
        assert_eq!(provenance.descriptor_path, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔚️set-line-ending/🔣️.json");
        assert_eq!(provenance.taxonomy_path, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
        assert!(provenance.workspace_token.iter().any(|byte| *byte != 0));
    }

    #[test]
    fn semantic_identity_matches_descriptor() {
        assert_eq!(<SetLineEndingMutation as protocol::MutationKind<TxtSnapshot, super::super::TxtMutation>>::SEMANTICS.kind, "set-line-ending");
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
        assert!(serde_json::from_str::<SetLineEndingMutation>(r#"{"value":"lf","unknown":true}"#).is_err());
    }
}
//#endregion 🧪️Tests
