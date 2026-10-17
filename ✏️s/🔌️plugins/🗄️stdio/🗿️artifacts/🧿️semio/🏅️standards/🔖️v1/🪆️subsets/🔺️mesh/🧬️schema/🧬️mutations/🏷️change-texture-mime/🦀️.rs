//! 🏷️ `change-texture-mime` — sets a texture's mime type. Decomposed from the old bundled `SetTextureBytes{mime,bytes}`: `mime`/`bytes` are two independent top-level fields on `SemioTexture` (derivation-rules.md rule 2: `change-<field>` per remaining scalar, `replace-<payload>` per large structured field).

use crate::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTextureMime {
    pub id: String,
    pub new_mime: String,
}

impl<Op> protocol::MutationKind<SemioMeshSnapshot, Op> for ChangeTextureMime
where
    Op: protocol::Mutation<SemioMeshSnapshot, Diff = crate::schema::diff::SemioMeshDiff> + From<ChangeTextureMime>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "texture-mime", kind: "change-texture-mime", record: "ChangedTextureMime" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change texture \"{}\" mime type", self.id), &format!("MIME-Typ von Textur \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
