//! 🎨️ Revision-guarded exact native PNG sample paint.

use crate::schema::snapshot::{PngNativePaint, PngRegion};
use crate::schema::diff::PngDiff;
use crate::schema::mutations::{PngMutation, ReplaceSamples};
use crate::PngSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintNativeSamplesMutation {
    pub revision: String,
    pub region: PngRegion,
    pub paint: PngNativePaint,
}


impl protocol::MutationKind<PngSnapshot, PngMutation> for PaintNativeSamplesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "native-sample-region", kind: "paint-native-samples", record: "PaintNativeSamples" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        match crate::schema::operations::paint_native_rect(base, &self.revision, self.region, self.paint) {
            Ok(rect) => protocol::MutationOutcome::new(PngDiff { rects: rect.into_iter().collect(), ..PngDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["native-sample-region"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        let changed = crate::schema::operations::native_rect(base, self.region, self.paint);
        Ok(changed.and_then(|rect| base.image.region_samples(rect.region)).map(|samples| PngMutation::ReplaceSamples(ReplaceSamples { region: self.region, samples })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint native PNG samples", "Native PNG-Abtastwerte malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("native-region:{},{},{},{}", self.region.x, self.region.y, self.region.width, self.region.height)]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    let base = PngSnapshot::default();
    let revision = crate::schema::operations::png_revision(&base);
    let region = PngRegion { x: 0, y: 0, width: 1, height: 1 };
    let paint = PngNativePaint::rgba(0, 0, 0, 255);
    PngMutation::PaintNativeSamples(PaintNativeSamplesMutation {
        revision,
        region,
        paint,
    })
}
