//! 🔀 `set-line-ending` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLineEnding {
    pub(crate) line_ending: LineEnding,
}

impl protocol::MutationKind<TsvSnapshot, TsvMutation> for SetLineEnding {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "line-ending", kind: "set-line-ending", record: "SetLineEnding" };

    fn diff(&self, base: &TsvSnapshot) -> protocol::MutationOutcome<<TsvMutation as Mutation<TsvSnapshot>>::Diff> {
        let Self { line_ending } = self;
        protocol::MutationOutcome::new(TsvDiff { line_ending: Some(*line_ending), ..TsvDiff::default() })
    }
    fn inverse(&self, base: &TsvSnapshot) -> Result<Vec<TsvMutation>, semio_framework_value::ValueError> {
        Ok(vec![TsvMutation::SetLineEnding(set_line_ending::SetLineEnding { line_ending: base.line_ending })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set line ending", "Zeilenende setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
