//! 🕸️ `apply-mesh` — authored as its own mutation leaf. Routes this envelope's own dispatch to
//! `stdio.semio`'s Mesh subset: `diff`/`inverse` delegate straight through to
//! `SemioMeshMutation`'s own already-real `Mutation` impl (via its `🔺️diff` and `↩️inverse`), never re-deriving that subset's own
//! per-field logic — the envelope routes, it does not redefine.

#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use crate::schema::diff::SemioDiff;
#[allow(unused_imports)]
use semio_s_artifact_stdio_semio_mesh::schema::mutations::SemioMeshMutation;
#[allow(unused_imports)]
use crate::schema::snapshot::SemioSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyMesh {
    pub mutation: SemioMeshMutation,
}

impl<Op> protocol::MutationKind<SemioSnapshot, Op> for ApplyMesh
where
    Op: protocol::Mutation<SemioSnapshot, Diff = crate::schema::diff::SemioDiff> + From<ApplyMesh>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "mesh", kind: "apply-mesh", record: "ApplyMesh" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        protocol::SemanticMutation::label(&self.mutation)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<SemioSnapshot, Op> for ApplyMesh
where
    Op: protocol::ColdMutation<SemioSnapshot, ReferenceDiff = crate::schema::diff::SemioDiff> + From<ApplyMesh>,
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
