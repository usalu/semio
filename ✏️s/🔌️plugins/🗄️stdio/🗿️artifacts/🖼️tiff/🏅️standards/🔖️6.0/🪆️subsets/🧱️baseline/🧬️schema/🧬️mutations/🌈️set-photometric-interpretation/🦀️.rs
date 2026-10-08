//! ⚙️ `set-photometric-interpretation` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetPhotometricInterpretation {
    pub photometric: u16,
}

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for SetPhotometricInterpretation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "photometric-interpretation", kind: "set-photometric-interpretation", record: "SetPhotometricInterpretation" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as Mutation<TiffSnapshot>>::Diff> {
        set_first_page_shorts(base, TAG_PHOTOMETRIC, vec![self.photometric])
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffBaselineMutation>, semio_framework_value::ValueError> {
        Ok(match first_page_shorts(base, TAG_PHOTOMETRIC) {
            Some([old]) if *old != self.photometric => vec![TiffBaselineMutation::SetPhotometricInterpretation(Self { photometric: *old })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set photometric interpretation", "Photometrische Interpretation setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
