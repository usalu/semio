//! 🔗️ Literal owned artifact identity preserves unrestricted snapshot fields.
use super::{ArtifactDialect,ArtifactRef};
use dsl::{DslField,FieldSpec,FieldValue,NativeDecodeControl,NativeSchemaControl,RecordLayout,RecordSpec,RecordSpecProducer,RecordValue,Shape};

fn spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"artifact-id",Shape::Text),FieldSpec::new(1,"artifact-kind",Shape::Text),FieldSpec::new(2,"standard",Shape::Text),FieldSpec::new(3,"subset",Shape::Text)])}
fn spec_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,String>{control.scoped_stage(|control|{control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;for(id,key)in[(0,"artifact-id"),(1,"artifact-kind"),(2,"standard"),(3,"subset")]{fields.push(dsl::schema::producer::field(id,key,Shape::Text,control)?);control.step()?;}dsl::schema::producer::record(None,RecordLayout::Inline,fields,control)})}
fn producer()->RecordSpecProducer{RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}
fn fields(record:&RecordValue)->Result<[&str;4],String>{
    if record.fields.len()!=4||record.fields.keys().any(|id|*id>3){return Err("artifact reference requires its four literal fields".into());}
    let text=|id|match record.get(id){Some(FieldValue::Text(value))=>Ok(value.as_str()),_=>Err("artifact reference identity field must be Text".to_string())};
    Ok([text(0)?,text(1)?,text(2)?,text(3)?])
}
fn project_record(reference:&ArtifactRef,control:&mut dsl::NativeEncodeControl<'_>)->Result<RecordValue,String>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|{
        control.begin_stage(4)?;let mut output=dsl::native_encoding::EncodedRecord::new(4,control)?;
        for(id,value)in [&reference.artifact_id,&reference.dialect.artifact_kind,&reference.dialect.standard,&reference.dialect.subset].into_iter().enumerate(){output.insert(id as u16,FieldValue::Text(control.copy_text(value)?));control.step()?;}
        Ok(output.take())
    }))
}
impl DslField for ArtifactRef{
    fn shape()->Shape{Shape::Record(producer())}
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,String>{control.checkpoint()?;Ok(Shape::Record(producer()))}
    fn to_value(&self)->FieldValue{
        let mut record=RecordValue::default();for(id,value)in [&self.artifact_id,&self.dialect.artifact_kind,&self.dialect.standard,&self.dialect.subset].into_iter().enumerate(){record.fields.insert(id as u16,FieldValue::Text(value.clone()));}FieldValue::Record(record)
    }
    fn to_value_controlled(&self,control:&mut dsl::NativeEncodeControl<'_>)->Result<FieldValue,String>{project_record(self,control).map(FieldValue::Record)}
    fn to_record_controlled(&self,control:&mut dsl::NativeEncodeControl<'_>)->Result<RecordValue,String>{project_record(self,control)}
    fn from_value(value:&FieldValue)->Result<Self,String>{let FieldValue::Record(record)=value else{return Err("expected artifact reference Record".into());};let[id,kind,standard,subset]=fields(record)?;Ok(Self{artifact_id:id.into(),dialect:ArtifactDialect{artifact_kind:kind.into(),standard:standard.into(),subset:subset.into()}})}
    fn from_value_controlled(value:&FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{let FieldValue::Record(record)=value else{return Err("expected artifact reference Record".into());};Self::from_record_controlled(record,control)}
    fn from_record_controlled(record:&RecordValue,control:&mut NativeDecodeControl<'_>)->Result<Self,String>{
        control.scoped_stage(|control|{control.begin_stage(4)?;let[id,kind,standard,subset]=fields(record)?;let artifact_id=control.copy_text(id)?;control.step()?;let artifact_kind=control.copy_text(kind)?;control.step()?;let standard=control.copy_text(standard)?;control.step()?;let subset=control.copy_text(subset)?;control.step()?;Ok(Self{artifact_id,dialect:ArtifactDialect{artifact_kind,standard,subset}})})
    }
}
