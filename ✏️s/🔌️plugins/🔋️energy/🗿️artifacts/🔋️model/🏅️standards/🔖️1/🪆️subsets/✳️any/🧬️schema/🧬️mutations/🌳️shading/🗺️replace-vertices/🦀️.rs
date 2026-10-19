//! 🗺️ Energy model mutation — `ReplaceShadingSurfaceVertices`: Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗺️ `replace-shading-surface-vertices` payload. Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-shading-surface-vertices")]
pub struct ReplaceShadingSurfaceVertices {
    pub id: crate::model::EntityId,
    pub new_vertices_m: Vec<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_shading_surface_vertices<Op: From<ReplaceShadingSurfaceVertices>>(id: crate::model::EntityId, new_vertices_m: Vec<[f64; 3]>) -> Op {
    ReplaceShadingSurfaceVertices { id, new_vertices_m }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReplaceShadingSurfaceVertices
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReplaceShadingSurfaceVertices>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "shading-surface", kind: "replace-shading-surface-vertices", record: "ReplacedShadingSurfaceVertices" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace shading surface {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len()), &format!("Verschattungsfläche {} durch ein Polygon mit {} Ecken ersetzen", self.id.0, self.new_vertices_m.len()))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ReplaceShadingSurfaceVertices
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ReplaceShadingSurfaceVertices>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
