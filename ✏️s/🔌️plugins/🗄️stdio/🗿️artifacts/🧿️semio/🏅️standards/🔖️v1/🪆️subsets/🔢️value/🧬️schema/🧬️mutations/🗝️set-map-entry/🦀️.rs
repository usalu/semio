//! 🗝️ `set-map-entry` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use crate::path::SemioValuePath;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMapEntry {
    pub path: SemioValuePath,
    pub key: String,
    pub value: SemioValue,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioValueSnapshot, Op> for SetMapEntry
where
    Op: protocol::Mutation<SemioValueSnapshot, Diff = SemioValueTreeDiff> + From<SetMapEntry>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "map-entry", kind: "set-map-entry", record: "SetMapEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set map entry", "Zuordnungseintrag setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioValueSnapshot, Op> for SetMapEntry
where
    Op: protocol::ColdMutation<SemioValueSnapshot, ReferenceDiff = SemioValueTreeDiff> + From<SetMapEntry> + From<crate::remove_map_entry::RemoveMapEntry>,
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
