//! 📦️ Flat owned JSON domain records preserve exact lexemes and ordered members.
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::*;
#[path="🛬️decode/🦀️.rs"]
pub(crate) mod decoding;
#[path="🛫️encode/🦀️.rs"]
mod encoding;
#[derive(semio_framework_dsl_record_derive::DslScalar)]
pub(crate) enum Kind { Null, Boolean, Number, String, Array, Object }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Member { key: String, value: u64 }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Node { kind: Kind, boolean: Option<bool>, number_lexeme: Option<String>, string_value: Option<String>, items: Vec<u64>, members: Vec<Member> }
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(crate) struct Snapshot { schema: String, nodes: Vec<Node> }

impl From<&JsonSnapshot> for Snapshot {
 fn from(snapshot:&JsonSnapshot)->Self {
  let mut pending=std::collections::VecDeque::from([&snapshot.value]);let mut nodes=Vec::new();let mut next=1u64;
  while let Some(value)=pending.pop_front(){
   let mut node=Node{kind:Kind::Null,boolean:None,number_lexeme:None,string_value:None,items:Vec::new(),members:Vec::new()};
   match value{
    JsonValue::Null=>{},
    JsonValue::Bool{value}=>{node.kind=Kind::Boolean;node.boolean=Some(*value)},
    JsonValue::Number{lexeme}=>{node.kind=Kind::Number;node.number_lexeme=Some(lexeme.clone())},
    JsonValue::String{value}=>{node.kind=Kind::String;node.string_value=Some(value.clone())},
    JsonValue::Array{items}=>{node.kind=Kind::Array;for child in items{node.items.push(next);next+=1;pending.push_back(child)}},
    JsonValue::Object{members}=>{node.kind=Kind::Object;for member in members{node.members.push(Member{key:member.key.clone(),value:next});next+=1;pending.push_back(&member.value)}}
   }
   nodes.push(node);
  }
  Self{schema:snapshot.schema.clone(),nodes}
 }
}
fn child(values:&mut[Option<JsonValue>],parent:usize,key:u64)->Result<JsonValue,String>{
 let key=usize::try_from(key).map_err(|_|"JSON logical child index exceeds native domain")?;
 if key<=parent||key>=values.len(){return Err("JSON logical child topology differs".into())}
 values[key].take().ok_or_else(||"JSON logical child has multiple owners".into())
}
fn validate_node(node:&Node)->Result<(),String>{
 let selected=match node.kind{Kind::Null=>node.boolean.is_none()&&node.number_lexeme.is_none()&&node.string_value.is_none()&&node.items.is_empty()&&node.members.is_empty(),Kind::Boolean=>node.boolean.is_some()&&node.number_lexeme.is_none()&&node.string_value.is_none()&&node.items.is_empty()&&node.members.is_empty(),Kind::Number=>node.boolean.is_none()&&node.number_lexeme.is_some()&&node.string_value.is_none()&&node.items.is_empty()&&node.members.is_empty(),Kind::String=>node.boolean.is_none()&&node.number_lexeme.is_none()&&node.string_value.is_some()&&node.items.is_empty()&&node.members.is_empty(),Kind::Array=>node.boolean.is_none()&&node.number_lexeme.is_none()&&node.string_value.is_none()&&node.members.is_empty(),Kind::Object=>node.boolean.is_none()&&node.number_lexeme.is_none()&&node.string_value.is_none()&&node.items.is_empty()};
 if !selected{return Err("JSON logical record contains unrelated variant fields".into())}Ok(())
}
fn validate(nodes:&[Node])->Result<(),String>{
 if nodes.is_empty(){return Err("JSON logical root is missing".into())}
 let mut owned=vec![false;nodes.len()];owned[0]=true;
 for(index,node)in nodes.iter().enumerate(){
  validate_node(node)?;
  for key in node.items.iter().copied().chain(node.members.iter().map(|member|member.value)){
   let key=usize::try_from(key).map_err(|_|"JSON logical child index exceeds native domain")?;
   if key<=index||key>=owned.len(){return Err("JSON logical child topology differs".into())}
   if std::mem::replace(&mut owned[key],true){return Err("JSON logical child has multiple owners".into())}
  }
 }
 if owned.iter().any(|present|!*present){return Err("JSON logical record contains unowned nodes".into())}Ok(())
}
impl TryFrom<Snapshot> for JsonSnapshot{
 type Error=String;
 fn try_from(snapshot:Snapshot)->Result<Self,String>{
  validate(&snapshot.nodes)?;
  let mut values=(0..snapshot.nodes.len()).map(|_|None).collect::<Vec<Option<JsonValue>>>();
  for(index,node)in snapshot.nodes.into_iter().enumerate().rev(){
   let Node{kind,boolean,number_lexeme,string_value,items,members}=node;
   values[index]=Some(match(kind,boolean,number_lexeme,string_value,items.is_empty(),members.is_empty()){
    (Kind::Null,None,None,None,true,true)=>JsonValue::Null,
    (Kind::Boolean,Some(value),None,None,true,true)=>JsonValue::Bool{value},
    (Kind::Number,None,Some(lexeme),None,true,true)=>JsonValue::Number{lexeme},
    (Kind::String,None,None,Some(value),true,true)=>JsonValue::String{value},
    (Kind::Array,None,None,None,_,true)=>JsonValue::Array{items:items.into_iter().map(|key|child(&mut values,index,key)).collect::<Result<Vec<_>,_>>()?},
    (Kind::Object,None,None,None,true,_)=>JsonValue::Object{members:members.into_iter().map(|member|Ok(JsonMember{key:member.key,value:child(&mut values,index,member.value)?})).collect::<Result<Vec<_>,String>>()?},
    _=>return Err("JSON logical record contains unrelated variant fields".into())
   });
  }
  let value=values[0].take().ok_or_else(||"JSON logical root is missing".to_string())?;
  if values.iter().any(Option::is_some){return Err("JSON logical record contains unowned nodes".into())}
  Ok(Self{schema:snapshot.schema,value})
 }
}


impl store::ArtifactPack for JsonSnapshot{
 fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
 fn record_spec()->Option<semio_framework_dsl_record::RecordSpec>{Some(Snapshot::__dsl_spec())}
 fn encode_pack_with(&self,options:&store::PackEncodeOptions)->Result<Vec<u8>,store::PackError>{
  let inner=store::pack_rt::encode_document(&Snapshot::__dsl_spec(),&Snapshot::from(self).__dsl_to_record(),options)?;
  let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.json",store::semio_format::Component::Pack,1).map_err(|error|store::PackError::from(error.into_value_error()))?;Ok(store::semio_format::wrap_binary(&envelope,&inner))
 }
 fn decode_pack_with(bytes:&[u8],options:&store::PackDecodeOptions)->Result<Self,store::PackError>{
  let(envelope,inner)=store::semio_format::unwrap_binary(bytes).map_err(|error|store::PackError::from(error.into_value_error()))?;
  if !envelope.matches_identity("stdio.json",store::semio_format::Component::Pack,1){return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "JSON logical pack identity differs")))}
  let(record,_)=store::pack_rt::decode_document(&inner,&Snapshot::__dsl_spec(),options)?;
  Snapshot::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.try_into().map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
 }
}

pub(crate) fn preflight(snapshot:&JsonSnapshot,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<(),semio_framework_value::ValueError>{
 let mut bound=store::sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;bound.add(8192)?;bound.repeated(snapshot.schema.len(),6)?;
 let mut pending=vec![&snapshot.value];let mut rows=2usize;bound.check_rows(rows)?;
 while let Some(value)=pending.pop(){
  bound.add(448)?;
  match value{
   JsonValue::Null|JsonValue::Bool{..}=>{},
   JsonValue::Number{lexeme}=>bound.repeated(lexeme.len(),6)?,
   JsonValue::String{value}=>bound.repeated(value.len(),6)?,
   JsonValue::Array{items}=>{rows=rows.checked_add(items.len().checked_mul(2).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON logical row count overflow"))?).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON logical row count overflow"))?;bound.check_rows(rows)?;bound.repeated(items.len(),96)?;pending.extend(items.iter().rev())},
   JsonValue::Object{members}=>{rows=rows.checked_add(members.len().checked_mul(2).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON logical row count overflow"))?).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON logical row count overflow"))?;bound.check_rows(rows)?;bound.repeated(members.len(),192)?;for member in members.iter().rev(){bound.repeated(member.key.len(),6)?;pending.push(&member.value)}}
  }
 }
 bound.finish()
}

pub(crate) fn retire(snapshot:JsonSnapshot){decoding::retire_value(snapshot.value)}

pub(crate) fn encode(snapshot:&JsonSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{encoding::encode(snapshot,encoding,control)}

pub(crate) fn reconstruct_record(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::native_decoding::NativeDecodeControl<'_>,maximum_rows:usize)->Result<JsonSnapshot,semio_framework_value::ValueError>{
 let snapshot=decoding::bind(record,control,maximum_rows)?;decoding::reconstruct(snapshot,control)
}

pub(crate) fn decode(payload:&store::io_schema::IoPayload,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<JsonSnapshot,semio_framework_value::ValueError>{
 let maximum_rows=control.limits().max_rows;
 store::decode_sqlite_snapshot_record_native(payload,"stdio.json",Snapshot::__dsl_spec_producer(),|record,native|reconstruct_record(record,native,maximum_rows),control)
}
