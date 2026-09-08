//! 🧬️ Direct set-line-ending mutation owner.
//#region 🔖️Payload
use crate::TxtSnapshot;
use crate::schema::diff::TxtDiff;
use crate::schema::mutation_support::{native_lines_error, native_snapshot_error};
use crate::schema::snapshot::LineEnding;

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
    let fields = crate::schema::mutation_support::txt_required_object(value, &["value"])?;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
