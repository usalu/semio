//! 🔺️ Energy model mutation — `ReplaceSurfaceVertices`: Swaps the surface's whole polygon. Geometry is one value — moving a single corner would leave the other vertices describing a different plane — so the taxonomy's `replace` verb carries the full ring, minimum three vertices.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔺️ `replace-surface-vertices` payload. Swaps the surface's whole polygon. Geometry is one value — moving a single corner would leave the other vertices describing a different plane — so the taxonomy's `replace` verb carries the full ring, minimum three vertices.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-surface-vertices")]
pub struct ReplaceSurfaceVertices {
    pub id: crate::model::EntityId,
    pub new_vertices_m: Vec<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_surface_vertices(id: crate::model::EntityId, new_vertices_m: Vec<[f64; 3]>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceSurfaceVertices(ReplaceSurfaceVertices { id, new_vertices_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceSurfaceVertices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "surface", kind: "replace-surface-vertices", record: "ReplacedSurfaceVertices" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Replace surface {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len())
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
