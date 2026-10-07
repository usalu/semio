//! 🖌️ Revision-guarded direct-color BMP region paint.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::{BmpMutation, SetSnapshot};
use crate::BmpSnapshot;
use protocol::DiffAlgebra;

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


impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintDirectRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "direct-region", kind: "paint-direct-region", record: "PaintDirectRegion" };

    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let region = crate::schema::snapshot::BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height };
        let color = crate::schema::snapshot::BmpColor { red: self.red, green: self.green, blue: self.blue, alpha: self.alpha };
        match crate::schema::operations::paint_direct_region_controlled(base, &self.revision, region, color, &mut |_, _| true) {
            Ok(next) => protocol::MutationOutcome::new(BmpDiff::between(base, &next)),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["direct-region"]),
        }
    }

    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![BmpMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint direct-color region", "Direktfarbbereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.x, self.y, self.width, self.height)]
    }
}
