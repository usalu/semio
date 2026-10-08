//! 🔺️ Sparse BMP diff: a whole-image replacement and ordered pixel rectangles.

use crate::schema::snapshot::{BmpImage, BmpSampleRect};
use crate::BmpSnapshot;
use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationApplyResult, MutationDiff};

#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp.diff")]
pub struct BmpDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub image: Option<BmpImage>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub rects: Vec<BmpSampleRect>,
}

impl MutationDiff<BmpSnapshot> for BmpDiff {
    fn apply(&self, base: &BmpSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<BmpSnapshot> {
        let mut image = self.image.clone().unwrap_or_else(|| base.image.clone());
        self.rects.iter().try_for_each(|rect| image.write_rect(rect)).map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["rects"]))?;
        let next = BmpSnapshot { schema: base.schema.clone(), image };
        next.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["image"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.image.is_some() {
            *self = other;
            return;
        }
        self.rects.extend(other.rects);
    }
}

impl DiffAlgebra<BmpSnapshot> for BmpDiff {
    /// 🔁️ Concrete diff-level undo: a replaced image comes back whole and the rectangles restore the pixels they overwrote, last rectangle
    /// first, each read from the image as the earlier rectangles left it.
    fn inverse(&self, base: &BmpSnapshot) -> Self {
        if self.image.is_some() {
            return Self { image: Some(base.image.clone()), rects: Vec::new() };
        }
        let mut running: Option<BmpImage> = None;
        let mut restores: Vec<BmpSampleRect> = self
            .rects
            .iter()
            .enumerate()
            .map(|(index, rect)| {
                let source = running.as_ref().unwrap_or(&base.image);
                let restore = source.region_rect(rect.region).unwrap_or_default();
                if index + 1 < self.rects.len() {
                    running.get_or_insert_with(|| base.image.clone()).write_rect(rect).ok();
                }
                restore
            })
            .collect();
        restores.reverse();
        Self { image: None, rects: restores }
    }

    fn is_empty(&self) -> bool {
        self.image.is_none() && self.rects.is_empty()
    }
}

#[cfg(test)]
pub(crate) fn demo_snap_a() -> BmpSnapshot {
    BmpSnapshot::default()
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<BmpDiff> {
    use crate::schema::snapshot::{BmpNativeSample, BmpRegion};
    vec![
        BmpDiff::default(),
        BmpDiff { image: Some(BmpImage::default()), rects: Vec::new() },
        BmpDiff { image: None, rects: vec![BmpSampleRect { region: BmpRegion { x: 0, y: 0, width: 1, height: 1 }, indices: Vec::new(), samples: vec![BmpNativeSample { red: 1, green: 2, blue: 3, alpha: 0, reserved: 0 }] }] },
    ]
}
