//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::ReplaceAirflowNetwork;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReplaceAirflowNetwork
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReplaceAirflowNetwork>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "airflow-network", kind: "replace-airflow-network", record: "ReplacedAirflowNetwork" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&{
        if self.present {
            format!("Replace airflow network with {} zone nodes", self.zone_ids.len())
        } else {
            "Detach the airflow network".to_string()
        }
        }, &{
        if self.present {
            format!("Luftströmungsnetz durch {} Zonenknoten ersetzen", self.zone_ids.len())
        } else {
            "Luftströmungsnetz entfernen".to_string()
        }
        })
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ReplaceAirflowNetwork
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ReplaceAirflowNetwork>,
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
