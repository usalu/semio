//! 🚮 `delete-material` — removes an id-keyed material. Does NOT cascade to clear `material_id` references on primitives that pointed at it — `material_id` is a soft `Option<String>` reference with no membership-cascade verb (same category of honest gap brep's own `loop.edges` exclusion documents), and this matches the PRE-EXISTING behaviour of the old `RemoveMaterial` variant exactly, not a new gap introduced by this wave.

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteMaterial {
    pub id: String,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for DeleteMaterial
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<DeleteMaterial> + From<crate::schema::mutations::create_material::CreateMaterial>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "material", kind: "delete-material", record: "DeletedMaterial" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete material \"{}\"", self.id), &format!("Material \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
