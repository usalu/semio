//! 🧬️ Direct set-line mutation owner.
//#region 🔖️Payload
use crate::schema::diff::{TxtDiff, TxtLineModified, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error, txt_u32_to_usize};
use crate::TxtSnapshot;


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetLineMutation {
    pub index: u32,
    pub text: String,
}

pub type SetLinePayload = SetLineMutation;

pub fn decode_set_line_payload(value: &semio_framework_value::DslValue) -> Result<SetLinePayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["index", "text"])?;
    Ok(SetLinePayload { index: crate::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)?, text: crate::schema::mutation_support::txt_unicode_string(fields[1].1, "text")? })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl SetLineMutation {
    /// 🧭️ The refusal reason, `None` for an unaddressable or unchanged line (nothing to do), or the base line index this edit rewrites.
    fn plan(&self, base: &TxtSnapshot) -> Result<Option<usize>, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        let Ok(index) = txt_u32_to_usize(self.index) else { return Ok(None) };
        if index >= base.lines.len() {
            return Ok(None);
        }
        let is_last = index == base.lines.len() - 1;
        let last_empty = if is_last { self.text.is_empty() } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len(), last_empty, base.trailing_newline, base.line_ending) {
            return Err(reason);
        }
        if let Some(reason) = native_text_error(&self.text, base.line_ending, !is_last || base.trailing_newline) {
            return Err(reason);
        }
        Ok((base.lines[index] != self.text).then_some(index))
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for SetLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line", kind: "set-line", record: "SetLine" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(None) => protocol::MutationOutcome::new(TxtDiff::default()),
            Ok(Some(index)) => protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index, text: self.text.clone() }], added: vec![] }), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(match self.plan(base) {
            Ok(Some(index)) => vec![super::TxtMutation::SetLine(Self { index: self.index, text: base.lines[index].clone() })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Line", "Zeile setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-line".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
