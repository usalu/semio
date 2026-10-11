//! 📦️ `set-payload` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when this field lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-payload")]
pub struct SetPayload {
    #[dsl(base64)]
    pub payload: Vec<u8>,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "payload", kind: "set-payload", record: "SetPayload" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as Mutation<DeflateSnapshot>>::Diff> {
        let Self { payload } = self;
        protocol::MutationOutcome::new( diff_set_payload(payload.clone()) )
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Result<Vec<DeflateMutation>, semio_framework_value::ValueError> {
        Ok(vec![DeflateMutation::SetPayload(set_payload::SetPayload { payload: base.payload.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set payload", "Nutzlast setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
