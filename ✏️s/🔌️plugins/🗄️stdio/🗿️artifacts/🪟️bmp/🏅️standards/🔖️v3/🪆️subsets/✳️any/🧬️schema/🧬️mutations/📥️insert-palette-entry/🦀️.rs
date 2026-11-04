//! 📥️ Insert palette entry: an indexed image's palette edit with exact inverse; every sample keeps pointing at the same color.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertPaletteEntry {
    pub index: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub reserved: u8,
    pub colors_used: u32,
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for InsertPaletteEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "palette-entry", kind: "insert-palette-entry", record: "InsertPaletteEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert palette entry", "Paletteneintrag einfügen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("palette:{}", self.index)]
    }
}

impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for InsertPaletteEntry {
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        match crate::schema::operations::palette_entry_inserted(&base.image, self.index as usize, crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }, self.colors_used) {
            Ok(image) => protocol::MutationOutcome::new(BmpDiff { image: Some(image), ..BmpDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        Ok(crate::schema::operations::palette_entry_inserted(&base.image, self.index as usize, crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }, self.colors_used).ok().map(|_| BmpMutation::RemovePaletteEntry(crate::schema::mutations::RemovePaletteEntry { index: self.index, colors_used: base.image.colors_used })).into_iter().collect())
    }
}
