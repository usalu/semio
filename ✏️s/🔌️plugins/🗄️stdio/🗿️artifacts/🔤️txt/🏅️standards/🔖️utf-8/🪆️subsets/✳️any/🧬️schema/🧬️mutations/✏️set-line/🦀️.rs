//! 🧬️ Direct set-line mutation owner.
//#region 🔖️Payload
use crate::schema::diff::{TxtDiff, TxtLineModified, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error, txt_u32_to_usize};
use crate::TxtSnapshot;

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
    let fields = crate::schema::mutation_support::txt_required_object(value, &["index", "text"])?;
    Ok(SetLinePayload { index: crate::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)?, text: crate::schema::mutation_support::txt_unicode_string(fields[1].1, "text")? })
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
        protocol::MutationOutcome::new(if base.lines.get(index).is_none_or(|current| current == &self.text) {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
