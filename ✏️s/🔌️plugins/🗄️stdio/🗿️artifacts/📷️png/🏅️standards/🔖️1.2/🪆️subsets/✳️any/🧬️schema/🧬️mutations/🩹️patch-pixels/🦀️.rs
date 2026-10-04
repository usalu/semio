//! 🩹️ Revision-guarded exact RGBA8 PNG region paint.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::{PngMutation, SetSnapshot};
use crate::PngSnapshot;
use protocol::DiffAlgebra;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<PngSnapshot, PngMutation> for PatchPixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "rgba8-region", kind: "patch-pixels", record: "PatchPixels" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let region = crate::io::PngRegion { x: self.x, y: self.y, width: self.width, height: self.height };
        match crate::io::paint_rgba8_region_controlled(base, &self.revision, region, [self.red, self.green, self.blue, self.alpha], &mut |_, _| true) {
            Ok(next) => protocol::MutationOutcome::new(PngDiff::between(base, &next)),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["rgba8-region"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PngMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]

    })())
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
    PngMutation::PatchPixels(PatchPixelsMutation { revision: crate::io::png_revision(&base), x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 })
}
