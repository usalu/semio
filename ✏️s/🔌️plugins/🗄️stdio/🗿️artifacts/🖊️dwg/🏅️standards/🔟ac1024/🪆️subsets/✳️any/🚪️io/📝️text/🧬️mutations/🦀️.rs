//! 📝️ Text representation codec surface for `stdio.dwg` (mutations).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

use crate::standards::v_ac1024::subsets::any::schema::mutations::DwgMutation;
impl protocol::OpText for DwgMutation {
 fn print_op(&self)->String{semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self)))}
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{let parsed=semio_framework_pack_json::parse(line,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(),semio_framework_diagnostic::TextSpan::at(1,1)))?;semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))}
}
