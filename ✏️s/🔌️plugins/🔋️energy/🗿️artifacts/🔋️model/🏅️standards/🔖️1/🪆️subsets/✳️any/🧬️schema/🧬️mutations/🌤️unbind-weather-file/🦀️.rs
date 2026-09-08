//! 🌤️ Energy model mutation — `UnbindWeatherFile`: Detaches the `weather` link slot. Refused when nothing is bound, so an undo chain can never invent an unbind that had no partner.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌤️ `unbind-weather-file` payload. Detaches the `weather` link slot. Refused when nothing is bound, so an undo chain can never invent an unbind that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "unbind-weather-file")]
pub struct UnbindWeatherFile {}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn unbind_weather_file() -> EnergyModelMutation {
    EnergyModelMutation::UnbindWeatherFile(UnbindWeatherFile {})
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for UnbindWeatherFile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "weather-file", kind: "unbind-weather-file", record: "UnboundWeatherFile" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Unbind the weather file".to_string()
    }
}
//#endregion 🔖️Mutation
