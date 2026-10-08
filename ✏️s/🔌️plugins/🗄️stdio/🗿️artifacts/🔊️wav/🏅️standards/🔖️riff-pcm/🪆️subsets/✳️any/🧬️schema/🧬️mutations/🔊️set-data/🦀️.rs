//! 🔊️ `set-data` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetData {
    pub data: WavData,
}

impl protocol::MutationKind<WavSnapshot, WavMutation> for SetData {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "data", kind: "set-data", record: "SetData" };

    fn diff(&self, base: &WavSnapshot) -> protocol::MutationOutcome<<WavMutation as Mutation<WavSnapshot>>::Diff> {
        let Self { data } = self;
        match validate_wav_serialization(&WavSnapshot { data: data.clone(), ..base.clone() }) {
            Ok(()) => protocol::MutationOutcome::new(sparse_against(base, diff_set_data(data.clone()))),
            Err(issue) => protocol::MutationOutcome::error("mutation.target-mismatch", format!("{}: {}", issue.code, issue.message), issue.target),
        }
    }
    fn inverse(&self, base: &WavSnapshot) -> Result<Vec<WavMutation>, semio_framework_value::ValueError> {
        Ok((base.data != self.data).then(|| WavMutation::SetData(set_data::SetData { data: base.data.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set data", "Daten setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
