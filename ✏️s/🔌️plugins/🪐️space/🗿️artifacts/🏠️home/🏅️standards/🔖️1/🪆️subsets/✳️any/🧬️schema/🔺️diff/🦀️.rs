//! 🧬️ S Home diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Home artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub catalog_generation: Option<u64>,
}
//#endregion 🔖️Diff

use crate::SHomeSnapshot;
use protocol::MutationDiff;

impl MutationDiff<SHomeSnapshot> for SHomeDiff {
    fn apply(&self, snapshot: &SHomeSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SHomeSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(value) = self.catalog_generation {
            next.catalog_generation = value;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(catalog_generation);
    }
}

impl protocol::DiffAlgebra<SHomeSnapshot> for SHomeDiff {
    fn inverse(&self, base: &SHomeSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), catalog_generation: self.catalog_generation.map(|_| base.catalog_generation) }
    }
    fn between(base: &SHomeSnapshot, other: &SHomeSnapshot) -> Self {
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), catalog_generation: (base.catalog_generation != other.catalog_generation).then_some(other.catalog_generation) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.catalog_generation.is_none()
    }
}
