//! 🧩️ `set-bits-per-sample` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBitsPerSample {
    pub bits: Vec<u16>,
}

impl protocol::MutationKind<TiffSnapshot, TiffBaselineMutation> for SetBitsPerSample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "bits-per-sample", kind: "set-bits-per-sample", record: "SetBitsPerSample" };

    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<<TiffBaselineMutation as Mutation<TiffSnapshot>>::Diff> {
        set_first_page_shorts(base, TAG_BITS_PER_SAMPLE, self.bits.clone())
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffBaselineMutation>, semio_framework_value::ValueError> {
        Ok(match first_page_shorts(base, TAG_BITS_PER_SAMPLE) {
            Some(old) if old != self.bits.as_slice() => vec![TiffBaselineMutation::SetBitsPerSample(Self { bits: old.to_vec() })],
            _ => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set bits per sample", "Bits pro Abtastwert setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
