//! 🕹️ `set-frame-user-input` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-user-input")]
pub struct SetFrameUserInput {
    pub(crate) index: usize,
    pub(crate) user_input: bool,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameUserInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-user-input", kind: "set-frame-user-input", record: "SetFrameUserInput" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::SetFrameUserInput(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::SetFrameUserInput(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Set frame user input", "Benutzereingabe-Kennung des Einzelbilds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
