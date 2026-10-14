//! 🧲️ Energy model mutation — `ChangeFenestrationSurface`: Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧲️ `change-fenestration-surface` payload. Rehosts the fenestration on another existing surface — the surface whose orientation, tilt and zone the window then inherits.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-surface")]
pub struct ChangeFenestrationSurface {
    pub id: crate::model::EntityId,
    pub new_surface_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_surface<Op: From<ChangeFenestrationSurface>>(id: crate::model::EntityId, new_surface_id: crate::model::EntityId) -> Op {
    ChangeFenestrationSurface { id, new_surface_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeFenestrationSurface
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeFenestrationSurface>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fenestration", kind: "change-fenestration-surface", record: "ChangedFenestrationSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move fenestration {} to surface {}", self.id.0, self.new_surface_id.0), &format!("Fenster {} nach Oberfläche {} verschieben", self.id.0, self.new_surface_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
