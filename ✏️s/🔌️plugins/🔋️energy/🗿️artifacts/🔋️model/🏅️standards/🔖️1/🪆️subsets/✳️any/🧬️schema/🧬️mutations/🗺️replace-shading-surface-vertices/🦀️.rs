//! 🗺️ Energy model mutation — `ReplaceShadingSurfaceVertices`: Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗺️ `replace-shading-surface-vertices` payload. Swaps the shading surface's whole polygon — geometry is one value, so the taxonomy's `replace` verb carries the full ring, minimum three vertices.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-shading-surface-vertices")]
pub struct ReplaceShadingSurfaceVertices {
    pub id: crate::model::EntityId,
    pub new_vertices_m: Vec<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_shading_surface_vertices(id: crate::model::EntityId, new_vertices_m: Vec<[f64; 3]>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceShadingSurfaceVertices(ReplaceShadingSurfaceVertices { id, new_vertices_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceShadingSurfaceVertices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "shading-surface", kind: "replace-shading-surface-vertices", record: "ReplacedShadingSurfaceVertices" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace shading surface {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len())
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
