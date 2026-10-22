//! 🔗 `set-primitive-material` — sets (or, given `None`, clears) a primitive's `material_id` reference — real address, one field.

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPrimitiveMaterial {
    pub mesh_id: String,
    pub primitive_id: String,
    pub material_id: Option<String>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for SetPrimitiveMaterial
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<SetPrimitiveMaterial>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "primitive", kind: "set-primitive-material", record: "SetPrimitiveMaterial" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set primitive \"{}\" material in mesh \"{}\"", self.primitive_id, self.mesh_id), &format!("Material von Primitiv \"{}\" in Netz \"{}\" setzen", self.primitive_id, self.mesh_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioMeshSnapshot, Op> for SetPrimitiveMaterial
where
    Op: protocol::ColdMutation<SemioMeshSnapshot, ReferenceDiff = crate::schema::diff::SemioMeshDiff> + From<SetPrimitiveMaterial>,
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
