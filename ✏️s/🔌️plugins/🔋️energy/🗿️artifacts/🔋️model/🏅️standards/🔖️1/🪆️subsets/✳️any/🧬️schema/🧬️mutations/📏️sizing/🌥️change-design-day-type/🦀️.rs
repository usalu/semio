//! 🌥️ Energy model mutation — `ChangeSizingObjectDesignDayType`: Swaps which design day the sizing run reads.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌥️ `change-sizing-object-design-day-type` payload. Swaps which design day the sizing run reads.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-design-day-type")]
pub struct ChangeSizingObjectDesignDayType {
    pub id: crate::model::EntityId,
    pub new_design_day_type: crate::model::DesignDayType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_sizing_object_design_day_type<Op: From<ChangeSizingObjectDesignDayType>>(id: crate::model::EntityId, new_design_day_type: crate::model::DesignDayType) -> Op {
    ChangeSizingObjectDesignDayType { id, new_design_day_type }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectDesignDayType
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSizingObjectDesignDayType>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change sizing object {} design day type to {:?}", self.id.0, self.new_design_day_type), &format!("Auslegungstagtyp von Auslegungsobjekt {} auf {:?} ändern", self.id.0, self.new_design_day_type))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSizingObjectDesignDayType
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSizingObjectDesignDayType>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sizing-object", kind: "change-sizing-object-design-day-type", record: "ChangedSizingObjectDesignDayType" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
