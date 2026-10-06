//! 🎮️ Canonical typed chart operations share the authored Record fields in Text and binary.
use crate::ChangeChartValue;
impl protocol::OpText for ChangeChartValue{
 fn print_op(&self)->String{semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline)}
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(line,&Self::__dsl_spec(),&Default::default())?)}
}
