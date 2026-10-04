//! 🪆️ Unboxed actual tagged fields and literal flattened node ownership.
use super::*;
use semio_framework_value::{DecodedValue,FromValue,ToValue,NativeDecodeControl,NativeEncodeControl,ValueError,ValueRefusalKind};
macro_rules! tagged_field{
 ($($name:ty),+)=>{$(
  impl semio_framework_dsl_record::DslField for $name{
   fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Statements(<Self as semio_framework_dsl_record::DslVariants>::variants())}
   fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,ValueError>{<Self as semio_framework_dsl_record::DslVariants>::variants_controlled(control).map(semio_framework_dsl_record::Shape::Statements)}
   fn to_value(&self)->semio_framework_dsl_record::FieldValue{semio_framework_dsl_record::FieldValue::Statements(vec![<Self as semio_framework_dsl_record::DslVariants>::to_named_record(self)])}
   fn from_value(value:&semio_framework_dsl_record::FieldValue)->Result<Self,String>{match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=><Self as semio_framework_dsl_record::DslVariants>::from_named_record(&values[0].0,&values[0].1).map_err(|error|error.message),_=>Err("DAG tagged field requires one literal statement".into())}}
   fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{semio_framework_dsl_record::native_encoding::project_statements(std::slice::from_ref(self),control)}
   fn from_value_controlled(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;match value{semio_framework_dsl_record::FieldValue::Statements(values)if values.len()==1=>{let value=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(<Self as semio_framework_dsl_record::DslVariants>::from_named_record_controlled(&values[0].0,&values[0].1,control)?,<Self as semio_framework_dsl_record::DslVariants>::retire_decoded_variant);control.checkpoint()?;Ok(value.take())},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"DAG tagged field requires one literal statement"))}})}
   fn retire_decoded(self){<Self as semio_framework_dsl_record::DslVariants>::retire_decoded_variant(self)}
  }
 )+};
}
tagged_field!(DagNodeKind,DagPreviewContent);
pub(super) fn decode_present_port_intrinsic(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Option<DslValue>,ValueError>{DslValue::from_value_controlled(value,control).map(Some)}
pub(super) fn encode_node(value:&DagNodeSpec,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;
  let kind=value.kind.to_value_controlled(control)?.guard_encoded();
  let DslValue::Object(kind_fields)=kind.get()else{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DAG kind requires literal object"))};
  let count=9usize.checked_add(usize::from(value.operator_kind.is_some())).and_then(|count|count.checked_add(kind_fields.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DAG node field count overflow"))?;
  let mut output=DslValue::object_encoding_controlled(count,control)?;
  macro_rules! field{($key:literal,$value:expr)=>{{let value=$value.to_value_controlled(control)?;DslValue::push_encoding_controlled(output.get_mut(),$key,value,control)?;}}}
  field!("id",value.id);field!("name",value.name);field!("abbreviation",value.abbreviation);field!("icon",value.icon);
  field!("x",value.x);field!("y",value.y);field!("width",value.width);field!("height",value.height);
  if let Some(value)=&value.operator_kind{field!("operatorKind",value);}
  field!("properties",value.properties);
  let DslValue::Object(fields)=kind.take()else{unreachable!()};
  for field in fields{output.get_mut().push(field);}
  control.checkpoint()?;Ok(DslValue::Object(output.take()))
 })
}
pub(super) fn decode_node(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DagNodeSpec,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0)?;let fields=value.object_controlled(control)?;
  let mut output=DecodedValue::new(DagNodeSpec::default(),retire_node);
  macro_rules! required{($name:ident,$key:literal)=>{output.get_mut().$name=FromValue::from_value_controlled(DslValue::field_controlled(fields,$key,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,concat!("missing DAG node field ",$key)))?,control)?;};}
  macro_rules! optional{($name:ident,$key:literal)=>{if let Some(value)=DslValue::field_controlled(fields,$key,control)?{output.get_mut().$name=FromValue::from_value_controlled(value,control)?;}};}
  required!(id,"id");required!(name,"name");optional!(abbreviation,"abbreviation");optional!(icon,"icon");
  optional!(x,"x");optional!(y,"y");optional!(width,"width");optional!(height,"height");optional!(operator_kind,"operatorKind");optional!(properties,"properties");
  output.get_mut().kind=DagNodeKind::from_value_controlled(value,control)?;
  control.checkpoint()?;Ok(output.take())
 })
}
pub(super) fn retire_node(value:DagNodeSpec){<DagNodeSpec as semio_framework_dsl_record::DslField>::retire_decoded(value)}
