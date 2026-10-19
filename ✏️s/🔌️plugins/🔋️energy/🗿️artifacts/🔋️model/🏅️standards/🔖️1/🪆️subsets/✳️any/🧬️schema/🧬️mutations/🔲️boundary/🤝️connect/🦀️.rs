//! 🤝️ Energy model mutation — `ConnectSurfaces`: Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🤝️ `connect-surfaces` payload. Creates the adjacency relationship between two existing surfaces. `AdjacencyPair` carries no id of its own, so the pair itself is the address, unordered — connecting B to A when A is already connected to B is a duplicate. The row lands at its ascending `(a, b)` position so `disconnect` ∘ `connect` restores the document exactly.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-surfaces")]
pub struct ConnectSurfaces {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn connect_surfaces<Op: From<ConnectSurfaces>>(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId, index: Option<u32>) -> Op {
    ConnectSurfaces { surface_a_id, surface_b_id, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ConnectSurfaces
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ConnectSurfaces> + From<crate::DisconnectSurfaces>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0), &format!("Oberflächen {} und {} verbinden", self.surface_a_id.0, self.surface_b_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ConnectSurfaces
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ConnectSurfaces> + From<crate::DisconnectSurfaces>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "surfaces", kind: "connect-surfaces", record: "ConnectedSurfaces" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
