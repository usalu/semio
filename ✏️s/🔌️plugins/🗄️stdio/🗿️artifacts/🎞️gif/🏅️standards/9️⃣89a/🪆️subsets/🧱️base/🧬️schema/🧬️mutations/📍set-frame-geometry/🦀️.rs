//! 📍️ `set-frame-geometry` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-frame-geometry")]
pub struct SetFrameGeometry {
    pub(crate) index: usize,
    pub(crate) left: u32,
    pub(crate) top: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl protocol::MutationKind<GifSnapshot, GifMutation> for SetFrameGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "frame-geometry", kind: "set-frame-geometry", record: "SetFrameGeometry" };

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<<GifMutation as Mutation<GifSnapshot>>::Diff> {
        agg_diff(&GifMutation::SetFrameGeometry(self.clone()), base)
    }
    fn inverse(&self, base: &GifSnapshot) -> Vec<GifMutation> {
        agg_inverse(&GifMutation::SetFrameGeometry(self.clone()), base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("set-frame-geometry", "Rahmengeometrie setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
