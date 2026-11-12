#[path = "📝️set-surface-count/🦀️.rs"]
pub mod set_surface_count;
pub use set_surface_count::SetSurfaceCount;
#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=super::SurfaceSnapshot,diff=super::SurfaceDiff,schema="plugin.testkit.surface", reference = super::SurfaceSnapshot, reference_diff = super::SurfaceDiff)]
pub enum SurfaceMutation {
    SetSurfaceCount(SetSurfaceCount),
}
impl protocol::OpText for SurfaceMutation {
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
        let body = semio_framework_dsl_record::print(&record, &(spec.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() { keyword } else { format!("{keyword} {body}") }
    }
}
impl protocol::OpBinary for SurfaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
dsl::original_ordinal_operation_receiving!(SurfaceMutation);
