//! 🎨️ Revision-guarded exact native PNG sample paint.

use crate::io::{PngNativePaint, PngRegion};
use crate::schema::diff::PngDiff;
use crate::schema::mutations::{PngMutation, SetSnapshot};
use crate::PngSnapshot;
use protocol::DiffAlgebra;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintNativeSamplesMutation {
    pub revision: String,
    pub region: PngRegion,
    pub paint: PngNativePaint,
    pub result: PngSnapshot,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<PngSnapshot, PngMutation> for PaintNativeSamplesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "native-sample-region", kind: "paint-native-samples", record: "PaintNativeSamples" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        if crate::io::png_revision(base) != self.revision {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, "png: source revision changed", ["native-sample-region"]);
        }
        match crate::io::validate_completed_native_paint(base, &self.result, self.region, self.paint) {
            Ok(()) => protocol::MutationOutcome::new(PngDiff::between(base, &self.result)),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["native-sample-region"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        Ok(vec![PngMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })])
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
    let revision = crate::io::png_revision(&base);
    let region = PngRegion { x: 0, y: 0, width: 1, height: 1 };
    let paint = PngNativePaint::rgba(0, 0, 0, 255);
    let result = crate::io::paint_native_region_owned_controlled(
        &base,
        &revision,
        region,
        paint,
        crate::io::MAXIMUM_NATIVE_PAINT_OWNED_BYTES,
        &mut |_| true,
    ).expect("default PNG native paint");
    PngMutation::PaintNativeSamples(PaintNativeSamplesMutation {
        revision,
        region,
        paint,
        result,
    })
}
