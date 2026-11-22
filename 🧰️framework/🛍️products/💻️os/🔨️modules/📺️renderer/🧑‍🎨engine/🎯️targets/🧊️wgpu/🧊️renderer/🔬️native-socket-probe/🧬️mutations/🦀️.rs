//! 🧬️ The probe's mutation roster: one direct leaf over the raw probe text.
#[path = "📝️set-probe/🦀️.rs"]
pub mod set_probe;
pub use crate::native_socket_probe::mutations::set_probe::SetProbe;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value_derive::RetireOwned, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = crate::native_socket_probe::NativeSocketProbeSnapshot, diff = crate::native_socket_probe::NativeSocketProbeDiff, schema = "native.socket-grant.probe/v1", reference = crate::native_socket_probe::NativeSocketProbeSnapshot, reference_diff = crate::native_socket_probe::NativeSocketProbeDiff)]
pub enum NativeSocketProbeMutation {
    SetProbe(SetProbe),
}

dsl::original_ordinal_operation_receiving!(NativeSocketProbeMutation);

impl store::os_spr::OpText for NativeSocketProbeMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        for (keyword, spec) in <Self as semio_framework_dsl_record::DslVariants>::variants() {
            let prefix = format!("{keyword} ");
            if line == keyword || line.starts_with(&prefix) {
                let body = line[keyword.len()..].trim_start();
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(&keyword, &semio_framework_dsl_record::parse(body, &(spec.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown operation line '{line}'"), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let spec = <Self as semio_framework_dsl_record::DslVariants>::variants().into_iter().find(|(candidate, _)| candidate == &keyword).expect("declared variant").1;
        format!("{keyword} {}", semio_framework_dsl_record::print(&record, &(spec.ordinary)(), semio_framework_dsl_record::JoinMode::Inline))
    }
}

impl store::os_spr::OpBinary for NativeSocketProbeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, store::os_spr::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, store::os_spr::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
