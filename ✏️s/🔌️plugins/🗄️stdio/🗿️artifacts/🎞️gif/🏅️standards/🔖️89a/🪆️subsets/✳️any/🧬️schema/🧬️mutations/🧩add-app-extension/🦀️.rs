//! 🧩️ `add-app-extension` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct AddAppExtension {
    pub(crate) index: usize,
    #[dsl(block)]
    pub(crate) extension: GifAppExtension,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for AddAppExtension {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "app-extension", kind: "add-app-extension", record: "AddAppExtension" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as protocol::Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::AddAppExtension(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::AddAppExtension(self.clone()), base)
    }
    fn label(&self) -> String {
        "add-app-extension".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
