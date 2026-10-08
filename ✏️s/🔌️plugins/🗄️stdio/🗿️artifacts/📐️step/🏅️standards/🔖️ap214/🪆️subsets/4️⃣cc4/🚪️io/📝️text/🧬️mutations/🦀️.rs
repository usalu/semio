//! 📝️ Canonical JSON text for the owned CC4 mutation vocabulary.

use crate::standards::v_ap214::subsets::cc4::schema::mutations::StepCc4Mutation;

impl protocol::OpText for StepCc4Mutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = semio_framework_pack_json::parse(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        <Self as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
