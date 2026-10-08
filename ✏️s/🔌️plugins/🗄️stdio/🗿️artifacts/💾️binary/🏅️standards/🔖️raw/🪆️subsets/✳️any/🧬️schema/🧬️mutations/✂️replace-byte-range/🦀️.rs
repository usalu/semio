//! ✂️ `replace-byte-range` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(semio_framework_dsl_record_derive::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(semio_framework_dsl_record_derive::DslEnum)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record. Leaf, wire tag, DSL keyword, grammars and
//! catalog all speak one name: `replace-byte-range`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "replace-byte-range")]
pub struct ReplaceByteRange {
    pub offset: usize,
    pub remove_len: usize,
    #[dsl(base64)]
    pub insert: Vec<u8>,
}

impl protocol::MutationKind<BinarySnapshot, BinaryMutation> for ReplaceByteRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "byte-range", kind: "replace-byte-range", record: "ReplaceByteRange" };

    fn diff(&self, base: &BinarySnapshot) -> protocol::MutationOutcome<<BinaryMutation as Mutation<BinarySnapshot>>::Diff> {
        let Self { offset, remove_len, insert } = self;
        protocol::MutationOutcome::new(BinaryDiff { splices: vec![ByteSplice { offset: *offset, remove_len: *remove_len, insert: insert.clone() }] })
    }
    fn inverse(&self, base: &BinarySnapshot) -> Result<Vec<BinaryMutation>, semio_framework_value::ValueError> {
        let Self { offset, remove_len, insert } = self;
        Ok({
            let start = (*offset).min(base.bytes.len());
            let end = (*offset + *remove_len).min(base.bytes.len());
            let removed_bytes = base.bytes[start..end].to_vec();
            vec![BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: start, remove_len: insert.len(), insert: removed_bytes })]
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace byte range", "Bytebereich ersetzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
