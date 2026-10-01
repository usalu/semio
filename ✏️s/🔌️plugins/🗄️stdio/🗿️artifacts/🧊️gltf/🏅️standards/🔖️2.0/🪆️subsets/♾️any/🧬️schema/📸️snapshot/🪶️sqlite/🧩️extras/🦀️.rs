//! 🧩️ Only GLTF's declared extensions and extras own these ordered local JSON values.
use super::*;
const NUMBER:&[FloatColumn]=&[FloatColumn::Binary64(3)];

pub(super) fn project(write:&mut Write<'_,'_>,value:Option<&GltfJson>)->Result<Option<i64>,String>{
 let Some(value)=value else{return Ok(None)};let root=write.json_id()?;let mut pending=std::collections::VecDeque::from([(root,value)]);
 while let Some((key,value))=pending.pop_front(){
  match value{
   GltfJson::Null=>write.float_key("gltf_json_value",key,&[Cell::Text("null"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?,
   GltfJson::Bool(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("boolean"),Cell::Integer(i64::from(*value)),Cell::Null,Cell::Null],NUMBER)?,
   GltfJson::Number(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("number"),Cell::Null,Cell::Real(*value),Cell::Null],NUMBER)?,
   GltfJson::String(value)=>write.float_key("gltf_json_value",key,&[Cell::Text("string"),Cell::Null,Cell::Null,Cell::Text(value)],NUMBER)?,
   GltfJson::Array(values)=>{write.check(values.len())?;write.float_key("gltf_json_value",key,&[Cell::Text("array"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?;for(position,value)in values.iter().enumerate(){let child=write.json_id()?;write.insert("gltf_json_array_element",&[Cell::Integer(key),ordinal(position)?,Cell::Integer(child)])?;pending.push_back((child,value));}},
   GltfJson::Object(values)=>{write.check(values.len())?;write.float_key("gltf_json_value",key,&[Cell::Text("object"),Cell::Null,Cell::Null,Cell::Null],NUMBER)?;for(position,(name,value))in values.iter().enumerate(){let child=write.json_id()?;write.insert("gltf_json_object_member",&[Cell::Integer(key),ordinal(position)?,Cell::Text(name),Cell::Integer(child)])?;pending.push_back((child,value));}}
  }
 }Ok(Some(root))
}

enum Pending{Value(i64),Array(i64,Vec<i64>),Object(i64,Vec<(String,i64)>)}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>,root:i64)->Result<GltfJson,String>{
 let mut pending=vec![Pending::Value(root)];let mut values=BTreeMap::new();
 while let Some(step)=pending.pop(){match step{
  Pending::Value(key)=>{
   let row=read.key("gltf_json_value",key)?;if row.values.len()!=7{return Err("GLTF extras value column count differs".into());}let scalar=FloatRow::new(row,NUMBER)?;let kind=row.text(1)?;
   if (kind!="boolean"&&row.values[2]!=SqliteValue::Null)||(kind!="string"&&row.values[4]!=SqliteValue::Null)||(kind!="number"&&!scalar.is_null(3)?){return Err("GLTF extras contains unused typed scalar fields".into());}
   let value=match kind{
    "null"=>GltfJson::Null,
    "boolean"=>GltfJson::Bool(read.boolean(row,2)?),
    "number"=>{read.scalar()?;GltfJson::Number(scalar.real(3)?)},
    "string"=>GltfJson::String(read.text(row,4)?),
    "array"=>{let rows=read.rows("gltf_json_array_element",1,key,2)?;let mut children=Vec::with_capacity(rows.len());for row in rows{let child=row.integer(3)?;if child<1{return Err("GLTF extras child must be a positive identity".into());}children.push(child);}pending.push(Pending::Array(key,children.clone()));for child in children.into_iter().rev(){pending.push(Pending::Value(child));}continue;},
    "object"=>{let rows=read.rows("gltf_json_object_member",1,key,2)?;let mut children=Vec::with_capacity(rows.len());for row in rows{let child=row.integer(4)?;if child<1{return Err("GLTF extras child must be a positive identity".into());}children.push((read.text(row,3)?,child));}let ids:Vec<_>=children.iter().map(|(_,child)|*child).collect();pending.push(Pending::Object(key,children));for child in ids.into_iter().rev(){pending.push(Pending::Value(child));}continue;},
    _=>return Err("GLTF extras value kind is unknown".into())
   };values.insert(key,value);
  },
  Pending::Array(key,children)=>{let mut array=Vec::with_capacity(children.len());for child in children{array.push(values.remove(&child).ok_or("GLTF extras child is absent or multiply owned")?);}values.insert(key,GltfJson::Array(array));},
  Pending::Object(key,children)=>{let mut object=Vec::with_capacity(children.len());for(name,child)in children{object.push((name,values.remove(&child).ok_or("GLTF extras child is absent or multiply owned")?));}values.insert(key,GltfJson::Object(object));}
 }}
 let value=values.remove(&root).ok_or("GLTF extras root is missing")?;if !values.is_empty(){return Err("GLTF extras unowned values remain".into());}Ok(value)
}
