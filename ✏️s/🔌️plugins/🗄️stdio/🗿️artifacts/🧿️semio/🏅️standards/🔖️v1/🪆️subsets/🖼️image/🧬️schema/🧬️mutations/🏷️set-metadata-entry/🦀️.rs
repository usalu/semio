//! 🏷️️ `set-metadata-entry` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioImageDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_metadata_entry;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetMetadataEntry {
    pub key: String,
    pub value: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioImageSnapshot, Op> for SetMetadataEntry
where
    Op: protocol::Mutation<SemioImageSnapshot, Diff = crate::schema::diff::SemioImageDiff> + From<SetMetadataEntry> + From<crate::schema::mutations::remove_metadata_entry::RemoveMetadataEntry>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "metadata-entry", kind: "set-metadata-entry", record: "SetMetadataEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set metadata entry", "Metadateneintrag setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioImageSnapshot, Op> for SetMetadataEntry
where
    Op: protocol::ColdMutation<SemioImageSnapshot, ReferenceDiff = crate::schema::diff::SemioImageDiff> + From<SetMetadataEntry> + From<crate::schema::mutations::remove_metadata_entry::RemoveMetadataEntry>,
{
    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioImageSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
