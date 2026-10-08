//! 🖌️ Revision-guarded direct-color BMP region paint.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::{BmpMutation, ReplaceSamples};
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintDirectRegion {
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


impl PaintDirectRegion {
    fn region(&self) -> crate::schema::snapshot::BmpRegion {
        crate::schema::snapshot::BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }

    fn color(&self) -> crate::schema::snapshot::BmpColor {
        crate::schema::snapshot::BmpColor { red: self.red, green: self.green, blue: self.blue, alpha: self.alpha }
    }
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintDirectRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "direct-region", kind: "paint-direct-region", record: "PaintDirectRegion" };

    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        match crate::schema::operations::paint_direct_rect(base, &self.revision, self.region(), self.color()) {
            Ok(rect) => protocol::MutationOutcome::new(BmpDiff { rects: rect.into_iter().collect(), ..BmpDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["direct-region"]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        let region = self.region();
        let changed = crate::schema::operations::checked_region(&base.image, region).ok().and_then(|()| crate::schema::operations::direct_rect(base, region, self.color()));
        Ok(changed.and_then(|rect| base.image.region_rect(rect.region)).map(|rect| BmpMutation::ReplaceSamples(ReplaceSamples { region: rect.region, indices: rect.indices, samples: rect.samples })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint direct-color region", "Direktfarbbereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.x, self.y, self.width, self.height)]
    }
}
