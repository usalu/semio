//! 🎨️ Revision-guarded indexed BMP region paint.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::{BmpMutation, ReplaceSamples};
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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


impl PaintIndexedRegion {
    fn region(&self) -> crate::schema::snapshot::BmpRegion {
        crate::schema::snapshot::BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for PaintIndexedRegion {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "paint", entity: "indexed-region", kind: "paint-indexed-region", record: "PaintIndexedRegion" };

    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let region = self.region();
        match crate::schema::operations::paint_indexed_rect(base, &self.revision, region, self.palette_index) {
            Ok(rect) => protocol::MutationOutcome::new(BmpDiff { rects: rect.into_iter().collect(), ..BmpDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["indexed-region"]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        let region = self.region();
        let changed = crate::schema::operations::checked_region(&base.image, region).ok().and_then(|()| crate::schema::operations::indexed_rect(base, region, self.palette_index));
        Ok(changed.and_then(|rect| base.image.region_rect(rect.region)).map(|rect| BmpMutation::ReplaceSamples(ReplaceSamples { region: rect.region, indices: rect.indices, samples: rect.samples })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Paint indexed region", "Indizierten Bereich malen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("region:{},{},{},{}", self.x, self.y, self.width, self.height)]
    }
}
