//! 🔺️ Owned PNG native image diff.

use crate::PngSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.diff")]
pub struct PngDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub image: Option<crate::schema::snapshot::PngImage>,
}

impl MutationDiff<PngSnapshot> for PngDiff {
    fn apply(&self, base: &PngSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<PngSnapshot> {
        let next = PngSnapshot { schema: base.schema.clone(), image: self.image.clone().unwrap_or_else(|| base.image.clone()) };
        next.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["image"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.image.is_some() {
            self.image = other.image;
        }
    }
}

impl DiffAlgebra<PngSnapshot> for PngDiff {
    fn inverse(&self, base: &PngSnapshot) -> Self {
        match &self.image {
            Some(_) => Self { image: Some(base.image.clone()) },
            None => Self::default(),
        }
    }

    fn between(base: &PngSnapshot, other: &PngSnapshot) -> Self {
        Self { image: (base.image != other.image).then(|| other.image.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.image.is_none()
    }
}

#[cfg(test)]
pub(crate) fn demo_snap_a() -> PngSnapshot {
    PngSnapshot::default()
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<PngDiff> {
    vec![PngDiff::default(), PngDiff { image: Some(crate::schema::snapshot::PngImage::default()) }]
}
