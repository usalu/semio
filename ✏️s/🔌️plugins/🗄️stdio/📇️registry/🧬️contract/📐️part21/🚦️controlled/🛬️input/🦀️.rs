//! 🛬️ One admitted workspace retains every failed Part21 prefix without synchronous destruction.
use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[derive(Default,semio_framework_value::RetireOwned)]
struct DecodeSlot{values:Vec<Part21Value>,name:String}
#[derive(Default,semio_framework_value::RetireOwned)]
struct DecodeOwner{
 slots:Vec<DecodeSlot>,text:Vec<u8>,entities:Vec<(String,Vec<Part21Value>)>,
 description:Vec<Part21Value>,header_name:Vec<Part21Value>,schema:Vec<Part21Value>,instances:Vec<Part21Instance>,
 value:Option<Part21Value>,instance:Option<Part21Instance>,decimal:Option<Part21Decimal>,header:Option<Part21Header>,document:Option<Part21Document>,
}
fn admitted<T>(control:&mut NativeDecodeControl<'_>,operation:impl FnOnce(&mut DecodeOwner,&mut NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{
 control.with_retirement_owner(std::mem::size_of::<ControlledRetirement<DecodeOwner>>(),|control|{
  let original=DecodeOwner::default();let mut owner=Box::new(ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original Part21 workspace has typed retirement")));
  let result=operation(owner.original_mut().unwrap(),control);(result,Some(owner))
 })
}
/// 🗂️ Validates the fixed Part21 object schema directly in borrowed slots without a discarded heap index.
fn object<'v>(value:&'v DslValue,allowed:&[&str],control:&mut NativeDecodeControl<'_>)->Result<&'v[(String,DslValue)],ValueError>{
 let DslValue::Object(entries)=value else{return Err(invalid("Part21 requires an object"));};
 fields(entries,allowed,control)?;Ok(entries.as_slice())
}
/// 🛡️ Keeps fixed-schema key rejection allocation-free at every typed child boundary.
fn fields(entries:&[(String,DslValue)],allowed:&[&str],control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 if entries.len()>allowed.len(){return Err(invalid("Part21 object exceeds its declared field roster"));}
 control.scoped_stage(|control|{
  control.begin_stage(entries.len())?;
  for(index,(key,_))in entries.iter().enumerate(){
   if !allowed.contains(&key.as_str()){return Err(invalid("unknown Part21 field"));}
   if entries[..index].iter().any(|(previous,_)|previous==key){return Err(invalid("duplicate Part21 field"));}
   control.step()?;
  }
  Ok(())
 })
}
impl DecodeOwner{
 fn text(&mut self,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<String,ValueError>{
  let text=value.as_str().ok_or_else(||invalid("Part21 text field requires Text"))?;self.text=control.allocate_vec(text.len())?;
  control.scoped_stage(|control|{control.begin_stage(text.len().div_ceil(65536))?;for chunk in text.as_bytes().chunks(65536){self.text.extend_from_slice(chunk);control.step()?;}Ok::<(),ValueError>(())})?;
  let original=std::mem::take(&mut self.text);Ok(unsafe{String::from_utf8_unchecked(original)})
 }
 fn slots(&mut self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if self.slots.is_empty(){self.slots=control.allocate_vec(64)?;for _ in 0..64{self.slots.push(DecodeSlot::default());}}Ok::<(),ValueError>(())}
 fn values(&mut self,value:&DslValue,depth:usize,control:&mut NativeDecodeControl<'_>)->Result<Vec<Part21Value>,ValueError>{
  let DslValue::Array(items)=value else{return Err(invalid("Part21 arguments require an array"));};self.slots(control)?;if depth>=self.slots.len(){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Part21 value nesting exceeds its admitted owner slots"));}
  self.slots[depth].values=control.allocate_vec(items.len())?;control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in items{let child=self.value(item,depth+1,control)?;self.slots[depth].values.push(child);control.step()?;}Ok::<(),ValueError>(())})?;
  Ok(std::mem::take(&mut self.slots[depth].values))
 }
 fn value(&mut self,value:&DslValue,depth:usize,control:&mut NativeDecodeControl<'_>)->Result<Part21Value,ValueError>{
  control.scoped_depth(64,|control|control.scoped_stage(|control|{
   let entries=object(value,&["kind","value","values","typeName"],control)?;let kind=required(entries,"kind",control)?.as_str().ok_or_else(||invalid("Part21 kind requires Text"))?;
   let allowed:&[&str]=match kind{"ref"|"str"|"enum"|"int"|"real"=>&["kind","value"],"list"=>&["kind","values"],"typed"=>&["kind","typeName","values"],"unset"|"derived"=>&["kind"],_=>return Err(invalid("unknown Part21 value kind"))};fields(entries,allowed,control)?;control.begin_stage(1)?;
   let result=match kind{
    "ref"=>Part21Value::Ref(u64::from_value_controlled(required(entries,"value",control)?,control)?),
    "str"=>Part21Value::Str(self.text(required(entries,"value",control)?,control)?),
    "enum"=>Part21Value::Enum(self.text(required(entries,"value",control)?,control)?),
    "int"=>Part21Value::Int(i64::from_value_controlled(required(entries,"value",control)?,control)?),
    "real"=>Part21Value::Real(self.decimal(required(entries,"value",control)?,control)?),
    "list"=>Part21Value::List(self.values(required(entries,"values",control)?,depth,control)?),
    "typed"=>{self.slots(control)?;if depth>=self.slots.len(){return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Part21 typed value exceeds its admitted owner slots"));}let name=self.text(required(entries,"typeName",control)?,control)?;self.slots[depth].name=name;let items=self.values(required(entries,"values",control)?,depth,control)?;Part21Value::Typed{name:std::mem::take(&mut self.slots[depth].name),items}},
    "unset"=>Part21Value::Unset,"derived"=>Part21Value::Derived,_=>unreachable!(),
   };self.value=Some(result);control.step()?;Ok(self.value.take().unwrap())
  }))
 }
 fn decimal(&mut self,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Decimal,ValueError>{
  let entries=object(value,&["negative","coefficient","scale","exponent"],control)?;let negative=bool::from_value_controlled(required(entries,"negative",control)?,control)?;let scale=u32::from_value_controlled(required(entries,"scale",control)?,control)?;let exponent=DslValue::field_controlled(entries,"exponent",control)?.map(|value|Option::<i32>::from_value_controlled(value,control)).transpose()?.flatten();let coefficient=self.text(required(entries,"coefficient",control)?,control)?;
  self.decimal=Some(Part21Decimal{negative,coefficient,scale,exponent});control.checkpoint()?;Ok(self.decimal.take().unwrap())
 }
 fn instance(&mut self,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Instance,ValueError>{
  control.scoped_stage(|control|{let entries=object(value,&["id","entities"],control)?;let id=u64::from_value_controlled(required(entries,"id",control)?,control)?;let DslValue::Array(records)=required(entries,"entities",control)?else{return Err(invalid("Part21 entities require an array"));};self.entities=control.allocate_vec(records.len())?;self.slots(control)?;control.begin_stage(records.len())?;
   for record in records{let fields=object(record,&["typeName","arguments"],control)?;self.slots[0].name=self.text(required(fields,"typeName",control)?,control)?;let arguments=self.values(required(fields,"arguments",control)?,1,control)?;self.entities.push((std::mem::take(&mut self.slots[0].name),arguments));control.step()?;}
   self.instance=Some(Part21Instance{id,entities:std::mem::take(&mut self.entities)});control.checkpoint()?;Ok(self.instance.take().unwrap())
  })
 }
 fn header(&mut self,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Header,ValueError>{
  let entries=object(value,&["fileDescription","fileName","fileSchema"],control)?;
  self.description=self.values(required(entries,"fileDescription",control)?,0,control)?;self.header_name=self.values(required(entries,"fileName",control)?,0,control)?;self.schema=self.values(required(entries,"fileSchema",control)?,0,control)?;
  self.header=Some(Part21Header{file_description:std::mem::take(&mut self.description),file_name:std::mem::take(&mut self.header_name),file_schema:std::mem::take(&mut self.schema)});control.checkpoint()?;Ok(self.header.take().unwrap())
 }
 fn document(&mut self,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Document,ValueError>{
  let entries=object(value,&["header","instances"],control)?;let header=self.header(required(entries,"header",control)?,control)?;self.header=Some(header);let DslValue::Array(records)=required(entries,"instances",control)?else{return Err(invalid("Part21 instances require an array"));};self.instances=control.allocate_vec(records.len())?;
  control.scoped_stage(|control|{control.begin_stage(records.len())?;for record in records{let instance=self.instance(record,control)?;self.instances.push(instance);control.step()?;}Ok::<(),ValueError>(())})?;
  self.document=Some(Part21Document{header:self.header.take().unwrap(),instances:std::mem::take(&mut self.instances)});control.checkpoint()?;Ok(self.document.take().unwrap())
 }
}
pub(crate) fn decode_value(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Value,ValueError>{admitted(control,|owner,control|owner.value(value,0,control))}
pub(crate) fn decode_instance(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Instance,ValueError>{admitted(control,|owner,control|owner.instance(value,control))}
pub(crate) fn decode_decimal(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Decimal,ValueError>{admitted(control,|owner,control|owner.decimal(value,control))}
pub(crate) fn decode_header(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Header,ValueError>{admitted(control,|owner,control|owner.header(value,control))}
pub(crate) fn decode_document(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Part21Document,ValueError>{admitted(control,|owner,control|owner.document(value,control))}
