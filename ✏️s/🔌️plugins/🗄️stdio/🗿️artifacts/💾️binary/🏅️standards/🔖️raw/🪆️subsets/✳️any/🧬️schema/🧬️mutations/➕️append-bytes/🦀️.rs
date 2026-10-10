//! ➕️ `append-bytes` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(semio_framework_dsl_record_derive::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when this field lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(semio_framework_dsl_record_derive::DslEnum)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "append-bytes")]
pub struct AppendBytes {
    #[dsl(base64)]
    pub data: Vec<u8>,
}

impl protocol::MutationKind<BinarySnapshot, BinaryMutation> for AppendBytes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "append", entity: "bytes", kind: "append-bytes", record: "AppendBytes" };

    fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<<BinaryMutation as Mutation<BinarySnapshot>>::Diff> {
        let Self { data } = self;
        protocol::MutationOutcome::new(BinaryDiff { splices: vec![ByteSplice { offset: base.bytes.len(), remove_len: 0, insert: data.clone() }] })
    }
    fn inverse(&self, base: &BinarySnapshot) -> Result<Vec<BinaryMutation>, semio_framework_value::ValueError> {
        Ok({
            // ↩️ Undo an append by truncating back to the pre-append length.
            vec![BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset: base.bytes.len() })]
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Append bytes", "Bytes anhängen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
