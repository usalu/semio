//! 🧬️ Direct set-trailing-newline mutation owner.
//#region 🔖️Payload
use crate::schema::diff::TxtDiff;
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error};
use crate::TxtSnapshot;


#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetTrailingNewlineMutation {
    pub value: bool,
}

pub type SetTrailingNewlinePayload = SetTrailingNewlineMutation;

pub fn decode_set_trailing_newline_payload(value: &semio_framework_value::DslValue) -> Result<SetTrailingNewlinePayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["value"])?;
    let value = fields[0].1.as_bool().ok_or_else(|| "payload field `value` must be boolean".to_string())?;
    Ok(SetTrailingNewlinePayload { value })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl SetTrailingNewlineMutation {
    /// 🧭️ The refusal reason, or whether the trailing newline actually changes.
    fn plan(&self, base: &TxtSnapshot) -> Result<bool, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        if let Some(reason) = native_shape_error(base.lines.len(), base.lines.last().is_some_and(|line| line.is_empty()), self.value, base.line_ending) {
            return Err(reason);
        }
        if let Some(reason) = base.lines.last().and_then(|line| native_text_error(line, base.line_ending, self.value)) {
            return Err(reason);
        }
        Ok(base.trailing_newline != self.value)
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetTrailingNewlineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "trailing-newline", kind: "set-trailing-newline", record: "SetTrailingNewline" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(false) => protocol::MutationOutcome::new(TxtDiff::default()),
            Ok(true) => protocol::MutationOutcome::new(TxtDiff { trailing_newline: Some(self.value), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(if self.plan(base) == Ok(true) { vec![super::TxtMutation::SetTrailingNewline(Self { value: base.trailing_newline })] } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Trailing Newline", "Abschließenden Zeilenumbruch setzen")
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
