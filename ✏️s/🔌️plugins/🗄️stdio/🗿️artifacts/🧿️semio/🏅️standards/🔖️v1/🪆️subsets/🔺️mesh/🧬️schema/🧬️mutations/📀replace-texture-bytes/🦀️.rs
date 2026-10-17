//! 📀 `replace-texture-bytes` — whole-value swap of a texture's raw byte payload. Raw image bytes are the \"large\" swapped payload (matches `replace-primitive-geometry`'s exact rename rationale), never edited byte-by-byte from outside, so `replace`, not `change`/`set`.

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceTextureBytes {
    pub id: String,
    pub new_bytes: Vec<u8>,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for ReplaceTextureBytes
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<ReplaceTextureBytes>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "texture-bytes", kind: "replace-texture-bytes", record: "ReplacedTextureBytes" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace texture \"{}\" bytes", self.id), &format!("Bytes von Textur \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
