//! 🧬️ Direct remove-line mutation owner.
//#region 🔖️Payload
use crate::TxtSnapshot;
use crate::schema::diff::{TxtDiff, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, txt_u32_to_usize};

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveLineMutation {
    pub index: u32,
}

pub type RemoveLinePayload = RemoveLineMutation;

pub fn decode_remove_line_payload(value: &dsl::DslValue) -> Result<RemoveLinePayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["index"])?;
    Ok(RemoveLinePayload { index: crate::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)? })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for RemoveLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "line", kind: "remove-line", record: "RemovedLine" };

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
        let last_empty = if index == base.lines.len() - 1 { base.lines.len().checked_sub(2).and_then(|index| base.lines.get(index)).is_some_and(|line| line.is_empty()) } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len() - 1, last_empty, base.trailing_newline, base.line_ending) {
            return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new());
        }
        protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() })
    }

    fn inverse(&self, base: &TxtSnapshot) -> Vec<super::TxtMutation> {
        let outcome = self.diff(base);
        if !outcome.messages().is_empty() || outcome.diff().lines.is_none() {
            return Vec::new();
        }
        let index = txt_u32_to_usize(self.index).expect("a non-empty diff has a representable line index");
        vec![super::TxtMutation::InsertLine(super::InsertLineMutation { index: self.index, text: base.lines[index].clone() })]
    }

    fn label(&self) -> String {
        "Remove Line".to_string()
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
