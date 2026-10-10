//! 🎮️ Canonical typed chart operations share the authored Record fields in Text and binary.
use crate::ChangeChartValue;
use semio_framework_value::{DslValue,FromValue,ToValue};
semio_framework_dsl_record_derive::record_binding!{
 #[dsl(keyword="change-chart-value")]
 struct ChangeChartValue{pub path:Vec<String>,pub value:Option<DslValue>}
}
impl ToValue for ChangeChartValue {
 fn to_value(&self)->DslValue{let mut fields=vec![("path".into(),self.path.to_value())];if let Some(value)=&self.value{fields.push(("value".into(),value.clone()));}DslValue::object(fields)}
}
impl FromValue for ChangeChartValue {
 fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{
  let DslValue::Object(fields)=value else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart mutation must be an object"));};
  let mut path=None;let mut next=None;
  for(key,value)in fields{match key.as_str(){"path"=>path=Some(Vec::<String>::from_value(value)?),"value"=>next=Some(value),_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown chart mutation field {key}"))),}}
  Ok(Self{path:path.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart mutation path is required"))?,value:next})
 }
}
impl protocol::OpText for ChangeChartValue{
 fn print_op(&self)->String{semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Inline)}
 fn parse_op(line:&str)->Result<Self,semio_framework_diagnostic::TextError>{Self::__dsl_from_record(&semio_framework_dsl_record::parse_exact(line,&Self::__dsl_spec(),&Default::default())?)}
}
