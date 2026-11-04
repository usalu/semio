//! 📤️ Remove palette entry: an indexed image's palette edit with exact inverse; every sample keeps pointing at the same color.

use crate::schema::diff::BmpDiff;
use crate::schema::mutations::BmpMutation;
use crate::BmpSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemovePaletteEntry {
    pub index: u32,
    pub colors_used: u32,
}

impl protocol::MutationKind<BmpSnapshot, BmpMutation> for RemovePaletteEntry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "palette-entry", kind: "remove-palette-entry", record: "RemovePaletteEntry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove palette entry", "Paletteneintrag entfernen")
    }

    fn target(&self) -> Vec<String> {
        vec![format!("palette:{}", self.index)]
    }
}

impl protocol::ColdMutationKind<BmpSnapshot, BmpMutation> for RemovePaletteEntry {
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        match crate::schema::operations::palette_entry_removed(&base.image, self.index as usize, self.colors_used) {
            Ok(image) => protocol::MutationOutcome::new(BmpDiff { image: Some(image), ..BmpDiff::default() }),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["palette-entry"]),
        }
    }
    fn inverse(&self, base: &BmpSnapshot) -> Result<Vec<BmpMutation>, semio_framework_value::ValueError> {
        let removed = crate::schema::operations::palette_entry_removed(&base.image, self.index as usize, self.colors_used).ok();
        Ok(removed.and_then(|_| base.image.palette.get(self.index as usize)).map(|entry| BmpMutation::InsertPaletteEntry(crate::schema::mutations::InsertPaletteEntry { index: self.index, red: entry.r, green: entry.g, blue: entry.b, reserved: entry.reserved, colors_used: base.image.colors_used })).into_iter().collect())
    }
}
