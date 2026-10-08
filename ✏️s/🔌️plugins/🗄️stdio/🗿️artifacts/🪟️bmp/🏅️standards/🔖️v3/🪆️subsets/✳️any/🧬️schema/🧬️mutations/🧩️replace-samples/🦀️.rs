//! 🧩️ Unguarded rectangle setter: the concrete undo of every paint, carrying the base pixels of exactly that region.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::schema::snapshot::{BmpNativeSample, BmpRegion, BmpSampleRect};
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplaceSamples {
    pub region: BmpRegion,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub indices: Vec<u8>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub samples: Vec<BmpNativeSample>,
}

impl ReplaceSamples {
    fn rect(&self) -> BmpSampleRect {
        BmpSampleRect { region: self.region, indices: self.indices.clone(), samples: self.samples.clone() }
    }
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ReplaceSamples {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "samples", kind: "replace-samples", record: "ReplaceSamples" };

    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let rect = self.rect();
        match base.image.validate_rect(&rect) {
            Ok(()) => protocol::MutationOutcome::new(BmpDiff { image: None, rects: (base.image.region_rect(self.region).as_ref() != Some(&rect)).then_some(rect).into_iter().collect() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["sample-region"]),
        }
    }

    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        let rect = self.rect();
        Ok(base.image.region_rect(self.region).filter(|current| *current != rect).map(|current| BmpMutation::ReplaceSamples(ReplaceSamples { region: current.region, indices: current.indices, samples: current.samples })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace samples", "Abtastwerte ersetzen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.region.x, self.region.y, self.region.width, self.region.height)]
    }
}
