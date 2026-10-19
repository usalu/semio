//! ✖️ `remove-map-entry` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use crate::path::SemioValuePath;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveMapEntry {
    pub path: SemioValuePath,
    pub key: String,
}

impl<Op> protocol::MutationKind<SemioValueSnapshot, Op> for RemoveMapEntry
where
    Op: protocol::Mutation<SemioValueSnapshot, Diff = SemioValueTreeDiff> + From<RemoveMapEntry>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "map-entry", kind: "remove-map-entry", record: "RemoveMapEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove map entry", "Zuordnungseintrag entfernen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioValueSnapshot, Op> for RemoveMapEntry
where
    Op: protocol::ColdMutation<SemioValueSnapshot, ReferenceDiff = SemioValueTreeDiff> + From<RemoveMapEntry> + From<crate::set_map_entry::SetMapEntry>,
{
    fn diff(&self, base: &SemioValueSnapshot) -> protocol::MutationOutcome<SemioValueTreeDiff> {
        diff::diff(self, base)
    }

    fn inverse(&self, base: &SemioValueSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
}

//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
