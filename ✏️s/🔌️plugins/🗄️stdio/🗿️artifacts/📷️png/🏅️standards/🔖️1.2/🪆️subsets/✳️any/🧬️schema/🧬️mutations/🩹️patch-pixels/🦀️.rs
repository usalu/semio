//! 🩹️ Revision-guarded exact RGBA8 PNG region paint.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::{PngMutation, ReplaceSamples};
use crate::PngSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PatchPixelsMutation {
    pub revision: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}


impl PatchPixelsMutation {
    fn region(&self) -> crate::schema::snapshot::PngRegion {
        crate::schema::snapshot::PngRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

impl protocol::MutationKind<PngSnapshot, PngMutation> for PatchPixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "rgba8-region", kind: "patch-pixels", record: "PatchPixels" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        match crate::schema::operations::paint_rgba8_rect(base, &self.revision, self.region(), [self.red, self.green, self.blue, self.alpha]) {
            Ok(rect) => protocol::MutationOutcome::new(PngDiff { rects: rect.into_iter().collect(), ..PngDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["rgba8-region"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        let changed = crate::schema::operations::native_rect(base, self.region(), crate::schema::snapshot::PngNativePaint::rgba(self.red.into(), self.green.into(), self.blue.into(), self.alpha.into()));
        Ok(changed.and_then(|rect| base.image.region_samples(rect.region)).map(|samples| PngMutation::ReplaceSamples(ReplaceSamples { region: self.region(), samples })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint RGBA8 region", "RGBA8-Bereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.x, self.y, self.width, self.height)]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    let base = PngSnapshot::default();
    PngMutation::PatchPixels(PatchPixelsMutation { revision: crate::schema::operations::png_revision(&base), x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 })
}
