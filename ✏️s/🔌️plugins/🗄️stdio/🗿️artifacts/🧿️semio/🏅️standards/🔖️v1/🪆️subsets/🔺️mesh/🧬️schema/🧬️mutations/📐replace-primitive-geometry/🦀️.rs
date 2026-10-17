//! 📐 `replace-primitive-geometry` — whole-value swap of a primitive's positions/normals/uvs/colors/indices buffer set. SMO-approved rename of the old `set-primitive-geometry`: a multi-array vertex-buffer blob is a structured sub-payload, so `set` was the wrong verb. SMO approved the reasoning and reserved the edit; SMO wound down without doing it; DKM completes it here.

use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioUv;
use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePrimitiveGeometry {
    pub mesh_id: String,
    pub primitive_id: String,
    pub positions: Vec<SemioPoint3>,
    pub normals: Vec<SemioPoint3>,
    pub uvs: Vec<SemioUv>,
    pub colors: Vec<SemioRgba>,
    pub indices: Vec<u32>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for ReplacePrimitiveGeometry
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<ReplacePrimitiveGeometry>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "primitive-geometry", kind: "replace-primitive-geometry", record: "ReplacedPrimitiveGeometry" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace primitive \"{}\" geometry in mesh \"{}\"", self.primitive_id, self.mesh_id), &format!("Geometrie von Primitiv \"{}\" in Netz \"{}\" ersetzen", self.primitive_id, self.mesh_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.primitive_id.clone()]
    }
}
//#endregion 🔖️Payload
