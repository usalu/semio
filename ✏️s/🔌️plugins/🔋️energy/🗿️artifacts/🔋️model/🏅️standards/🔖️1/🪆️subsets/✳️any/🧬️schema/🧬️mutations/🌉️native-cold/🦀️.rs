//! 🌉️ The cold mutation contract over the native persistent snapshot runs on its plain reference projection.
use crate::snapshot::EnergyModelSnapshot as Native;
use crate::snapshot::reference::EnergyModelSnapshot as Reference;
use crate::EnergyModelMutation;

impl protocol::ColdMutation<Native> for EnergyModelMutation {
    type ReferenceDiff = crate::diff::ReferenceEnergyModelDiff;

    fn diff(&self, base: &Native) -> protocol::MutationOutcome<Self::ReferenceDiff> {
        match base.to_reference() {
            Ok(reference) => <Self as protocol::ColdMutation<Reference>>::diff(self, &reference),
            Err(error) => protocol::MutationOutcome::fatal("mutation.native-bridge", error.to_string(), Vec::<String>::new()),
        }
    }

    fn inverse(&self, base: &Native) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        <Self as protocol::ColdMutation<Reference>>::inverse(self, &base.to_reference()?)
    }
}
