//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::UnbindWeatherFile;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for UnbindWeatherFile
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<UnbindWeatherFile> + From<crate::BindWeatherFile>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "weather-file", kind: "unbind-weather-file", record: "UnboundWeatherFile" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unbind the weather file", "Bindung der Wetterdatei aufheben")
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for UnbindWeatherFile
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<UnbindWeatherFile> + From<crate::BindWeatherFile>,
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
