//! 🧊 `set-text-box-blocks` — authored as its own mutation leaf; its diff and inverse live in `🔺️diff` and `↩️inverse`.
//!
//! 🧭️ `SEMANTICS.kind` — and this leaf's folder — is `set-text-box-blocks`:
//! `dsl::Mutations`' derive asserts `SEMANTICS.kind == to_kebab("SetTextBoxBlocks")`, and
//! `to_kebab` splits before every uppercase letter that follows a lowercase one, so `TextBox` ->
//! `text-box` (verified against the already-migrated `svg` baseline's `SetViewBox` ->
//! `set-view-box`). The op-text/binary keyword (`print_op`/`parse_op`/`📡️.protocol.semio` records, this
//! artifact's grammar files and the committed test fixtures) shares the same canonical
//! `set-text-box-blocks` spelling.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetTextBoxBlocks {
    pub slide_index: usize,
    pub shape_index: usize,
    pub blocks: Vec<DocBlock>,
}

impl protocol::MutationKind<SemioPresentationSnapshot, SemioPresentationMutation> for SetTextBoxBlocks {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "text-box-blocks", kind: "set-text-box-blocks", record: "SetTextBoxBlocks" };

    fn diff(&self, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<<SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::Diff> {
        diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    Ok({
        inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set text box blocks", "Blöcke des Textfelds setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload

//#region 🪢️TaxonomyMounts
#[path = "🔺️diff/🦀️.rs"]
mod diff;
#[path = "↩️inverse/🦀️.rs"]
mod inverse;
//#endregion 🪢️TaxonomyMounts
