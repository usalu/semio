//! ➕️ `insert-list-item` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.

use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use crate::path::SemioValuePath;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertListItem {
    pub path: SemioValuePath,
    pub index: usize,
    pub value: SemioValue,
}

impl<Op> protocol::MutationKind<SemioValueSnapshot, Op> for InsertListItem
where
    Op: protocol::Mutation<SemioValueSnapshot, Diff = SemioValueTreeDiff> + From<InsertListItem>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "list-item", kind: "insert-list-item", record: "InsertListItem" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert list item", "Listenelement einfügen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioValueSnapshot, Op> for InsertListItem
where
    Op: protocol::ColdMutation<SemioValueSnapshot, ReferenceDiff = SemioValueTreeDiff> + From<InsertListItem> + From<crate::remove_list_item::RemoveListItem>,
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
