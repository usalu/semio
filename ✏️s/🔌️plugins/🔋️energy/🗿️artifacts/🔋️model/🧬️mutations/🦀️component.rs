//! 🧬️ EnergyModel artifact — minimal mutation dispatch.
use serde::{Deserialize, Serialize};
use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum EnergyModelMutation {
    #[default]
    NoMutation,
    SetDocument {
        #[dsl(block)]
        document: EnergyModelDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EnergyModelDocument {
    pub schema: String,
}

pub fn apply_model_mutation(projection: &mut EnergyModelDocument, mutation: &EnergyModelMutation) {
    match mutation {
        EnergyModelMutation::NoMutation => {}
        EnergyModelMutation::SetDocument { document } => *projection = document.clone(),
    }
}

impl Mutation<EnergyModelDocument> for EnergyModelMutation {
    type Diff = EnergyModelMutation;
    fn diff(&self, _p: &EnergyModelDocument) -> Self::Diff { self.clone() }
    fn inverse(&self, p: &EnergyModelDocument) -> Vec<Self> {
        vec![EnergyModelMutation::SetDocument { document: p.clone() }]
    }
}
