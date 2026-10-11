//! 🗑️ `remove-stream` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveStream {
    pub index: usize,
}

impl protocol::MutationKind<SemioVideoSnapshot, SemioVideoMutation> for RemoveStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "stream", kind: "remove-stream", record: "RemoveStream" };

    fn diff(&self, base: &SemioVideoSnapshot) -> protocol::MutationOutcome<<SemioVideoMutation as Mutation<SemioVideoSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove stream", "Datenstrom entfernen")
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
