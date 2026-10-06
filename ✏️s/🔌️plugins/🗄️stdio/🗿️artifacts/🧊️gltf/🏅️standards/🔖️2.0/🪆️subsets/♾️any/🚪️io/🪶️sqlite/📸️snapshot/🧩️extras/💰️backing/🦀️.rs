//! 🧩️ Adjacent exact GLTF extras ownership preserves literal ordered duplicate members.
use super::*;
const NUMBER:&[FloatColumn]=&[FloatColumn::Binary64(3)];
struct Values(Vec<(i64,Option<GltfJson>)>);
impl Drop for Values{fn drop(&mut self){for(_,value)in self.0.drain(..){if let Some(value)=value{<GltfJson as semio_framework_dsl_record::DslField>::retire_decoded(value);}}}}
impl Values{
 fn put(&mut self,key:i64,value:GltfJson,read:&mut Read<'_,'_,'_>)->Result<(),ValueError>{let value=owned(value);match self.0.binary_search_by_key(&key,|item|item.0){Ok(_)=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras value has multiple owners")),Err(position)=>{if self.0.len()==self.0.capacity(){let additional=self.0.capacity().max(1);read.reserve_fields(&mut self.0,additional)?;}self.0.insert(position,(key,Some(value.take())));Ok(())}}}
 fn take(&mut self,key:i64)->Result<GltfJson,ValueError>{self.0.binary_search_by_key(&key,|item|item.0).ok().and_then(|index|self.0.remove(index).1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras child is absent or multiply owned"))}
}
struct Object(Vec<(String,GltfJson)>);
impl Drop for Object{fn drop(&mut self){for(_,value)in self.0.drain(..){<GltfJson as semio_framework_dsl_record::DslField>::retire_decoded(value);}}}
enum Pending{Value(i64),Array(i64,Vec<i64>),Object(i64,Vec<(String,i64)>)}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>,root:i64)->Result<GltfJson,ValueError>{
 let mut pending=read.reserve(1)?;pending.push(Pending::Value(root));let mut values=Values(Vec::new());
 while let Some(step)=pending.pop(){read.checkpoint()?;match step{
  Pending::Value(key)=>{
   let row=read.key("gltf_json_value",key)?;if row.values.len()!=7{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras value column count differs"))}let scalar=FloatRow::new(row,NUMBER)?;let kind=row.text(1)?;
   if(kind!="boolean"&&row.values[2]!=SqliteValue::Null)||(kind!="string"&&row.values[4]!=SqliteValue::Null)||(kind!="number"&&!scalar.is_null(3)?){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras contains unused typed scalar fields"))}
   let value=match kind{
    "null"=>GltfJson::Null,"boolean"=>GltfJson::Bool(read.boolean(row,2)?),"number"=>{read.scalar()?;GltfJson::Number(scalar.real(3)?)},"string"=>GltfJson::String(read.text(row,4)?),
    "array"=>{let rows=read.rows("gltf_json_array_element",1,key,2)?;let mut children=read.reserve(rows.len())?;for row in rows{let child=row.integer(3)?;if child<1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras child must be a positive identity"))}children.push(child);}let count=children.len();read.reserve_fields(&mut pending,count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras task count overflow"))?)?;let parent=pending.len();pending.push(Pending::Array(key,children));for position in(0..count).rev(){let child=match &pending[parent]{Pending::Array(_,children)=>children[position],_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"GLTF extras array task missing"))};pending.push(Pending::Value(child));}continue},
    "object"=>{let rows=read.rows("gltf_json_object_member",1,key,2)?;let mut children=read.reserve(rows.len())?;for row in rows{let child=row.integer(4)?;if child<1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras child must be a positive identity"))}children.push((read.text(row,3)?,child));}let count=children.len();read.reserve_fields(&mut pending,count.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"GLTF extras task count overflow"))?)?;let parent=pending.len();pending.push(Pending::Object(key,children));for position in(0..count).rev(){let child=match &pending[parent]{Pending::Object(_,children)=>children[position].1,_=>return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"GLTF extras object task missing"))};pending.push(Pending::Value(child));}continue},
    _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras value kind is unknown"))
   };values.put(key,value,read)?;
  },
  Pending::Array(key,children)=>{let mut array=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(read.reserve(children.len())?,<Vec<GltfJson>as semio_framework_dsl_record::DslField>::retire_decoded);for child in children{array.as_mut().push(values.take(child)?);read.checkpoint()?;}values.put(key,GltfJson::Array(array.take()),read)?;},
  Pending::Object(key,children)=>{let mut object=Object(read.reserve(children.len())?);for(name,child)in children{object.0.push((name,values.take(child)?));read.checkpoint()?;}values.put(key,GltfJson::Object(std::mem::take(&mut object.0)),read)?;}
 }}
 if values.0.iter().filter(|(_,value)|value.is_some()).count()!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"GLTF extras unowned values remain"))}values.take(root)
}
pub(super) fn project(write:&mut Write<'_,'_>,value:Option<&GltfJson>)->Result<Option<i64>, ValueError>{
 let Some(value)=value else{return Ok(None)};let root=write.json_id()?;let mut pending=write.projection.allocate_frontier(1)?;pending.push((root,value));let mut position=0;
 while position<pending.len(){let(key,value)=pending[position];position+=1;
  match value{
   GltfJson::Null=>write.float_key("gltf_json_value",key,&[Cell::Text("null"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?,
   GltfJson::Bool(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("boolean"),Cell::Integer(i64::from(*value)),Cell::Null,Cell::Null],NUMBER)?,
   GltfJson::Number(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("number"),Cell::Null,Cell::Real(*value),Cell::Null],NUMBER)?,
   GltfJson::String(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("string"),Cell::Null,Cell::Null,Cell::Text(value)],NUMBER)?,
   GltfJson::Array(values)=>{write.check(values.len())?;write.float_key("gltf_json_value",key,&[Cell::Text("array"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?;for(position,value)in values.iter().enumerate(){let child=write.json_id()?;write.insert("gltf_json_array_element",&[Cell::Integer(key),ordinal(position)?,Cell::Integer(child)])?;write.projection.push_frontier(&mut pending,(child,value))?;}},
   GltfJson::Object(values)=>{write.check(values.len())?;write.float_key("gltf_json_value",key,&[Cell::Text("object"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?;for(position,(name,value))in values.iter().enumerate(){let child=write.json_id()?;write.insert("gltf_json_object_member",&[Cell::Integer(key),ordinal(position)?,Cell::Text(name),Cell::Integer(child)])?;write.projection.push_frontier(&mut pending,(child,value))?;}}
  }
 }Ok(Some(root))
}

