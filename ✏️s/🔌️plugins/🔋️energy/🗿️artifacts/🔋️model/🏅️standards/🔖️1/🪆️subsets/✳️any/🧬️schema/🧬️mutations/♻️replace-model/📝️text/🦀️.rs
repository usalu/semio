//! 📝️ Direct `replace-model` text payload codec and aggregate wire bridge.

use super::super::EnergyModelMutation;
use super::ReplaceModel;
use protocol::OpText;

/// 🏷️ Stable text opcode for `ReplaceModel`.
pub const TEXT_OPCODE: &str = "replace-model";

//#region 🔖️WireMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum EnergyModelMutationDsl {
    ReplaceModel { new_model_json: String },
}

impl OpText for EnergyModelMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(candidate, _)| candidate == &keyword).map(|(_, spec)| *spec).expect("variant spec must exist for its own keyword");
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

fn to_dsl(mutation: &EnergyModelMutation) -> EnergyModelMutationDsl {
    match mutation {
        EnergyModelMutation::ReplaceModel(payload) => EnergyModelMutationDsl::ReplaceModel { new_model_json: payload.new_model_json.clone() },
    }
}

fn from_dsl(mutation: EnergyModelMutationDsl) -> EnergyModelMutation {
    match mutation {
        EnergyModelMutationDsl::ReplaceModel { new_model_json } => EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json }),
    }
}

impl OpText for EnergyModelMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(from_dsl(<EnergyModelMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <EnergyModelMutationDsl as OpText>::print_op(&to_dsl(self))
    }
}

impl protocol::OpBinary for EnergyModelMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(from_dsl(EnergyModelMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️WireMirror

//#region 🧪️RoundTrip
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn text_and_binary_wire_forms_round_trip() {
        let operation = EnergyModelMutation::ReplaceModel(ReplaceModel { new_model_json: r#"{"name":"demo"}"#.to_string() });
        store::os_store::test_support::assert_op_line_round_trip(&operation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
//#endregion 🧪️RoundTrip
