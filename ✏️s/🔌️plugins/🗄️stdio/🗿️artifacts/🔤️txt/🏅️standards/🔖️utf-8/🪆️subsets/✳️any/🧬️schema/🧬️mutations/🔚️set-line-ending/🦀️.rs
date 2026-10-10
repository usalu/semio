//! 🧬️ Direct set-line-ending mutation owner.
//#region 🔖️Payload
use crate::schema::diff::TxtDiff;
use crate::schema::mutation_support::{native_lines_error, native_snapshot_error};
use crate::schema::snapshot::LineEnding;
use crate::TxtSnapshot;


#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetLineEndingMutation {
    pub value: LineEnding,
}

pub type SetLineEndingPayload = SetLineEndingMutation;

pub fn decode_set_line_ending_payload(value: &semio_framework_value::DslValue) -> Result<SetLineEndingPayload, String> {
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
impl SetLineEndingMutation {
    /// 🧭️ The refusal reason, or whether the line ending actually changes.
    fn plan(&self, base: &TxtSnapshot) -> Result<bool, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        if let Some(reason) = native_lines_error(&base.lines, base.trailing_newline, self.value) {
            return Err(reason);
        }
        Ok(base.line_ending != self.value)
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetLineEndingMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line-ending", kind: "set-line-ending", record: "SetLineEnding" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(false) => protocol::MutationOutcome::new(TxtDiff::default()),
            Ok(true) => protocol::MutationOutcome::new(TxtDiff { line_ending: Some(self.value), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(if self.plan(base) == Ok(true) { vec![super::TxtMutation::SetLineEnding(Self { value: base.line_ending })] } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Line Ending", "Zeilenende setzen")
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
