//! 🔺️ Exact PNG byte-authority diff.

use crate::PngSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.png.diff")]
pub struct PngDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

impl MutationDiff<PngSnapshot> for PngDiff {
    fn apply(&self, base: &PngSnapshot) -> MutationApplyResult<PngSnapshot> {
        let next = PngSnapshot { schema: base.schema.clone(), bytes: self.bytes.clone().unwrap_or_else(|| base.bytes.clone()) };
        crate::standards::v1_2::subsets::any::io::png_layout(&next).map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-bytes", message).at(["bytes"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.bytes.is_some() {
            self.bytes = other.bytes;
        }
    }
}

impl DiffAlgebra<PngSnapshot> for PngDiff {
    fn inverse(&self, base: &PngSnapshot) -> Self {
        match self.bytes {
            Some(_) => Self { bytes: Some(base.bytes.clone()) },
            None => Self::default(),
        }
    }

    fn between(base: &PngSnapshot, other: &PngSnapshot) -> Self {
        Self { bytes: (base.bytes != other.bytes).then(|| other.bytes.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.bytes.is_none()
    }
}

pub fn diff_set_snapshot(base: &PngSnapshot, next: &PngSnapshot) -> PngDiff {
    PngDiff::between(base, next)
}

#[cfg(test)]
pub(crate) fn demo_snap_a() -> PngSnapshot {
    PngSnapshot::default()
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<PngDiff> {
    vec![PngDiff::default(), PngDiff { bytes: Some(crate::standards::v1_2::subsets::any::io::empty_png_bytes()) }]
}
