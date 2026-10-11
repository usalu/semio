//! 🧬️ S Space index diff schema — sparse field delta over the artifact. `artifacts` is a positional row delta
//! (`removed`/`inserted`/`moved`/`modified`, see `protocol::list_delta`): a mutation names exactly the rows it creates, deletes or patches.

use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Space index artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space")]
pub struct SSpaceDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub artifacts: Option<SSpaceArtifactsDelta>,
}

protocol::list_delta! {
    pub SSpaceArtifactsDelta { removal: SSpaceArtifactsRemoval, insertion: SSpaceArtifactsInsertion, relocation: SSpaceArtifactsRelocation, modification: SSpaceArtifactsModification, row: SpaceArtifactRow, patch: SSpaceArtifactPatch, key: id, values_only }
}

/// 🩹 Field patch of one artifact row; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct SSpaceArtifactPatch {
    pub name: Option<String>,
    pub kind_id: Option<String>,
    pub schema: Option<String>,
    pub dialect: Option<SpaceArtifactDialect>,
    pub created_at_ms: Option<u64>,
    pub created_by: Option<String>,
    pub updated_at_ms: Option<u64>,
    pub updated_by: Option<String>,
}
//#endregion 🔖️Diff


impl protocol::list_delta::RowPatch<SpaceArtifactRow> for SSpaceArtifactPatch {
    fn commit_into(&self, row: &mut SpaceArtifactRow, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        let patch = self;
        if let Some(value) = &patch.name {
            row.name = value.clone();
        }
        if let Some(value) = &patch.kind_id {
            row.kind_id = value.clone();
        }
        if let Some(value) = &patch.schema {
            row.schema = value.clone();
        }
        if let Some(value) = &patch.dialect {
            row.dialect = value.clone();
        }
        if let Some(value) = patch.created_at_ms {
            row.created_at_ms = value;
        }
        if let Some(value) = &patch.created_by {
            row.created_by = value.clone();
        }
        if let Some(value) = patch.updated_at_ms {
            row.updated_at_ms = value;
        }
        if let Some(value) = &patch.updated_by {
            row.updated_by = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        let earlier = self.clone();
        let first = &earlier;
        let later = &later;
        *self = {
        SSpaceArtifactPatch {
            name: later.name.clone().or_else(|| first.name.clone()),
            kind_id: later.kind_id.clone().or_else(|| first.kind_id.clone()),
            schema: later.schema.clone().or_else(|| first.schema.clone()),
            dialect: later.dialect.clone().or_else(|| first.dialect.clone()),
            created_at_ms: later.created_at_ms.or(first.created_at_ms),
            created_by: later.created_by.clone().or_else(|| first.created_by.clone()),
            updated_at_ms: later.updated_at_ms.or(first.updated_at_ms),
            updated_by: later.updated_by.clone().or_else(|| first.updated_by.clone()),
        }
        };
    }
    fn inverse(&self, row: &SpaceArtifactRow) -> Self {
        let patch = self;
        let base = row;
        SSpaceArtifactPatch {
                        name: patch.name.as_ref().map(|_| base.name.clone()),
            kind_id: patch.kind_id.as_ref().map(|_| base.kind_id.clone()),
            schema: patch.schema.as_ref().map(|_| base.schema.clone()),
            dialect: patch.dialect.as_ref().map(|_| base.dialect.clone()),
            created_at_ms: patch.created_at_ms.map(|_| base.created_at_ms),
            created_by: patch.created_by.as_ref().map(|_| base.created_by.clone()),
            updated_at_ms: patch.updated_at_ms.map(|_| base.updated_at_ms),
            updated_by: patch.updated_by.as_ref().map(|_| base.updated_by.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        let patch = self;
        patch.name.is_none() && patch.kind_id.is_none() && patch.schema.is_none() && patch.dialect.is_none() && patch.created_at_ms.is_none() && patch.created_by.is_none() && patch.updated_at_ms.is_none() && patch.updated_by.is_none()
    }
}

//#region 🔖️Apply
impl protocol::MutationDiff<SSpaceSnapshot> for SSpaceDiff {
    fn apply(&self, snapshot: &SSpaceSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SSpaceSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.artifacts {
            next.artifacts = delta.commit_onto(&next.artifacts, capability).map_err(|error| error.under(["artifacts"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.artifacts = match (self.artifacts.take(), other.artifacts) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<SSpaceSnapshot> for SSpaceDiff {
    fn inverse(&self, base: &SSpaceSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), artifacts: self.artifacts.as_ref().map(|delta| delta.inverse(&base.artifacts)) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.artifacts.as_ref().is_none_or(SSpaceArtifactsDelta::is_empty)
    }
}
//#endregion 🔖️Apply
