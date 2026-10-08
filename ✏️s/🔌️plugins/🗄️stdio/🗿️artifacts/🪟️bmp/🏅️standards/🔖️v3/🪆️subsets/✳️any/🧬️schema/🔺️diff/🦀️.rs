//! 🔺️ Owned BMP native image diff.

use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.diff")]
pub struct BmpDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub image: Option<crate::schema::snapshot::BmpImage>,
}

impl MutationDiff<BmpSnapshot> for BmpDiff {
    fn apply(&self, base: &BmpSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<BmpSnapshot> {
        let next = BmpSnapshot { schema: base.schema.clone(), image: self.image.clone().unwrap_or_else(|| base.image.clone()) };
        next.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["image"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.image.is_some() {
            self.image = other.image;
        }
    }
}

impl DiffAlgebra<BmpSnapshot> for BmpDiff {
    fn inverse(&self, base: &BmpSnapshot) -> Self {
        match &self.image {
            Some(_) => Self { image: Some(base.image.clone()) },
            None => Self::default(),
        }
    }

    fn between(base: &BmpSnapshot, other: &BmpSnapshot) -> Self {
        Self { image: (base.image != other.image).then(|| other.image.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.image.is_none()
    }
}

pub fn diff_set_snapshot(base: &BmpSnapshot, next: &BmpSnapshot) -> BmpDiff {
    BmpDiff::between(base, next)
}

#[cfg(test)]
pub(crate) fn demo_snap_a() -> BmpSnapshot {
    BmpSnapshot::default()
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<BmpDiff> {
    vec![BmpDiff::default(), BmpDiff { image: Some(crate::schema::snapshot::BmpImage::default()) }]
}
