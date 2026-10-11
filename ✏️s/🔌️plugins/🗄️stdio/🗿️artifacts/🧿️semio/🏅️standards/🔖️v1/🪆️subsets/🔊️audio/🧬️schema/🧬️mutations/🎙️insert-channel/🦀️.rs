//! 🎙️ `insert-channel` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertChannel {
    pub index: usize,
    pub channel: SemioAudioChannel,
}

impl protocol::MutationKind<SemioAudioSnapshot, SemioAudioMutation> for InsertChannel {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "channel", kind: "insert-channel", record: "InsertChannel" };

    fn diff(&self, base: &SemioAudioSnapshot) -> protocol::MutationOutcome<<SemioAudioMutation as Mutation<SemioAudioSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert channel", "Kanal einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
