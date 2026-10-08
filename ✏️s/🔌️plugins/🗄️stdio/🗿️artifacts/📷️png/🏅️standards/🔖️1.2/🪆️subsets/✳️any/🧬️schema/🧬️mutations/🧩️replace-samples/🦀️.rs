//! 🧩️ Unguarded rectangle setter: the concrete undo of every paint, carrying the base samples of exactly that region.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::{PngRegion, PngSampleRect};
use crate::PngSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceSamples {
    pub region: PngRegion,
    pub samples: Vec<u16>,
}

impl protocol::MutationKind<PngSnapshot, PngMutation> for ReplaceSamples {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "samples", kind: "replace-samples", record: "ReplaceSamples" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let rect = PngSampleRect { region: self.region, samples: self.samples.clone() };
        match base.image.validate_rect(&rect) {
            Ok(()) => protocol::MutationOutcome::new(PngDiff { rects: (base.image.region_samples(self.region).as_deref() != Some(self.samples.as_slice())).then_some(rect).into_iter().collect(), ..PngDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["native-sample-region"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        Ok(base.image.region_samples(self.region).filter(|samples| *samples != self.samples).map(|samples| PngMutation::ReplaceSamples(ReplaceSamples { region: self.region, samples })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace samples", "Abtastwerte ersetzen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("native-region:{},{},{},{}", self.region.x, self.region.y, self.region.width, self.region.height)]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    PngMutation::ReplaceSamples(ReplaceSamples { region: PngRegion { x: 0, y: 0, width: 1, height: 1 }, samples: vec![0, 0, 0, 255] })
}
