//! 📄️ `set-snapshot` — authored as its own mutation leaf. The aggregate's original `diff`/
//! `inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.

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
#[value(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub(crate) snapshot: SemioAnimationSnapshot,
}

impl protocol::MutationKind<SemioAnimationSnapshot, SemioAnimationMutation> for SetSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "snapshot", kind: "set-snapshot", record: "SetSnapshot" };

    fn diff(&self, base: &SemioAnimationSnapshot) -> protocol::MutationOutcome<<SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::Diff> {
        agg_diff(&SemioAnimationMutation::SetSnapshot(self.clone()), base)
    }
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Vec<SemioAnimationMutation> {
        agg_inverse(&SemioAnimationMutation::SetSnapshot(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-snapshot", "Momentaufnahme setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
