//! 🗂️ `apply-table` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Table subset: `diff`/`inverse` delegate straight through to
//! `SemioTableMutation`'s own already-real `Mutation` impl (via its `🔺️diff` and `↩️inverse`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_table_mutations::schema::mutations::SemioTableMutation;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyTable {
    pub mutation: SemioTableMutation,
}

impl<Op> protocol::MutationKind<SemioSnapshot, Op> for ApplyTable
where
    Op: protocol::Mutation<SemioSnapshot, Diff = crate::schema::diff::SemioDiff> + From<ApplyTable>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "table", kind: "apply-table", record: "ApplyTable" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        protocol::SemanticMutation::label(&self.mutation)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioSnapshot, Op> for ApplyTable
where
    Op: protocol::ColdMutation<SemioSnapshot, ReferenceDiff = crate::schema::diff::SemioDiff> + From<ApplyTable>,
{
    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioSnapshot>>::ReferenceDiff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
