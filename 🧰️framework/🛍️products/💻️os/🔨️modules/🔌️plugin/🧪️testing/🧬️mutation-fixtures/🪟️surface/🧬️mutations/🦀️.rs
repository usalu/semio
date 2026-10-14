#[path = "📝️set-surface-count/🦀️.rs"]
pub mod set_surface_count;
pub(crate) use set_surface_count::SetSurfaceCount;
#[derive(semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=super::SurfaceSnapshot,diff=super::SurfaceDiff,schema="plugin.testkit.surface")]
pub(crate) enum SurfaceMutation {
    SetSurfaceCount(SetSurfaceCount),
}
impl protocol::OpText for SurfaceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(SetSurfaceCount::parse_op(line)?.into())
    }
    fn print_op(&self) -> String {
        match self {
            Self::SetSurfaceCount(value) => value.print_op(),
        }
    }
}
impl protocol::OpBinary for SurfaceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match self {
            Self::SetSurfaceCount(value) => value.encode_op(),
        }
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(SetSurfaceCount::decode_op(bytes)?.into())
    }
}
