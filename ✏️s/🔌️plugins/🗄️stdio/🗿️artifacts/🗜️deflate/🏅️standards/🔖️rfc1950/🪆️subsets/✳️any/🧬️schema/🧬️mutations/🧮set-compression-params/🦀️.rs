//! 🧮️ `set-compression-params` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//! `#[derive(dsl::DslRecord)]` gives this leaf its own `DslField` impl with the SAME field spec
//! `record_codegen` built when these fields lived inline in the enum variant — the aggregate's
//! tuple variant is a single-field newtype, so `#[derive(dsl::DslOps)]`'s `DslVariants` derive
//! delegates straight through to this leaf's own record, keeping the committed mutations
//! grammar/protocol facets byte-identical to before this leaf existed.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_dsl_record_derive::DslRecord)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-compression-params")]
pub struct SetCompressionParams {
    pub method: u8,
    pub window_bits: u8,
    pub level_hint: DeflateLevelHint,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetCompressionParams {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "compression-params", kind: "set-compression-params", record: "SetCompressionParams" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as Mutation<DeflateSnapshot>>::Diff> {
        let Self { method, window_bits, level_hint } = self;
        if *method > 15 || *window_bits > 15 {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::Invariant, "deflate: the method and window size are 4-bit header fields", ["compression-params"]);
        }
        protocol::MutationOutcome::new( diff_set_compression_params(*method, *window_bits, *level_hint) )
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Result<Vec<DeflateMutation>, semio_framework_value::ValueError> {
        Ok(vec![DeflateMutation::SetCompressionParams(set_compression_params::SetCompressionParams { method: base.compression_method, window_bits: base.window_bits, level_hint: base.compression_level_hint })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set compression params", "Kompressionsparameter setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
