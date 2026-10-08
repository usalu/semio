//! 🎨️ Revision-guarded indexed BMP region paint.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::{BmpMutation, ReplaceImage};
use crate::BmpSnapshot;
use protocol::DiffAlgebra;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintIndexedRegion {
    pub revision: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub palette_index: u8,
}


impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintIndexedRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "indexed-region", kind: "paint-indexed-region", record: "PaintIndexedRegion" };

    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let region = crate::schema::snapshot::BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height };
        match crate::schema::operations::paint_indexed_region_controlled(base, &self.revision, region, self.palette_index, &mut |_, _| true) {
            Ok(next) => protocol::MutationOutcome::new(BmpDiff::between(base, &next)),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["indexed-region"]),
        }
    }

    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
    Ok({
        vec![BmpMutation::ReplaceImage(ReplaceImage { image: base.image.clone() })]
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint indexed region", "Indizierten Bereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.x, self.y, self.width, self.height)]
    }
}
