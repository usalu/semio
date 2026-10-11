//! 🎚️ `set-layer` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLayer {
    pub name: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub line_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl protocol::MutationKind<SemioCadSnapshot, SemioCadMutation> for SetLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer", record: "SetLayer" };

    fn diff(&self, base: &SemioCadSnapshot) -> protocol::MutationOutcome<<SemioCadMutation as Mutation<SemioCadSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set layer", "Ebene setzen")
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
