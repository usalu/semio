//! 📝️ Text transport for owned chart edits.
use crate::{ChartDiff,ChartEdit};
use semio_framework_value::{DslValue,FromValue,ToValue};
semio_framework_value_derive::value_codec!{struct ChartDiff{pub edits:Vec<ChartEdit>}}
impl ToValue for ChartEdit{
 fn to_value(&self)->DslValue{let mut fields=vec![("path".into(),self.path.to_value())];if let Some(value)=&self.before{fields.push(("before".into(),value.clone()));}if let Some(value)=&self.after{fields.push(("after".into(),value.clone()));}DslValue::object(fields)}
}
impl FromValue for ChartEdit{
 fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{
  let DslValue::Object(fields)=value else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart edit must be an object"));};
  let mut path=None;let mut before=None;let mut after=None;
  for(key,value)in fields{match key.as_str(){"path"=>path=Some(Vec::<String>::from_value(value)?),"before"=>before=Some(value),"after"=>after=Some(value),_=>return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("unknown chart edit field {key}"))),}}
  Ok(Self{path:path.ok_or_else(||semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"chart edit path is required"))?,before,after})
 }
}
impl protocol::DiffText for ChartDiff {
    fn print_diff(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_diff(text: &str) -> Result<Self,semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))
    }
}
