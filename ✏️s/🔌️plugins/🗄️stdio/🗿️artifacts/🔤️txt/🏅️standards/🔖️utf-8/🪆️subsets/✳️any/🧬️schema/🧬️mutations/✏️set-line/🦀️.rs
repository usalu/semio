//! 🧬️ Direct set-line mutation owner.
//#region 🔖️Payload
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::txt::schema::diff::{TxtDiff, TxtLineModified, TxtLinesDiff};
use crate::artifacts::txt::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error, txt_u32_to_usize};

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetLineMutation {
    pub index: u32,
    pub text: String,
}

pub type SetLinePayload = SetLineMutation;

pub fn decode_set_line_payload(value: &dsl::DslValue) -> Result<SetLinePayload, String> {
    let fields = crate::artifacts::txt::schema::mutation_support::txt_required_object(value, &["index", "text"])?;
    Ok(SetLinePayload {
        index: crate::artifacts::txt::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)?,
        text: crate::artifacts::txt::schema::mutation_support::txt_unicode_string(fields[1].1, "text")?,
    })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line", kind: "set-line", record: "SetLine" };

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
        let is_last = index == base.lines.len() - 1;
        let last_empty = if is_last { self.text.is_empty() } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len(), last_empty, base.trailing_newline, base.line_ending) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        if let Some(reason) = native_text_error(&self.text, base.line_ending, !is_last || base.trailing_newline) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        protocol::MutationOutcome::new(if base.lines.get(index).map_or(true, |current| current == &self.text) {
            TxtDiff::default()
        } else {
            TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index, text: self.text.clone() }], added: vec![] }), ..Default::default() }
        })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        let outcome = self.diff(base);
        if !outcome.messages().is_empty() || outcome.diff().lines.is_none() {
            return Vec::new();
        }
        let index = txt_u32_to_usize(self.index).expect("a non-empty diff has a representable line index");
        vec![super::TxtMutation::SetLine(Self { index: self.index, text: base.lines[index].clone() })]
    }

    fn label(&self) -> String {
        "Set Line".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["set-line".to_string()]
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
        let expected: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).expect("valid canonical set-line descriptor");
        assert_eq!(serde_json::to_value(<SetLineMutation as MutationLeaf>::DESCRIPTOR).expect("serializable descriptor"), expected);
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
        assert_eq!(<SetLineMutation as protocol::MutationKind<TxtSnapshot, super::super::TxtMutation>>::SEMANTICS.kind, "set-line");
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
        assert!(serde_json::from_str::<SetLineMutation>(r#"{"index":0,"text":"x","unknown":true}"#).is_err());
        assert!(TxtMutation::decode_op(&[vec![5], vec![255]].concat()).is_err());
        let value = serde_json::to_value(TxtMutation::SetLine(SetLineMutation { index: 1, text: "a".into() })).unwrap();
        assert_eq!(value["mutation"], "set-line");
        assert_eq!(value["payload"], serde_json::json!({ "index": 1, "text": "a" }));
    }
}
//#endregion 🧪️Tests
