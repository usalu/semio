//! 🛫️ Explicit owned field projection under one native output control.
use super::{DslField,DslVariants,FieldValue,RecordValue,NativeEncodeControl};

/// ♻️ Retires intermediate native fields without recursive container drops.
pub fn retire_field(value:FieldValue){
    let mut pending=vec![value];
    while let Some(value)=pending.pop(){match value{
        FieldValue::Tuple(mut items)|FieldValue::List(mut items)=>pending.append(&mut items),
        FieldValue::Record(record)=>pending.extend(record.fields.into_values()),
        FieldValue::Block(value)=>pending.push(*value),
        FieldValue::Statements(items)=>for(_,record)in items{pending.extend(record.fields.into_values());},
        FieldValue::Map(items)=>for(_,value)in items{pending.push(value);},
        FieldValue::Value(value)=><super::DslValue as protocol::value::FromValue>::retire_decoded(value),
        FieldValue::Wire(value)=><super::DslValue as protocol::value::FromValue>::retire_decoded(value.properties),
        FieldValue::Expr(value)=>{let mut expressions=vec![value];while let Some(value)=expressions.pop(){match value{super::ExprValue::Neg(value)=>expressions.push(*value),super::ExprValue::Binary(_,left,right)=>{expressions.push(*left);expressions.push(*right);},super::ExprValue::Call(_,mut items)=>expressions.append(&mut items),super::ExprValue::Num(_)|super::ExprValue::Var(_)=>{}}}},
        FieldValue::Bool(_)|FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)|FieldValue::Text(_)|FieldValue::Bytes64(_)|FieldValue::Enum(_)|FieldValue::Absent=>{}
    }}
}

/// 🛡️ Holds intermediate fields until the complete record is committed.
pub struct EncodedRecord{record:Option<RecordValue>}
impl EncodedRecord{
    /// 🛡️ Retains a producer-admitted complete record across physical output failures.
    pub fn from_record(record:RecordValue)->Self{Self{record:Some(record)}}
    /// 🔎️ Borrows the guarded record without materializing another field graph.
    pub fn as_record(&self)->&RecordValue{self.record.as_ref().unwrap()}
    /// 📦️ Admits the hash table's complete known frontier before allocation.
    pub fn new(count:usize,control:&mut NativeEncodeControl<'_>)->Result<Self,String>{let bytes=count.checked_mul(std::mem::size_of::<(u16,FieldValue)>().saturating_add(128)).ok_or("native record slot overflow")?;control.charge(bytes)?;let mut record=RecordValue::default();record.fields.try_reserve(count).map_err(|_|"native record allocation failed")?;Ok(Self{record:Some(record)})}
    /// 📥️ Publishes a completed explicit field into this guarded record.
    pub fn insert(&mut self,id:u16,value:FieldValue){if let Some(previous)=self.record.as_mut().unwrap().fields.insert(id,value){retire_field(previous);}}
    /// 📤️ Transfers the fully constructed record to its physical encoder.
    pub fn take(mut self)->RecordValue{self.record.take().unwrap()}
}
impl Drop for EncodedRecord{fn drop(&mut self){if let Some(record)=self.record.take(){for value in record.fields.into_values(){retire_field(value);}}}}

/// 📋️ Projects a borrowed collection with its exact item workload and partial-field retirement.
pub fn project_list<T:DslField>(values:&[T],control:&mut NativeEncodeControl<'_>)->Result<Vec<FieldValue>,String>{
    control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?,|items:Vec<FieldValue>|{for item in items{retire_field(item);}});for value in values{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control)})?);control.step()?;}Ok(output.take())})
}

/// 🗺️ Projects literal map keys and their typed values before materializing the physical map.
pub fn project_map<T:DslField>(values:&std::collections::BTreeMap<String,T>,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,String>{
    control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?,|items:Vec<(String,FieldValue)>|{for(_,value)in items{retire_field(value);}});for(key,value)in values{let key=control.copy_text(key)?;let value=control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control)})?;output.as_mut().push((key,value));control.step()?;}Ok(FieldValue::Map(output.take()))})
}

/// 🌿️ Projects tagged records with literal owner keywords and known collection progress.
pub fn project_statements<T:DslVariants>(values:&[T],control:&mut NativeEncodeControl<'_>)->Result<FieldValue,String>{
    control.scoped_stage(|control|{control.begin_stage(values.len())?;let mut output=super::__rt::DecodedFieldOwner::new(control.allocate_vec(values.len())?,|items:Vec<(String,RecordValue)>|{for(_,record)in items{for value in record.fields.into_values(){retire_field(value);}}});for value in values{output.as_mut().push(control.scoped_stage(|control|{control.begin_stage(0)?;value.to_named_record_controlled(control)})?);control.step()?;}Ok(FieldValue::Statements(output.take()))})
}
