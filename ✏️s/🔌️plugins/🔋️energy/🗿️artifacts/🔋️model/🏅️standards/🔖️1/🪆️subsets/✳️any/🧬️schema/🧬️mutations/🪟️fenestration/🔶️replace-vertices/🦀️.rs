//! 🔶️ Energy model mutation — `ReplaceFenestrationVertices`: Swaps the aperture's own polygon. An empty ring hands the shape back to the derived area/height/sill rectangle; three or more vertices, planar and in the host surface's plane, ARE the aperture.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔶️ `replace-fenestration-vertices` payload. Swaps the aperture's own polygon. An empty ring hands the shape back to the derived area/height/sill rectangle; three or more vertices, planar and in the host surface's plane, ARE the aperture.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-fenestration-vertices")]
pub struct ReplaceFenestrationVertices {
    pub id: crate::model::EntityId,
    pub new_vertices_m: Vec<[f64; 3]>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn replace_fenestration_vertices(id: crate::model::EntityId, new_vertices_m: Vec<[f64; 3]>) -> EnergyModelMutation {
    EnergyModelMutation::ReplaceFenestrationVertices(ReplaceFenestrationVertices { id, new_vertices_m })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReplaceFenestrationVertices {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "fenestration", kind: "replace-fenestration-vertices", record: "ReplacedFenestrationVertices" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<EnergyModelMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace fenestration {} with a {}-vertex polygon", self.id.0, self.new_vertices_m.len()), &format!("Fenster {} durch ein Polygon mit {} Ecken ersetzen", self.id.0, self.new_vertices_m.len()))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
