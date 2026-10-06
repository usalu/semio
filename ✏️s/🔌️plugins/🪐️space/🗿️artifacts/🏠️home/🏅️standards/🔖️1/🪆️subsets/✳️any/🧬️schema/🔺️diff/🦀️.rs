//! 🧬️ S Home diff schema — sparse field delta over the artifact.

use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Home artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub catalog_generation: Option<u64>,
}
//#endregion 🔖️Diff

use crate::standards::v1::subsets::any::schema::SHomeArtifact;
use crate::SHomeSnapshot;
use protocol::MutationDiff;

impl SHomeDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &SHomeArtifact) -> protocol::MutationApplyResult<SHomeArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(value) = self.catalog_generation {
                next.catalog_generation = value;
            }
            next
        })
    }
}

impl MutationDiff<SHomeSnapshot> for SHomeDiff {
    fn apply(&self, snapshot: &SHomeSnapshot) -> protocol::MutationApplyResult<SHomeSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(value) = self.catalog_generation {
                next.catalog_generation = value;
            }
            next
        })
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
