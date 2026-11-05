//! 📝️ I-JSON mutation text framing over the schema-owned canonical JSON value.
use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::JsonIJsonMutation;

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
const PREFIX: &str = "json-i-json-mutation ";

fn error(detail: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

impl protocol::OpText for JsonIJsonMutation {
    fn print_op(&self) -> String {
        format!("{PREFIX}{}", semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(self)))
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let payload = line.strip_prefix(PREFIX).ok_or_else(|| error("expected I-JSON mutation frame"))?;
        let value = semio_framework_pack_json::parse(payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|cause| error(cause.to_string()))?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|cause| error(cause.to_string()))
    }
}
