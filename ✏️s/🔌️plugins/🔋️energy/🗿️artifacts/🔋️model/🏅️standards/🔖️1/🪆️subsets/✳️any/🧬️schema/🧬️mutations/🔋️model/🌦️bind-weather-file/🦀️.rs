//! 🌦️ Energy model mutation — `BindWeatherFile`: Attaches the `weather` link slot to an `🌦️epw` stdio artifact addressed by its `ArtifactRef` identity. The link is pinned to the target's head; the model never embeds weather bytes.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌦️ `bind-weather-file` payload. Attaches the `weather` link slot to an `🌦️epw` stdio artifact addressed by its `ArtifactRef` identity. The link is pinned to the target's head; the model never embeds weather bytes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "bind-weather-file")]
pub struct BindWeatherFile {
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn bind_weather_file<Op: From<BindWeatherFile>>(target: semio_framework_artifact_reference::ArtifactRef) -> Op {
    BindWeatherFile { target }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for BindWeatherFile
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<BindWeatherFile> + From<crate::UnbindWeatherFile>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "weather-file", kind: "bind-weather-file", record: "BoundWeatherFile" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Bind weather file {}", self.target.artifact_id), &format!("Wetterdatei {} binden", self.target.artifact_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.target.artifact_id.clone()]
    }
}
//#endregion 🔖️Mutation
