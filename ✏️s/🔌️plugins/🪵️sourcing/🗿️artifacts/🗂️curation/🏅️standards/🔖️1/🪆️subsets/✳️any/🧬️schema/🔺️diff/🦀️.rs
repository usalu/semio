//! 🧬️ Curation diff schema — sparse field delta over the artifact.

use crate::{CuratedItem, ObjectKindExtra, CurationSnapshot};
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse parent delta for catalog identity, sourcing entries and selection.
/// Kit content changes belong to the child's own mutation history.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationDiff {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurationStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurationCuratedDelta>,
}
impl semio_framework_value::FromValue for CurationDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            let bit = match key.as_str() { "catalog" => 1, "stockExtra" => 2, "curated" => 4, _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Curation diff field {key}"))) };
            if seen & bit != 0 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Curation diff field {key}"))); }
            seen |= bit;
            match key.as_str() {
                "catalog" => result.catalog = semio_framework_value::FromValue::from_value(value)?,
                "stockExtra" => result.stock_extra = semio_framework_value::FromValue::from_value(value)?,
                "curated" => result.curated = semio_framework_value::FromValue::from_value(value)?,
                _ => unreachable!(),
            }
        }
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
}
impl CurationDiff {
    /// 🛡 Checks typed parent replacements before applying document changes.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(catalog) = &self.catalog { validate_semio_child_identity(&catalog.child_id, &catalog.target, "kit")?; }
        Ok(())
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock-extra entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationObjectKindExtraPatchEntry {
    pub extra: ObjectKindExtra,
}

protocol::list_delta! {
    pub CurationStockExtraDelta { removal: CurationStockExtraRemoval, insertion: CurationStockExtraInsertion, relocation: CurationStockExtraRelocation, modification: CurationStockExtraModification, row: ObjectKindExtra, patch: CurationObjectKindExtraPatchEntry, key: id, values_only }
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationCuratedPatchEntry {
    pub count: Option<u32>,
}

protocol::list_delta! {
    pub CurationCuratedDelta { removal: CurationCuratedRemoval, insertion: CurationCuratedInsertion, relocation: CurationCuratedRelocation, modification: CurationCuratedModification, row: CuratedItem, patch: CurationCuratedPatchEntry, key: object_id, values_only }
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
impl protocol::list_delta::RowPatch<ObjectKindExtra> for CurationObjectKindExtraPatchEntry {
    fn commit_into(&self, row: &mut ObjectKindExtra, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if self.extra.id != self.id {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "stock entry patch cannot change its identity"));
        }
        *row = self.extra.clone();
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        *self = later;
    }
    fn inverse(&self, row: &ObjectKindExtra) -> Self {
        CurationObjectKindExtraPatchEntry { extra: row.clone() }
    }
    fn is_empty(&self) -> bool {
        false
    }
}

impl protocol::list_delta::RowPatch<CuratedItem> for CurationCuratedPatchEntry {
    fn commit_into(&self, row: &mut CuratedItem, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        if let Some(count) = self.count {
            row.count = count;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        self.count = later.count.or(self.count);
    }
    fn inverse(&self, row: &CuratedItem) -> Self {
        CurationCuratedPatchEntry { count: self.count.map(|_| row.count) }
    }
    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl MutationDiff<CurationSnapshot> for CurationDiff {
    fn apply(&self, snapshot: &CurationSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CurationSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.child-identity", message))?;
        let mut next = snapshot.clone();
        if let Some(handle) = &self.catalog {
            next.catalog = handle.clone();
        }
        if let Some(delta) = &self.stock_extra {
            next.stock_extra = delta.commit_onto(&next.stock_extra, capability).map_err(|error| error.under(["stockExtra"]))?;
        }
        if let Some(delta) = &self.curated {
            next.curated = delta.commit_onto(&next.curated, capability).map_err(|error| error.under(["curated"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.catalog.is_some() {
            self.catalog = other.catalog;
        }
        self.stock_extra = match (self.stock_extra.take(), other.stock_extra) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
        self.curated = match (self.curated.take(), other.curated) {
            (Some(mut first), Some(later)) => {
                first.absorb(later);
                Some(first)
            }
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<CurationSnapshot> for CurationDiff {
    fn inverse(&self, base: &CurationSnapshot) -> Self {
        Self {
            catalog: self.catalog.as_ref().map(|_| base.catalog.clone()),
            stock_extra: self.stock_extra.as_ref().map(|delta| delta.inverse(&base.stock_extra)),
            curated: self.curated.as_ref().map(|delta| delta.inverse(&base.curated)),
        }
    }
    fn is_empty(&self) -> bool {
        self.catalog.is_none() && self.stock_extra.as_ref().is_none_or(CurationStockExtraDelta::is_empty) && self.curated.as_ref().is_none_or(CurationCuratedDelta::is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
