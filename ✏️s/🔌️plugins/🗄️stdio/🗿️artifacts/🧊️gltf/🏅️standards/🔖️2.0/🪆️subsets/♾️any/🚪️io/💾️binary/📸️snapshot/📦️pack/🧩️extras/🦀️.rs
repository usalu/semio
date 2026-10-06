//! 🧩️ Ordered GLTF extras use explicit flat typed nodes without recursive wire lowering.
use super::*;
#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum JsonKind{Null,Boolean,Number,String,Array,Object}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct JsonMember{name:String,value:u64}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct JsonNode{kind:JsonKind,boolean:Option<bool>,number:Option<f64>,text:Option<String>,items:Vec<u64>,members:Vec<JsonMember>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(super) struct Json{nodes:Vec<JsonNode>}
impl From<&GltfJson> for Json{
 fn from(value:&GltfJson)->Self{let mut pending=std::collections::VecDeque::from([value]);let mut nodes=Vec::new();let mut next=1u64;while let Some(value)=pending.pop_front(){let mut node=JsonNode{kind:JsonKind::Null,boolean:None,number:None,text:None,items:Vec::new(),members:Vec::new()};match value{
 GltfJson::Null=>{},GltfJson::Bool(value)=>{node.kind=JsonKind::Boolean;node.boolean=Some(*value)},GltfJson::Number(value)=>{node.kind=JsonKind::Number;node.number=Some(*value)},GltfJson::String(value)=>{node.kind=JsonKind::String;node.text=Some(value.clone())},
 GltfJson::Array(values)=>{node.kind=JsonKind::Array;for value in values{node.items.push(next);next+=1;pending.push_back(value)}},
 GltfJson::Object(values)=>{node.kind=JsonKind::Object;for(name,value)in values{node.members.push(JsonMember{name:name.clone(),value:next});next+=1;pending.push_back(value)}}}nodes.push(node)}Self{nodes}}
}
fn child(values:&mut[Option<GltfJson>],parent:usize,key:u64)->Result<GltfJson,String>{let key=usize::try_from(key).map_err(|_|"GLTF extras child index exceeds native domain")?;if key<=parent||key>=values.len(){return Err("GLTF extras child topology differs".into())}values[key].take().ok_or_else(||"GLTF extras child has multiple owners".into())}
fn array(values:&mut[Option<GltfJson>],parent:usize,keys:Vec<u64>)->Result<Vec<GltfJson>,String>{let mut result=OwnedValues(Vec::with_capacity(keys.len()));for key in keys{result.0.push(Some(child(values,parent,key)?));}Ok(result.0.iter_mut().filter_map(Option::take).collect())}
fn object(values:&mut[Option<GltfJson>],parent:usize,members:Vec<JsonMember>)->Result<Vec<(String,GltfJson)>,String>{let mut result=Vec::with_capacity(members.len());for member in members{match child(values,parent,member.value){Ok(value)=>result.push((member.name,value)),Err(error)=>{for(_,value)in result{retire(value);}return Err(error)}}}Ok(result)}
impl TryFrom<Json> for GltfJson{
 type Error=String;
 fn try_from(value:Json)->Result<Self,String>{if value.nodes.is_empty(){return Err("GLTF extras root is missing".into())}let mut values=OwnedValues((0..value.nodes.len()).map(|_|None).collect());for(index,node)in value.nodes.into_iter().enumerate().rev(){let JsonNode{kind,boolean,number,text,items,members}=node;values.0[index]=Some(match(kind,boolean,number,text,items.is_empty(),members.is_empty()){
 (JsonKind::Null,None,None,None,true,true)=>Self::Null,
 (JsonKind::Boolean,Some(value),None,None,true,true)=>Self::Bool(value),
 (JsonKind::Number,None,Some(value),None,true,true)=>Self::Number(value),
 (JsonKind::String,None,None,Some(value),true,true)=>Self::String(value),
 (JsonKind::Array,None,None,None,_,true)=>Self::Array(array(&mut values.0,index,items)?),
 (JsonKind::Object,None,None,None,true,_)=>Self::Object(object(&mut values.0,index,members)?),
 _=>return Err("GLTF extras record contains unrelated variant fields".into())});}if values.0.iter().skip(1).any(Option::is_some){return Err("GLTF extras contains an unowned node".into())}values.0[0].take().ok_or_else(||"GLTF extras root is missing".into())}
}

struct OwnedValues(Vec<Option<GltfJson>>);
pub(super) fn retire(value:GltfJson){let mut pending=vec![value];while let Some(value)=pending.pop(){match value{GltfJson::Array(values)=>pending.extend(values),GltfJson::Object(values)=>pending.extend(values.into_iter().map(|(_,child)|child)),_=>{}}}}
impl Drop for OwnedValues{fn drop(&mut self){for value in self.0.iter_mut().filter_map(Option::take){retire(value);}}}
impl Json{
 pub(super) fn reconstruct_controlled(self,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<GltfJson,ValueError>{
  if self.nodes.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras root is missing"))}
  let mut total=self.nodes.len();for node in&self.nodes{total=total.checked_add(node.items.len()).and_then(|n|n.checked_add(node.members.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras entity count overflow"))?;}
  control.begin_stage(total)?;
  let mut slots=control.allocate_vec(self.nodes.len())?;slots.resize_with(self.nodes.len(),||None);let mut values=OwnedValues(slots);
  for(index,node)in self.nodes.into_iter().enumerate().rev(){control.step()?;let JsonNode{kind,boolean,number,text,items,members}=node;let value=match(kind,boolean,number,text,items.is_empty(),members.is_empty()){
   (JsonKind::Null,None,None,None,true,true)=>GltfJson::Null,
   (JsonKind::Boolean,Some(value),None,None,true,true)=>GltfJson::Bool(value),
   (JsonKind::Number,None,Some(value),None,true,true)=>GltfJson::Number(value),
   (JsonKind::String,None,None,Some(value),true,true)=>GltfJson::String(value),
   (JsonKind::Array,None,None,None,_,true)=>{let mut children=control.allocate_vec(items.len())?;for key in items{if let Err(error)=control.step(){for value in children{retire(value);}return Err(error)}match child(&mut values.0,index,key){Ok(value)=>children.push(value),Err(error)=>{for value in children{retire(value);}return Err(ValueError::new(ValueRefusalKind::InvalidValue,error))}}}GltfJson::Array(children)},
   (JsonKind::Object,None,None,None,true,_)=>{let mut children=control.allocate_vec(members.len())?;for member in members{if let Err(error)=control.step(){for(_,value)in children{retire(value);}return Err(error)}match child(&mut values.0,index,member.value){Ok(value)=>children.push((member.name,value)),Err(error)=>{for(_,value)in children{retire(value);}return Err(ValueError::new(ValueRefusalKind::InvalidValue,error))}}}GltfJson::Object(children)},
   _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras record contains unrelated variant fields"))};values.0[index]=Some(value);
  }if values.0.iter().skip(1).any(Option::is_some){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras contains an unowned node"))}control.checkpoint()?;values.0[0].take().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras root is missing"))
 }
}
