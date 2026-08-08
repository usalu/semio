pub struct EnergyModelEngine { projection: crate::artifacts::model::mutations::EnergyModelDocument }
impl protocol::ArtifactEngine for EnergyModelEngine {
    type Projection = crate::artifacts::model::mutations::EnergyModelDocument;
    type Mutation = crate::artifacts::model::mutations::EnergyModelMutation;
    type Diff = crate::artifacts::model::mutations::EnergyModelMutation;
    fn projection(&self) -> &Self::Projection { &self.projection }
    fn apply(&mut self, m: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        crate::artifacts::model::mutations::apply_model_mutation(&mut self.projection, m);
        Ok(m.diff(&self.projection))
    }
    fn inverse(&self, m: &Self::Mutation) -> Vec<Self::Mutation> { m.inverse(&self.projection) }
}
