//! 🔧️ EnergyModel — OpText/OpBinary codecs for `EnergyModelMutation`. Mutation apply/inverse live
//! in `🧬️mutations`; this facet only handcrafts the op wire forms.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::mutations::replace_model;
use protocol::OpText;

//#region 🔖️OpText
/// ✂️ Local DSL-only mirror of `EnergyModelMutation` — the sole `replace-model` variant flattened
/// into its own keyworded record, converted at the `store::OpText` boundary only; `EnergyModelMutation`
/// itself, and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum EnergyModelMutationDsl {
    ReplaceModel { new_model_json: String },
}

//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl OpText for EnergyModelMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for EnergyModelMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

fn energy_model_mutation_to_dsl(mutation: &EnergyModelMutation) -> EnergyModelMutationDsl {
    match mutation {
        EnergyModelMutation::ReplaceModel(payload) => EnergyModelMutationDsl::ReplaceModel { new_model_json: payload.new_model_json.clone() },
    }
}

fn energy_model_mutation_from_dsl(mutation: EnergyModelMutationDsl) -> EnergyModelMutation {
    match mutation {
        EnergyModelMutationDsl::ReplaceModel { new_model_json } => EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json }),
    }
}

impl OpText for EnergyModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(energy_model_mutation_from_dsl(<EnergyModelMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <EnergyModelMutationDsl as OpText>::print_op(&energy_model_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `EnergyModelMutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        energy_model_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(energy_model_mutation_from_dsl(EnergyModelMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_text_round_trips_replace_model() {
        store::os_store::test_support::assert_op_line_round_trip(&EnergyModelMutation::ReplaceModel(replace_model::mutation::ReplaceModel { new_model_json: r#"{"a":1}"#.to_string() }));
    }
}
//#endregion 🧪️Tests
