//! 📋️ Actual property declarations and port directions own their typed native fields.
use super::{PropertyDef,PropertyKind,PortDirection};
use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::FieldValue;
use semio_framework_dsl_record::RecordValue;
use semio_framework_dsl_record::RecordSpec;
use semio_framework_dsl_record::RecordSpecProducer;
use semio_framework_dsl_record::FieldSpec;
use semio_framework_dsl_record::RecordLayout;
use semio_framework_dsl_record::Shape;
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_value::NativeEncodeControl;
use semio_framework_value::NativeDecodeControl;
use semio_framework_value::ValueError;
use semio_framework_value::{FromValue,ValueRefusalKind,ValueType,DecodedValue};
macro_rules! scalar {
    ($ty:ty,$first:ident,$second:ident,$first_name:literal,$second_name:literal)=>{
        impl semio_framework_dsl_record::BorrowedDslField for $ty { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Enum(&[($first_name,0),($second_name,1)]); }
        impl DslField for $ty {
            fn shape()->Shape{Shape::Enum(vec![($first_name.into(),0),($second_name.into(),1)])}
            fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.scoped_stage(|control|{control.begin_stage(2)?;let mut values=control.allocate_vec(2)?;values.push((control.copy_text($first_name)?,0));control.step()?;values.push((control.copy_text($second_name)?,1));control.step()?;Ok(Shape::Enum(values))})}
            fn to_value(&self)->FieldValue{FieldValue::Enum(match self{Self::$first=>0,Self::$second=>1})}
            fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.checkpoint()?;Ok(<Self as DslField>::to_value(self))}
            fn from_value(value:&FieldValue)->Result<Self,String>{match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err("invalid declared enum ordinal".into())}}
            fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;match value{FieldValue::Enum(0)=>Ok(Self::$first),FieldValue::Enum(1)=>Ok(Self::$second),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid declared enum ordinal"))}}
        }
    };
}
scalar!(PropertyKind,Data,Derived,"data","derived");
scalar!(PortDirection,In,Out,"in","out");
fn ordinary_spec()->RecordSpec{semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,vec![FieldSpec::new(0,"name",<String as DslField>::shape()),FieldSpec::new(1,"kind",<PropertyKind as DslField>::shape()),FieldSpec::new(2,"value-type",<ValueType as DslField>::shape()),FieldSpec::new(3,"expr",<String as DslField>::shape()).optional()])}
fn controlled_spec<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{
    control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;
    for(id,key,shape,optional)in[(0,"name",<String as DslField>::shape_controlled(control)?,false),(1,"kind",<PropertyKind as DslField>::shape_controlled(control)?,false),(2,"value-type",<ValueType as DslField>::shape_controlled(control)?,false),(3,"expr",<String as DslField>::shape_controlled(control)?,true)]{let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
    semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
})}
fn decoding_spec(control:&mut NativeDecodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn encoding_spec(control:&mut NativeEncodeControl<'_>)->Result<RecordSpec,ValueError>{controlled_spec(control)}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:ordinary_spec,decoding:decoding_spec,encoding:encoding_spec}}
fn retire(value:PropertyDef){<ValueType as FromValue>::retire_decoded(value.value_type)}
fn encode_field<T:DslField>(value:&T,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;T::to_value_controlled(value,control)})}
fn decode_field<T:DslField>(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(value,control)})}
impl DslField for PropertyDef {
    fn shape()->Shape{semio_framework_dsl_record::Shape::Record(producer())}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(producer()))}
    fn to_value(&self)->FieldValue{semio_framework_dsl_record::FieldValue::Record(RecordValue{fields:[(0,<String as DslField>::to_value(&self.name)),(1,<PropertyKind as DslField>::to_value(&self.kind)),(2,<ValueType as DslField>::to_value(&self.value_type)),(3,self.expr.as_ref().map(<String as DslField>::to_value).unwrap_or(semio_framework_dsl_record::FieldValue::Absent))].into_iter().collect()})}
    fn to_record_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<RecordValue,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(4,control)?;
        record.insert(0,encode_field(&self.name,control)?)?;control.step()?;
        record.insert(1,encode_field(&self.kind,control)?)?;control.step()?;
        record.insert(2,encode_field(&self.value_type,control)?)?;control.step()?;
        record.insert(3,match &self.expr{Some(value)=>encode_field(value,control)?,None=>semio_framework_dsl_record::FieldValue::Absent})?;control.step()?;
        Ok(record.take())
    })}
    fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<FieldValue,ValueError>{let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(Self::to_record_controlled(self,control)?);Ok(semio_framework_dsl_record::FieldValue::Record(record.take()))}
    fn from_value(value:&FieldValue)->Result<Self,String>{let semio_framework_dsl_record::FieldValue::Record(record)=value else{return Err("expected property declaration record".into())};let get=|id|record.get(id).ok_or_else(||"missing property declaration field".to_string());Ok(Self{name:<String as DslField>::from_value(get(0)?)?,kind:<PropertyKind as DslField>::from_value(get(1)?)?,value_type:<ValueType as DslField>::from_value(get(2)?)?,expr:match get(3)?{semio_framework_dsl_record::FieldValue::Absent=>None,value=>Some(<String as DslField>::from_value(value)?)}})}
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_stage(|control|{
        control.begin_stage(4)?;if record.fields.len()!=4{return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue,"invalid property declaration fields"));}
        let get=|id|record.get(id).ok_or_else(||semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue,"missing property declaration field"));
        let name=decode_field::<String>(get(0)?,control)?;control.step()?;
        let kind=decode_field::<PropertyKind>(get(1)?,control)?;control.step()?;
        let value_type=DecodedValue::new(decode_field::<ValueType>(get(2)?,control)?,<ValueType as FromValue>::retire_decoded);control.step()?;
        let expr=match get(3)?{semio_framework_dsl_record::FieldValue::Absent=>None,value=>Some(decode_field::<String>(value,control)?)};
        let value=DecodedValue::new(Self{name,kind,value_type:value_type.take(),expr},retire);control.step()?;Ok(value.take())
    })}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(record)=>Self::from_record_controlled(record,control),_=>Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue,"expected property declaration record"))}}
    fn retire_decoded(self){retire(self)}
}

impl semio_framework_dsl_record::BorrowedDslRecord for PropertyDef {
    const RECORD:semio_framework_dsl_record::BorrowedRecordSpec=semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&[
        semio_framework_dsl_record::BorrowedFieldSpec::new(0,"name",semio_framework_dsl_record::BorrowedShape::Text),
        semio_framework_dsl_record::BorrowedFieldSpec::new(1,"kind",<PropertyKind as semio_framework_dsl_record::BorrowedDslField>::SHAPE),
        semio_framework_dsl_record::BorrowedFieldSpec::new(2,"value-type",semio_framework_dsl_record::BorrowedShape::Value),
        semio_framework_dsl_record::BorrowedFieldSpec{optional:true,..semio_framework_dsl_record::BorrowedFieldSpec::new(3,"expr",semio_framework_dsl_record::BorrowedShape::Text)},
    ]};
}
impl semio_framework_dsl_record::BorrowedDslField for PropertyDef { const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>); }
