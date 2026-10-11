//! 📋️ `set-stream-meta` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetStreamMeta {
    pub index: usize,
    pub kind: SemioVideoStreamKind,
    pub codec: String,
    pub width: u32,
    pub height: u32,
    pub rate: SemioRational,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for SetStreamMeta {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "stream-meta", kind: "set-stream-meta", record: "SetStreamMeta" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as Mutation<SemioVideoSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set stream meta", "Datenstrom-Metadaten setzen")
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
