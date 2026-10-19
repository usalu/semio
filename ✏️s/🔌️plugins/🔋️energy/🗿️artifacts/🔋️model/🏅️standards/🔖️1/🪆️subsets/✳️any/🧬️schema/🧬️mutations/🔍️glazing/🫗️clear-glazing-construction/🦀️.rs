//! 🫗️ Energy model mutation — `ClearFenestrationGlazingConstruction`: Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫗️ `clear-fenestration-glazing-construction` payload. Empties the fenestration's optional glazing slot, handing the optics back to `uValueWM2k`/`shgc`/`vlt`. Refused when the slot is already empty, so an undo chain can never invent a clear that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "clear-fenestration-glazing-construction")]
pub struct ClearFenestrationGlazingConstruction {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn clear_fenestration_glazing_construction<Op: From<ClearFenestrationGlazingConstruction>>(id: crate::model::EntityId) -> Op {
    ClearFenestrationGlazingConstruction { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ClearFenestrationGlazingConstruction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ClearFenestrationGlazingConstruction> + From<crate::BindFenestrationGlazingConstruction>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Clear fenestration {} glazing construction", self.id.0), &format!("Verglasungsaufbau von Fenster {} entfernen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ClearFenestrationGlazingConstruction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ClearFenestrationGlazingConstruction> + From<crate::BindFenestrationGlazingConstruction>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "clear", entity: "fenestration", kind: "clear-fenestration-glazing-construction", record: "ClearedFenestrationGlazingConstruction" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
