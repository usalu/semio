//! 📸️️ `set-snapshot` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Facets
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
#[path = "🦠️mutation/🦀️.rs"]
pub mod mutation;
//#endregion 🔖️Facets
//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSnapshot {
    pub(crate) snapshot: SemioImageSnapshot,
}

impl protocol::MutationKind<SemioImageSnapshot, SemioImageMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &SemioImageSnapshot) -> protocol::MutationOutcome<<SemioImageMutation as Mutation<SemioImageSnapshot>>::Diff> {
        agg_diff(&SemioImageMutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
        agg_inverse(&SemioImageMutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-snapshot", "Momentaufnahme setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
