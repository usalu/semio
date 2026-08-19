//! 🧬️ S Space index diff schema — sparse field delta over the artifact. `artifacts` is a whole-field
//! replace (mirrors `DagDiff.content`'s own whole-collection-field pattern): every mutation below
//! computes the full next `Vec<SpaceArtifactRow>` and sets it as one field, never a per-row delta.
//! `MutationDiff` (apply/absorb) is hand-written below — mirrors `SHomeDiff`'s own hand-written impl
//! (that one lives in the separate `🔺️diff/📝️text` facet this artifact does not have this wave; kept
//! here instead, see `$T/📓️w1-e-report.md` scope note).

use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactRow};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Space index artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space")]
pub struct SSpaceDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub artifacts: Option<Vec<SpaceArtifactRow>>,
}
//#endregion 🔖️Diff

//#region 🔖️Apply
impl protocol::MutationDiff<SSpaceSnapshot> for SSpaceDiff {
    async fn apply(&self, snapshot: &SSpaceSnapshot) -> protocol::MutationApplyResult<SSpaceSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(artifacts) = &self.artifacts {
                next.artifacts = artifacts.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(artifacts);
    }
}
//#endregion 🔖️Apply
