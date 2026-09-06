//! 🌦️ Energy model mutation — `BindWeatherFile`: Attaches the `weather` link slot to an `🌦️epw` stdio artifact addressed by its `ArtifactRef` URI. The link is pinned to the target's head; the model never embeds weather bytes.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌦️ `bind-weather-file` payload. Attaches the `weather` link slot to an `🌦️epw` stdio artifact addressed by its `ArtifactRef` URI. The link is pinned to the target's head; the model never embeds weather bytes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "bind-weather-file")]
pub struct BindWeatherFile {
    pub target_uri: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn bind_weather_file(target_uri: String) -> EnergyModelMutation {
    EnergyModelMutation::BindWeatherFile(BindWeatherFile { target_uri })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for BindWeatherFile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "weather-file", kind: "bind-weather-file", record: "BoundWeatherFile" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Bind weather file {}", self.target_uri)
    }

    fn target(&self) -> Vec<String> {
        vec![self.target_uri.clone()]
    }
}
//#endregion 🔖️Mutation
