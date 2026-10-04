//! 🔺️ Exact BMP byte-authority diff.

use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_os_kernel::DslDiff)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.diff")]
pub struct BmpDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
}

impl MutationDiff<BmpSnapshot> for BmpDiff {
    fn apply(&self, base: &BmpSnapshot) -> MutationApplyResult<BmpSnapshot> {
        let next = BmpSnapshot { schema: base.schema.clone(), bytes: self.bytes.clone().unwrap_or_else(|| base.bytes.clone()) };
        crate::io::bmp_layout(&next).map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-bytes", message).at(["bytes"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.bytes.is_some() {
            self.bytes = other.bytes;
        }
    }
}

impl DiffAlgebra<BmpSnapshot> for BmpDiff {
    fn inverse(&self, base: &BmpSnapshot) -> Self {
        match &self.bytes {
            Some(_) => Self { bytes: Some(base.bytes.clone()) },
            None => Self::default(),
        }
    }

    fn between(base: &BmpSnapshot, other: &BmpSnapshot) -> Self {
        Self { bytes: (base.bytes != other.bytes).then(|| other.bytes.clone()) }
    }

    fn is_empty(&self) -> bool {
        self.bytes.is_none()
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
    vec![BmpDiff::default(), BmpDiff { bytes: Some(crate::io::empty_bmp_bytes()) }]
}
