//! 🧱 `change-material-roughness` — sets a material's PBR roughness factor. See `change-material-metallic`'s sibling doc comment; the same decompose reasoning applies symmetrically.

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeMaterialRoughness {
    pub id: String,
    pub new_roughness: f32,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for ChangeMaterialRoughness
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<ChangeMaterialRoughness>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "material-roughness", kind: "change-material-roughness", record: "ChangedMaterialRoughness" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change material \"{}\" roughness factor", self.id), &format!("Rauigkeitsfaktor von Material \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioMeshSnapshot, Op> for ChangeMaterialRoughness
where
    Op: protocol::ColdMutation<SemioMeshSnapshot, ReferenceDiff = crate::schema::diff::SemioMeshDiff> + From<ChangeMaterialRoughness>,
{
    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioMeshSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
