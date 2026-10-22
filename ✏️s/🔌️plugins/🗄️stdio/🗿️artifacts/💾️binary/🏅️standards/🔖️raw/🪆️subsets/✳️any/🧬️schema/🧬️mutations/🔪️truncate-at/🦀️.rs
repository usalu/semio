//! 🔪️ `truncate-at` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(semio_framework_dsl_record_derive::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when this field lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(semio_framework_dsl_record_derive::DslEnum)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "truncate-at")]
pub struct TruncateAt {
    pub offset: usize,
}

impl protocol::MutationKind<BinarySnapshot, BinaryMutation> for TruncateAt {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "trailing-bytes", kind: "truncate-at", record: "TruncateAt" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Truncate at position", "An Position kürzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<BinarySnapshot, BinaryMutation> for TruncateAt {
    fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<<BinaryMutation as protocol::ColdMutation<BinarySnapshot>>::ReferenceDiff> {
        let Self { offset } = self;
        protocol::MutationOutcome::new({
            if *offset >= base.bytes.len() {
                BinaryDiff::default()
            } else {
                BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: base.bytes.len() - offset, insert: vec![] }] }
            }
        })
    }
    fn inverse(&self, base: &BinarySnapshot) -> Result<Vec<BinaryMutation>, semio_framework_value::ValueError> {
        let Self { offset } = self;
        Ok({
            {
                if *offset >= base.bytes.len() {
                    Vec::new()
                } else {
                    vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: *offset, remove_len: 0, insert: base.bytes[*offset..].to_vec() })]
                }
            }
        })
    }
}
//#endregion 🔖️Payload
