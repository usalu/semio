//! 🚪️ IDML native text, pack and operation representations over intrinsic package fields.
use semio_framework_os_kernel as protocol;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
use semio_framework_pack::{snapshot::{ArtifactDsl, ArtifactPack}, PackError};
use semio_framework_diagnostic::{TextError, TextSpan};
use crate::{IdmlSnapshot, IdmlDiff, IdmlMutation};

fn text_error(error: impl std::fmt::Display) -> TextError { TextError::new(ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)) }
fn invalid(message: &str) -> PackError { PackError::from(ValueError::new(ValueRefusalKind::InvalidValue, message)) }
fn parse<T: FromValue>(source: &str) -> Result<T, TextError> {
    semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(text_error)
}
fn envelope(component: protocol::semio_format::Component) -> protocol::semio_format::SemioEnvelope {
    protocol::semio_format::SemioEnvelope::from_envelope_id(crate::DOCUMENT_SCHEMA, component, 1).expect("valid authored IDML envelope identity")
}

/// 🧬️ The exact field specification used by the intrinsic value pack codec below.
pub fn native_record_spec() -> semio_framework_dsl_record::RecordSpec {
    semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, vec![semio_framework_dsl_record::FieldSpec::new(1, "value", semio_framework_dsl_record::Shape::Value)])
}
/// 🔑️ Canonical first-party schema identity of the actual encoded record.
pub fn schema_hash() -> [u8; 32] { semio_framework_pack::record::schema_hash(&native_record_spec()) }

impl ArtifactDsl for IdmlSnapshot {
    const EXTENSION: &'static str = "idml";
    fn envelope_id() -> &'static str { crate::DOCUMENT_SCHEMA }
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match protocol::semio_format::split_text_preamble(text) {
            Ok((identity, body)) => {
                if !identity.matches_identity(crate::DOCUMENT_SCHEMA, protocol::semio_format::Component::Dsl, 1) { return Err(text_error("stdio.idml.text-identity")); }
                body
            }
            Err(_) => text,
        };
        parse(body)
    }
    fn print_dsl(&self) -> String {
        protocol::semio_format::wrap_text(&envelope(protocol::semio_format::Component::Dsl), &semio_framework_pack_json::to_json_string(self))
    }
}
impl ArtifactPack for IdmlSnapshot {
    fn encode_pack_with(&self, options: &semio_framework_pack::record::EncodeOptions) -> Result<Vec<u8>, PackError> {
        self.package().map_err(PackError::from)?;
        let inner = self.to_value().encode_pack_with(options)?;
        Ok(protocol::semio_format::wrap_binary(&envelope(protocol::semio_format::Component::Pack), &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &semio_framework_pack::record::DecodeOptions) -> Result<Self, PackError> {
        let (identity, inner) = protocol::semio_format::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !identity.matches_identity(crate::DOCUMENT_SCHEMA, protocol::semio_format::Component::Pack, 1) { return Err(invalid("stdio.idml.pack-identity")); }
        Self::from_value(DslValue::decode_pack_with(&inner, options)?).map_err(PackError::from)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { Some(native_record_spec()) }
}
impl protocol::os_store::serialization::ArtifactStoreSnapshot for IdmlSnapshot {}

impl protocol::OpText for IdmlMutation {
    fn print_op(&self) -> String { format!("{} {}", protocol::Mutation::descriptor(self).semantic_kind, semio_framework_pack_json::to_json_string(&protocol::Mutation::payload_value(self))) }
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let (kind, payload) = line.trim().split_once(' ').ok_or_else(|| text_error("stdio.idml.operation-text"))?;
        <Self as protocol::Mutation<IdmlSnapshot>>::from_payload_value(kind, parse(payload)?).map_err(text_error)
    }
}
impl protocol::OpBinary for IdmlMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut bytes = vec![1, match self { Self::ReplacePackage { .. } => 1, Self::ReplacePart { .. } => 2 }];
        bytes.extend(protocol::Mutation::payload_value(self).encode_pack());
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: &str| protocol::ProtocolError::Malformed { what: "stdio.idml.operation", offset: 0, detail: detail.into() };
        let [1, tag, body @ ..] = bytes else { return Err(malformed("unsupported or truncated IDML operation version")); };
        let kind = match tag { 1 => "replace-package", 2 => "replace-part", _ => return Err(malformed("unknown IDML mutation tag")) };
        let payload = DslValue::decode_pack(body).map_err(protocol::ProtocolError::from)?;
        <Self as protocol::Mutation<IdmlSnapshot>>::from_payload_value(kind, payload).map_err(|error| malformed(&error.to_string()))
    }
}
impl protocol::DiffText for IdmlDiff {
    fn print_diff(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_diff(line: &str) -> Result<Self, TextError> { parse(line) }
}
impl protocol::DiffBinary for IdmlDiff {
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(self.to_value().encode_pack()) }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Self::from_value(DslValue::decode_pack(bytes).map_err(protocol::ProtocolError::from)?).map_err(|error| protocol::ProtocolError::Malformed { what: "stdio.idml.diff", offset: 0, detail: error.to_string() })
    }
}

/// 🛂 Borrowed package admission with funded numeric metadata pages.
#[path="🛂️admission/🦀️.rs"]
pub mod admission;
