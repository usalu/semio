//! 🧬️ Direct set-trailing-newline mutation owner.
//#region 🔖️Payload
use crate::TxtSnapshot;
use crate::schema::diff::TxtDiff;
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error};

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
    let fields = crate::schema::mutation_support::txt_required_object(value, &["value"])?;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
