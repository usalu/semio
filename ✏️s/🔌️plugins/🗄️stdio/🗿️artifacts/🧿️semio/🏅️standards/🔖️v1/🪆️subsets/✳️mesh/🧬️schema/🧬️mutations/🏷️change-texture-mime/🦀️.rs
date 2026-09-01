//! 🏷️ `change-texture-mime` — sets a texture's mime type. Decomposed from the old bundled `SetTextureBytes{mime,bytes}`: `mime`/`bytes` are two independent top-level fields on `SemioTexture` (derivation-rules.md rule 2: `change-<field>` per remaining scalar, `replace-<payload>` per large structured field).

use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeTextureMime {
    pub id: String,
    pub new_mime: String,
}

impl protocol::MutationKind<SemioMeshSnapshot, SemioMeshMutation> for ChangeTextureMime {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "texture-mime", kind: "change-texture-mime", record: "ChangedTextureMime" };

    fn diff(&self, base: &SemioMeshSnapshot) -> protocol::MutationOutcome<<SemioMeshMutation as protocol::Mutation<SemioMeshSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioMeshSnapshot) -> Vec<SemioMeshMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change texture \"{}\" mime type", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Payload
