//! 📏️ `set-scale-and-offset` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 📏️ Sets X/Y/Z scale factors and offsets — the two header fields that jointly
//! reconstruct real-world coordinates from the on-disk integer point records — and
//! nothing else. The records keep the exact integers they carry, so every coordinate
//! is re-read under the new parameters (`coordinate = record * scale + offset`) rather
//! than held fixed while the records are silently re-quantized. Lossless and exactly
//! invertible in either direction; see
//! `../../🔺️diff/🦀️.rs::diff_set_scale_and_offset` for the reproduction that
//! settled it.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetScaleAndOffset {
    pub scale: (f64, f64, f64),
    pub offset: (f64, f64, f64),
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetScaleAndOffset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "scale-and-offset", kind: "set-scale-and-offset", record: "SetScaleAndOffset" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { scale, offset } = self;
        protocol::MutationOutcome::new(diff::diff_set_scale_and_offset(base, *scale, *offset))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok({ vec![LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (base.header.x_scale, base.header.y_scale, base.header.z_scale), offset: (base.header.x_offset, base.header.y_offset, base.header.z_offset) })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set scale and offset", "Maßstab und Versatz setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
