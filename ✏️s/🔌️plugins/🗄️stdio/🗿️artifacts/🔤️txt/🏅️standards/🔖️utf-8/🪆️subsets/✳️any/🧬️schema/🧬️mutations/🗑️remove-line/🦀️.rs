//! 🧬️ Direct remove-line mutation owner.
//#region 🔖️Payload
use crate::schema::diff::{TxtDiff, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, txt_u32_to_usize};
use crate::TxtSnapshot;


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveLineMutation {
    pub index: u32,
}

pub type RemoveLinePayload = RemoveLineMutation;

pub fn decode_remove_line_payload(value: &semio_framework_value::DslValue) -> Result<RemoveLinePayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["index"])?;
    Ok(RemoveLinePayload { index: crate::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)? })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl RemoveLineMutation {
    /// 🧭️ The refusal reason, `None` for an unaddressable line (nothing to do), or the base line index this removal drops.
    fn plan(&self, base: &TxtSnapshot) -> Result<Option<usize>, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        let Ok(index) = txt_u32_to_usize(self.index) else { return Ok(None) };
        if index >= base.lines.len() {
            return Ok(None);
        }
        let last_empty = if index == base.lines.len() - 1 { base.lines.len().checked_sub(2).and_then(|index| base.lines.get(index)).is_some_and(|line| line.is_empty()) } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len() - 1, last_empty, base.trailing_newline, base.line_ending) {
            return Err(reason);
        }
        Ok(Some(index))
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for RemoveLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "line", kind: "remove-line", record: "RemovedLine" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(None) => protocol::MutationOutcome::new(TxtDiff::default()),
            Ok(Some(index)) => protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(match self.plan(base) {
            Ok(Some(index)) => vec![super::TxtMutation::InsertLine(super::InsertLineMutation { index: self.index, text: base.lines[index].clone() })],
            _ => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove Line", "Zeile entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-line".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
