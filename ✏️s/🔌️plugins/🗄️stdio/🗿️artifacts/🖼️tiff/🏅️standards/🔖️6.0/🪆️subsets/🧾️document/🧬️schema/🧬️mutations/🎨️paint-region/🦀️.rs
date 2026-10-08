//! 🎨️ Revision-guarded exact tiled TIFF region paint.

use crate::schema::diff::{TiffDiff, TiffIfdDiff, TiffIfdModified, TiffIfdsDiff, TiffSampleRun};
use crate::schema::mutations::{ReplaceSamplesMutation, TiffMutation};
use crate::TiffSnapshot;
use samples::{paint_tiff_region_runs, TiffRegion};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintRegionMutation {
    pub revision: String,
    pub ifd_index: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl PaintRegionMutation {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn region(&self) -> TiffRegion {
        TiffRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn color(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

impl protocol::MutationKind<TiffSnapshot, TiffMutation> for PaintRegionMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "tiled-region", kind: "paint-region", record: "PaintRegion" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        match paint_tiff_region_runs(base, &self.revision, self.ifd_index, self.region(), self.color(), &mut |_, _| true) {
            Ok(runs) if runs.is_empty() => protocol::MutationOutcome::new(TiffDiff::default()),
            Ok(runs) => protocol::MutationOutcome::new(TiffDiff { ifds: Some(TiffIfdsDiff { modified: vec![TiffIfdModified { index: self.ifd_index, diff: TiffIfdDiff { runs, ..TiffIfdDiff::default() } }], ..TiffIfdsDiff::default() }) }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, [format!("ifd:{}:region:{},{},{},{}", self.ifd_index, self.x, self.y, self.width, self.height)]),
        }
    }

    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        let current = |run: &TiffSampleRun| base.ifds.get(self.ifd_index).and_then(|ifd| ifd.blocks.get(run.block)).and_then(|block| block.samples.get(run.offset..run.offset + run.samples.len()));
        let runs = paint_tiff_region_runs(base, &self.revision, self.ifd_index, self.region(), self.color(), &mut |_, _| true).unwrap_or_default();
        Ok(runs.iter().filter_map(|run| current(run).map(|words| TiffMutation::ReplaceSamples(ReplaceSamplesMutation { ifd_index: self.ifd_index, block: run.block, offset: run.offset, samples: words.to_vec() }))).collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint tiled region", "Kachelbereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("ifd:{}:region:{},{},{},{}", self.ifd_index, self.x, self.y, self.width, self.height)]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> TiffMutation {
    let base = crate::standards::v6_0::subsets::document::schema::blank_tiff_snapshot();
    TiffMutation::PaintRegion(PaintRegionMutation { revision: crate::standards::v6_0::subsets::document::schema::mutations::paint_region::samples::tiff_revision(&base), ifd_index: 0, x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 })
}

#[path="🖌️samples/🦀️.rs"]
pub mod samples;
