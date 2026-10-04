//! 🛬️ Literal JSON record fields are admitted before typed storage and reconstruction.
use super::*;
use semio_framework_value::{NativeDecodeControl,ValueError,ValueRefusalKind};

fn field(record:&semio_framework_dsl_record::RecordValue,id:u16)->Result<&semio_framework_dsl_record::FieldValue,ValueError>{record.get(id).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,format!("JSON logical field {id} is missing")))}
fn list(value:&semio_framework_dsl_record::FieldValue)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match value{semio_framework_dsl_record::FieldValue::List(values)=>Ok(values),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical list field differs"))}}
fn record(value:&semio_framework_dsl_record::FieldValue)->Result<&semio_framework_dsl_record::RecordValue,ValueError>{match value{semio_framework_dsl_record::FieldValue::Record(value)=>Ok(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical record field differs"))}}
fn text(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{match value{semio_framework_dsl_record::FieldValue::Text(value)=>control.copy_text(value),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical text field differs"))}}
fn optional_text(value:&semio_framework_dsl_record::FieldValue,control:&mut NativeDecodeControl<'_>)->Result<Option<String>,ValueError>{if matches!(value,semio_framework_dsl_record::FieldValue::Absent){Ok(None)}else{text(value,control).map(Some)}}
fn exact_fields(record:&semio_framework_dsl_record::RecordValue,count:u16)->Result<(),ValueError>{if record.fields.len()!=count as usize||record.fields.keys().any(|id|*id>=count){Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical record fields differ"))}else{Ok(())}}

pub(super) fn bind(source:&semio_framework_dsl_record::RecordValue,control:&mut NativeDecodeControl<'_>,maximum_rows:usize)->Result<Snapshot,ValueError>{
 control.checkpoint()?;exact_fields(source,2)?;let nodes=list(field(source,1)?)?;
 let mut rows=nodes.len().checked_add(1).filter(|rows|*rows<=maximum_rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"JSON logical row count exceeds caller limit"))?;
 if nodes.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical root is missing"))}control.begin_stage(nodes.len())?;
 for node in nodes{control.step()?;let node=record(node)?;exact_fields(node,6)?;let items=list(field(node,4)?)?.len();let members=list(field(node,5)?)?.len();rows=rows.checked_add(items).and_then(|rows|rows.checked_add(members)).filter(|rows|*rows<=maximum_rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"JSON logical row count exceeds caller limit"))?;}
 control.begin_stage(rows-1)?;
 let schema=text(field(source,0)?,control)?;let mut output=control.allocate_vec::<Node>(nodes.len())?;
 for source in nodes{
  control.step()?;let source=record(source)?;exact_fields(source,6)?;
  let kind=control.scoped_stage(|control|{control.begin_stage(0)?;<Kind as semio_framework_dsl_record::DslField>::from_value_controlled(field(source,0)?,control)})?;
  let boolean=match field(source,1)?{semio_framework_dsl_record::FieldValue::Absent=>None,semio_framework_dsl_record::FieldValue::Bool(value)=>Some(*value),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical boolean field differs"))};
  let number_lexeme=optional_text(field(source,2)?,control)?;let string_value=optional_text(field(source,3)?,control)?;
  let items=list(field(source,4)?)?;let members=list(field(source,5)?)?;
  let mut owned_items=control.allocate_vec::<u64>(items.len())?;
  for item in items{control.step()?;owned_items.push(control.scoped_stage(|control|{control.begin_stage(0)?;<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(item,control)})?)}
  let mut owned_members=control.allocate_vec::<Member>(members.len())?;
  for member in members{control.step()?;let member=record(member)?;exact_fields(member,2)?;owned_members.push(Member{key:text(field(member,0)?,control)?,value:control.scoped_stage(|control|{control.begin_stage(0)?;<u64 as semio_framework_dsl_record::DslField>::from_value_controlled(field(member,1)?,control)})?})}
  output.push(Node{kind,boolean,number_lexeme,string_value,items:owned_items,members:owned_members});
 }
 control.checkpoint()?;Ok(Snapshot{schema,nodes:output})
}

struct Items(Vec<JsonValue>);
impl Drop for Items{fn drop(&mut self){for value in self.0.drain(..){retire_value(value)}}}
struct Members(Vec<JsonMember>);
impl Drop for Members{fn drop(&mut self){for member in self.0.drain(..){retire_value(member.value)}}}
struct Values(Vec<Option<JsonValue>>);
impl Drop for Values{fn drop(&mut self){for value in &mut self.0{if let Some(value)=value.take(){retire_value(value)}}}}

pub(super) fn reconstruct(snapshot:Snapshot,control:&mut NativeDecodeControl<'_>)->Result<JsonSnapshot,ValueError>{
 control.checkpoint()?;if snapshot.nodes.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical root is missing"))}
 let mut units=snapshot.nodes.len();control.begin_stage(snapshot.nodes.len())?;for node in &snapshot.nodes{control.step()?;units=units.checked_add(node.items.len()).and_then(|units|units.checked_add(node.members.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"JSON logical work count overflow"))?;}
 control.begin_stage(units)?;
 let mut owned=control.allocate_vec::<bool>(snapshot.nodes.len())?;owned.resize(snapshot.nodes.len(),false);owned[0]=true;
 for(index,node)in snapshot.nodes.iter().enumerate(){
  control.step()?;validate_node(node).map_err(|message|ValueError::new(ValueRefusalKind::InvalidValue,message))?;
  for key in node.items.iter().copied().chain(node.members.iter().map(|member|member.value)){
   control.step()?;let key=usize::try_from(key).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical child index exceeds native domain"))?;
   if key<=index||key>=owned.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical child topology differs"))}
   if std::mem::replace(&mut owned[key],true){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical child has multiple owners"))}
  }
 }
 if owned.iter().any(|present|!*present){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical record contains unowned nodes"))}
 control.begin_stage(units)?;let mut values=Values(control.allocate_vec::<Option<JsonValue>>(snapshot.nodes.len())?);values.0.resize_with(snapshot.nodes.len(),||None);
 for(index,node)in snapshot.nodes.into_iter().enumerate().rev(){
  control.step()?;let Node{kind,boolean,number_lexeme,string_value,items,members}=node;
  let value=match kind{
   Kind::Null=>JsonValue::Null,
   Kind::Boolean=>JsonValue::Bool{value:boolean.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical boolean is missing"))?},
   Kind::Number=>JsonValue::Number{lexeme:number_lexeme.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical number is missing"))?},
   Kind::String=>JsonValue::String{value:string_value.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical string is missing"))?},
   Kind::Array=>{let mut output=Items(control.allocate_vec::<JsonValue>(items.len())?);for key in items{control.step()?;output.0.push(child(&mut values.0,index,key).map_err(|message|ValueError::new(ValueRefusalKind::InvalidValue,message))?)}JsonValue::Array{items:std::mem::take(&mut output.0)}},
   Kind::Object=>{let mut output=Members(control.allocate_vec::<JsonMember>(members.len())?);for member in members{control.step()?;output.0.push(JsonMember{key:member.key,value:child(&mut values.0,index,member.value).map_err(|message|ValueError::new(ValueRefusalKind::InvalidValue,message))?})}JsonValue::Object{members:std::mem::take(&mut output.0)}}
  };values.0[index]=Some(value);
 }
 control.checkpoint()?;let value=values.0[0].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"JSON logical root is missing"))?;Ok(JsonSnapshot{schema:snapshot.schema,value})
}

pub(super) fn retire_value(value:JsonValue){
 let mut pending=vec![value];while let Some(value)=pending.pop(){match value{JsonValue::Array{items}=>pending.extend(items),JsonValue::Object{members}=>pending.extend(members.into_iter().map(|member|member.value)),_=>{}}}
}
