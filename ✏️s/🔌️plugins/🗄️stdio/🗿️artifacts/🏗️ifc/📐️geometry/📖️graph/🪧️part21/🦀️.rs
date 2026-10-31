use super::GeometryValue;
use semio_s_artifact_stdio_contract::part21::Part21Value;

impl GeometryValue for Part21Value {
    fn as_reference(&self) -> Option<u64> { self.as_ref_id() }
    fn as_str(&self) -> Option<&str> { Part21Value::as_str(self) }
    fn as_enum(&self) -> Option<&str> { Part21Value::as_enum(self) }
    fn as_real(&self) -> Option<f64> { Part21Value::as_real(self) }
    fn as_integer(&self) -> Option<i64> { if let Self::Int(value) = self { Some(*value) } else { None } }
    fn as_aggregate(&self) -> Option<&[Self]> { self.as_list() }
    fn as_typed(&self)->Option<(&str,&[Self])>{Part21Value::as_typed(self)}
    fn is_absent(&self) -> bool { matches!(self, Self::Unset | Self::Derived) }
    fn is_unset(&self) -> bool { matches!(self, Self::Unset) }
    fn is_derived(&self) -> bool { matches!(self, Self::Derived) }
}

