//! ⚙️ SHome artifact — headless compute (constitutional: engine).

//#region 🔖️ArtifactEngine
pub struct SHomeEngine {
    projection: crate::artifacts::home::SHomeDocument,
}

impl SHomeEngine {
    pub fn new(projection: crate::artifacts::home::SHomeDocument) -> Self {
        Self { projection }
    }
}

impl protocol::ArtifactEngine for SHomeEngine {
    type Projection = crate::artifacts::home::SHomeDocument;
    type Mutation = crate::artifacts::home::mutations::SHomeMutation;
    type Diff = crate::artifacts::home::diff::SHomeDiff;

    fn projection(&self) -> &Self::Projection {
        &self.projection
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::home::mutations::apply_shome_mutation(&mut self.projection, mutation);
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }
}
//#endregion 🔖️ArtifactEngine
