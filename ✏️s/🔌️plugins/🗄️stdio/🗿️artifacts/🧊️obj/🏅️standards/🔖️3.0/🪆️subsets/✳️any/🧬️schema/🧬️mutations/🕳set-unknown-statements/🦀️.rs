//! 🕳️ `set-unknown-statements` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate
//! value and delegates, so the semantics are preserved by construction rather than re-derived.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record (`✨️derive/🦀️component.rs`'s
//! `dsl_variants_codegen`, "single-field tuple variant" branch), keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, dsl::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-unknown-statements")]
pub struct SetUnknownStatements {
    pub unknown_statements: Vec<ObjUnknownStatement>,
}

impl protocol::MutationKind<ObjSnapshot, ObjMutation> for SetUnknownStatements {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "unknown-statements", kind: "set-unknown-statements", record: "SetUnknownStatements" };

    fn diff(&self, base: &ObjSnapshot) -> protocol::MutationOutcome<<ObjMutation as protocol::Mutation<ObjSnapshot>>::Diff> {
        agg_diff(&ObjMutation::SetUnknownStatements(self.clone()), base)
    }
    fn inverse(&self, base: &ObjSnapshot) -> Vec<ObjMutation> {
        agg_inverse(&ObjMutation::SetUnknownStatements(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-unknown-statements".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
