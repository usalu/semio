//! 🔚 `set-trailing-newline` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTrailingNewline {
    pub(crate) trailing_newline: bool,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for SetTrailingNewline {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "trailing-newline", kind: "set-trailing-newline", record: "SetTrailingNewline" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as Mutation<TsvSnapshot>>::Diff> {
        let Self { trailing_newline } = self;
        protocol::MutationOutcome::new(TsvDiff { trailing_newline: Some(*trailing_newline), ..TsvDiff::default() })
    }
    fn inverse(&self, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
        Ok(vec![TsvMutation::SetTrailingNewline(set_trailing_newline::SetTrailingNewline { trailing_newline: base.trailing_newline })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set trailing newline", "Abschließenden Zeilenumbruch setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
