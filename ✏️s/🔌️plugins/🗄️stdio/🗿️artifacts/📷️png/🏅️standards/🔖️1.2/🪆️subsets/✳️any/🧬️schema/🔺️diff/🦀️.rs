//! 🔺️ Sparse PNG diff: a whole-image replacement, a gamma setter and ordered sample rectangles.

use crate::schema::snapshot::{PngGammaValue, PngImage, PngSampleRect};
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
    pub image: Option<PngImage>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gamma: Option<PngGammaValue>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub rects: Vec<PngSampleRect>,
}

impl MutationDiff<PngSnapshot> for PngDiff {
    fn apply(&self, base: &PngSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<PngSnapshot> {
        let mut image = self.image.clone().unwrap_or_else(|| base.image.clone());
        if let Some(gamma) = self.gamma {
            image.gamma = gamma.gama;
        }
        self.rects.iter().try_for_each(|rect| image.write_rect(rect)).map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["rects"]))?;
        let next = PngSnapshot { schema: base.schema.clone(), image };
        next.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-image", message).at(["image"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.image.is_some() {
            *self = other;
            return;
        }
        if other.gamma.is_some() {
            self.gamma = other.gamma;
        }
        self.rects.extend(other.rects);
    }
}

impl DiffAlgebra<PngSnapshot> for PngDiff {
    /// 🔁️ Concrete diff-level undo: a replaced image comes back whole, a set gamma returns to the base value and the rectangles restore
    /// the samples they overwrote, last rectangle first, each read from the image as the earlier rectangles left it.
    fn inverse(&self, base: &PngSnapshot) -> Self {
        if self.image.is_some() {
            return Self { image: Some(base.image.clone()), ..Self::default() };
        }
        let mut running: Option<PngImage> = None;
        let mut restores: Vec<PngSampleRect> = self
            .rects
            .iter()
            .enumerate()
            .map(|(index, rect)| {
                let source = running.as_ref().unwrap_or(&base.image);
                let restore = PngSampleRect { region: rect.region, samples: source.region_samples(rect.region).unwrap_or_default() };
                if index + 1 < self.rects.len() {
                    running.get_or_insert_with(|| base.image.clone()).write_rect(rect).ok();
                }
                restore
            })
            .collect();
        restores.reverse();
        Self { image: None, gamma: self.gamma.filter(|gamma| gamma.gama != base.image.gamma).map(|_| PngGammaValue { gama: base.image.gamma }), rects: restores }
    }

    fn is_empty(&self) -> bool {
        self.image.is_none() && self.gamma.is_none() && self.rects.is_empty()
    }
}

#[cfg(test)]
pub(crate) fn demo_snap_a() -> PngSnapshot {
    PngSnapshot::default()
}

#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<PngDiff> {
    use crate::schema::snapshot::PngRegion;
    vec![
        PngDiff::default(),
        PngDiff { image: Some(PngImage::default()), ..PngDiff::default() },
        PngDiff { gamma: Some(PngGammaValue { gama: Some(45_455) }), ..PngDiff::default() },
        PngDiff { rects: vec![PngSampleRect { region: PngRegion { x: 0, y: 0, width: 1, height: 1 }, samples: vec![1, 2, 3, 4] }], ..PngDiff::default() },
    ]
}
