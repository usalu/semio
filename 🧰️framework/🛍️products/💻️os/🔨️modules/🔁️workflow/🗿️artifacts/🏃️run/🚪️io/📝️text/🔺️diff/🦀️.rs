//! 🔺️ Physical diff representation.

impl semio_framework_os_kernel::DiffText for crate::diff::RunDiff {
    fn print_diff(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_diff(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
