//! 🎨️ `set-mtllib` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record (`✨️derive/🦀️.rs`'s
//! `dsl_variants_codegen`, "single-field tuple variant" branch), keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-mtllib")]
pub struct SetMtllib {
    pub mtllib: Option<String>,
}

impl protocol::MutationKind<ObjSnapshot, ObjMutation> for SetMtllib {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "mtllib", kind: "set-mtllib", record: "SetMtllib" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set mtllib", "mtllib setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<ObjSnapshot, ObjMutation> for SetMtllib {
    fn diff(&self, _base: &ObjSnapshot) -> protocol::MutationOutcome<<ObjMutation as protocol::ColdMutation<ObjSnapshot>>::ReferenceDiff> {
        let Self { mtllib } = self;
        protocol::MutationOutcome::new(diff_set_mtllib(mtllib.clone()))
    }
    fn inverse(&self, base: &ObjSnapshot) -> Result<Vec<ObjMutation>, semio_framework_value::ValueError> {
        Ok(vec![ObjMutation::SetMtllib(SetMtllib { mtllib: base.mtllib.clone() })])
    }
}
//#endregion 🔖️Payload
