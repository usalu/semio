//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::BindWeatherFile;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for BindWeatherFile
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<BindWeatherFile> + From<crate::UnbindWeatherFile>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "weather-file", kind: "bind-weather-file", record: "BoundWeatherFile" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Bind weather file {}", self.link.target.artifact_id), &format!("Wetterdatei {} binden", self.link.target.artifact_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.link.target.artifact_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for BindWeatherFile
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<BindWeatherFile> + From<crate::UnbindWeatherFile>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
