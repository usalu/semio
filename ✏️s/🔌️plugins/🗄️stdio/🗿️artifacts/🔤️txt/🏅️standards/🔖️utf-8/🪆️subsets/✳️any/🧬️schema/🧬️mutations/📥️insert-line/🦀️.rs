//! 🧬️ Direct insert-line mutation owner.
//#region 🔖️Payload
use crate::schema::diff::{TxtDiff, TxtLineAdded, TxtLinesDiff};
use crate::schema::mutation_support::{native_shape_error, native_snapshot_error, native_text_error, txt_u32_to_usize, txt_usize_to_u32};
use crate::TxtSnapshot;


#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertLineMutation {
    pub index: u32,
    pub text: String,
}

pub type InsertLinePayload = InsertLineMutation;

pub fn decode_insert_line_payload(value: &semio_framework_value::DslValue) -> Result<InsertLinePayload, String> {
    let fields = crate::schema::mutation_support::txt_required_object(value, &["index", "text"])?;
    Ok(InsertLinePayload { index: crate::schema::mutation_support::txt_graphql_u32_variable(fields[0].1)?, text: crate::schema::mutation_support::txt_unicode_string(fields[1].1, "text")? })
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl InsertLineMutation {
    /// 🧭️ The refusal reason, or the line index this insert lands at (clamped to the end).
    fn plan(&self, base: &TxtSnapshot) -> Result<usize, String> {
        if let Some(reason) = native_snapshot_error(base) {
            return Err(reason);
        }
        let at = txt_u32_to_usize(self.index).unwrap_or(base.lines.len()).min(base.lines.len());
        let last_empty = if at == base.lines.len() { self.text.is_empty() } else { base.lines.last().is_some_and(|line| line.is_empty()) };
        if let Some(reason) = native_shape_error(base.lines.len() + 1, last_empty, base.trailing_newline, base.line_ending) {
            return Err(reason);
        }
        if let Some(reason) = native_text_error(&self.text, base.line_ending, at < base.lines.len() || base.trailing_newline) {
            return Err(reason);
        }
        if at == base.lines.len() {
            if let Some(reason) = base.lines.last().and_then(|line| native_text_error(line, base.line_ending, true)) {
                return Err(reason);
            }
        }
        Ok(at)
    }
}

impl protocol::MutationKind<TxtSnapshot, super::TxtMutation> for InsertLineMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "line", kind: "insert-line", record: "InsertedLine" };

    fn diff(&self, base: &TxtSnapshot) -> protocol::MutationOutcome<TxtDiff> {
        match self.plan(base) {
            Err(reason) => protocol::MutationOutcome::fatal("mutation.invariant", reason, Vec::<String>::new()),
            Ok(at) => protocol::MutationOutcome::new(TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: at, text: self.text.clone() }] }), ..Default::default() }),
        }
    }

    fn inverse(&self, base: &TxtSnapshot) -> Result<Vec<super::TxtMutation>, semio_framework_value::ValueError> {
        Ok(match self.plan(base) {
            Ok(at) => vec![super::TxtMutation::RemoveLine(super::RemoveLineMutation { index: txt_usize_to_u32(at).expect("line length is within the public uint32 domain after a successful insert") })],
            Err(_) => Vec::new(),
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert Line", "Zeile einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-line".to_string()]
    }
}
//#endregion ⚙️Semantics

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
