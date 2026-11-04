//! 🌈️ Set palette entry: an indexed image's palette edit with exact inverse; every sample keeps pointing at the same color.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetPaletteEntry {
    pub index: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub reserved: u8,
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for SetPaletteEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "palette-entry", kind: "set-palette-entry", record: "SetPaletteEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set palette entry", "Paletteneintrag setzen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("palette:{}", self.index)]
    }
}

impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for SetPaletteEntry {
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        if base.image.palette.get(self.index as usize) == Some(&crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }) {
            return protocol::MutationOutcome::new(BmpDiff::default());
        }
        match crate::schema::operations::palette_entry_set(&base.image, self.index as usize, crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }) {
            Ok(image) => protocol::MutationOutcome::new(BmpDiff { image: Some(image), ..BmpDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        Ok(base.image.palette.get(self.index as usize).filter(|current| **current != crate::schema::snapshot::BmpPaletteEntry { b: self.blue, g: self.green, r: self.red, reserved: self.reserved }).map(|current| BmpMutation::SetPaletteEntry(Self { index: self.index, red: current.r, green: current.g, blue: current.b, reserved: current.reserved })).into_iter().collect())
    }
}
