//#region 🧬️JobTestOperationRoster
use super::{JobTestDiff, JobTestSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "➕️add-value/🦀️.rs"]
mod add_value;
pub(crate) use add_value::AddValue;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
#[mutations(snapshot=JobTestSnapshot,diff=JobTestDiff,schema="jobtest.mutation-plan.document")]
pub(crate) enum JobTestOp {
    AddValue(AddValue),
}

impl store::OpBinary for JobTestOp {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())).into())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) }.into())
    }
}
//#endregion 🧬️JobTestOperationRoster
