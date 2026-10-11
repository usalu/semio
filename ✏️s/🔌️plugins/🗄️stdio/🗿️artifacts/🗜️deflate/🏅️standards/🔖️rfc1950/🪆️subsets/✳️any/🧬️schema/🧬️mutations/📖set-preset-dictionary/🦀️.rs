//! 📖️ `set-preset-dictionary` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
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
#[dsl(keyword = "set-preset-dictionary")]
pub struct SetPresetDictionary {
    pub dict_id: Option<u32>,
}

impl protocol::MutationKind<DeflateSnapshot, DeflateMutation> for SetPresetDictionary {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "preset-dictionary", kind: "set-preset-dictionary", record: "SetPresetDictionary" };

    fn diff(&self, base: &DeflateSnapshot) -> protocol::MutationOutcome<<DeflateMutation as Mutation<DeflateSnapshot>>::Diff> {
        let Self { dict_id } = self;
        protocol::MutationOutcome::new( diff_set_preset_dictionary(*dict_id) )
    }
    fn inverse(&self, base: &DeflateSnapshot) -> Result<Vec<DeflateMutation>, semio_framework_value::ValueError> {
        Ok({
            vec![DeflateMutation::SetPresetDictionary(set_preset_dictionary::SetPresetDictionary { dict_id: base.dict_id })]
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set preset dictionary", "Voreingestelltes Wörterbuch setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
