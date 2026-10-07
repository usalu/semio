//! 🪆️ Explicit typed optional values retain clearing independently of field omission.
use crate::*;
use crate::native_encoding::{FieldProjectionView as View,projection_path_error};

fn specification<T:DslField>()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"value",T::shape()).optional()])}
fn specification_controlled<T:DslField,C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;let mut fields=control.allocate_vec(1)?;fields.push(producer::field(0,"value",T::shape_controlled(control)?,control)?.optional());control.step()?;producer::record(None,RecordLayout::Inline,fields,control)})}
fn decoding_spec<T:DslField>(control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{specification_controlled::<T,_>(control)}
fn encoding_spec<T:DslField>(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{specification_controlled::<T,_>(control)}
fn producer<T:DslField>()->RecordSpecProducer{RecordSpecProducer{ordinary:specification::<T>,decoding:decoding_spec::<T>,encoding:encoding_spec::<T>}}
fn borrowed_spec<T:BorrowedDslField>()->BorrowedRecordSpec{<Option<T> as BorrowedDslRecord>::RECORD}
fn borrowed_shape<T:BorrowedDslField>()->BorrowedShape{BorrowedShape::Record(borrowed_spec::<T>)}

impl<T:BorrowedDslField> BorrowedDslRecord for Option<T>{const RECORD:BorrowedRecordSpec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&[BorrowedFieldSpec{id:0,key:"value",position:None,shape:T::SHAPE,optional:true,flatten:false,defines:None,is_call_name:false}]};}
impl<T:BorrowedDslField> BorrowedDslField for Option<T>{const SHAPE:BorrowedShape=BorrowedShape::Block(borrowed_shape::<T>);}

/// 📦️ A braced optional-value record preserves exact inner metadata and nullable delta intent.
impl<T:DslField> DslField for Option<T>{
 fn shape()->Shape{Shape::Block(Box::new(Shape::Record(producer::<T>())))}
 fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Block(producer::boxed(Shape::Record(producer::<T>()),control)?))}
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{
  if path.is_empty(){return Ok(View::Block)}if path[0]!=0{return Err(projection_path_error())}if path.len()==1{return Ok(View::Record(&[0]))}if path[1]!=0{return Err(projection_path_error())}
  match self{Some(value)=>value.projection_view(&path[2..]),None if path.len()==2=>Ok(View::Absent),None=>Err(projection_path_error())}
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path.len()<2||path[0]!=0||path[1]!=0{return Err(projection_path_error())}self.as_ref().ok_or_else(projection_path_error)?.projection_key(&path[2..],index)}
 fn to_value(&self)->FieldValue{let mut record=RecordValue::default();if let Some(value)=self{record.fields.insert(0,value.to_value());}FieldValue::Block(Box::new(FieldValue::Record(record)))}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.charge(std::mem::size_of::<FieldValue>())?;self.to_record_controlled(control).map(|record|FieldValue::Block(Box::new(FieldValue::Record(record))))}
 fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;let mut fields=RecordFields::from_empty_slots(control.allocate_vec(1)?);if let Some(value)=self{fields.insert(0,value.to_value_controlled(control)?);}Ok(RecordValue{fields})})}
 fn from_value(value:&FieldValue)->Result<Self,String>{let FieldValue::Block(inner)=value else{return Err("expected optional value block".into())};let FieldValue::Record(record)=inner.as_ref()else{return Err("expected optional value record".into())};if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return Err("unknown optional value field".into())}match record.get(0){None|Some(FieldValue::Absent)=>Ok(None),Some(value)=>T::from_value(value).map(Some)}}
 fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.step()?;let FieldValue::Block(inner)=value else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected optional value block"))};let FieldValue::Record(record)=inner.as_ref()else{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected optional value record"))};self_from_record::<T>(record,control)}
 fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{self_from_record::<T>(record,control)}
 fn retire_decoded(self){if let Some(value)=self{T::retire_decoded(value);}}
}
fn self_from_record<T:DslField>(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Option<T>,ValueError>{control.step()?;if record.fields.len()>1||record.fields.keys().any(|id|*id!=0){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown optional value field"))}match record.get(0){None|Some(FieldValue::Absent)=>Ok(None),Some(value)=>control.scoped_stage(|control|T::from_value_controlled(value,control)).map(Some)}}
