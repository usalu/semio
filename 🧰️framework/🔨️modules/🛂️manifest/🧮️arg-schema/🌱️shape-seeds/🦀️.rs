//! 🌱️ Cold schema-owned seed compilation preserves declared defaults without admitting domain mutations.
use crate::{ActionArgDef,ArgSchema};
use semio_framework_value::DslValue;
/// 🌿️ One static schema node has one draft seed, addressed in its original descriptor tree.
#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
#[serde(rename_all="camelCase")]
#[value(rename_all="camelCase")]
pub struct ActionArgShapeSeed{
 pub schema_path:Vec<usize>,pub value:DslValue,
 #[serde(default,skip_serializing_if="Option::is_none")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub key:Option<String>,
 #[serde(default,skip_serializing_if="Vec::is_empty")]
 #[value(default,skip_serializing_if="Vec::is_empty")]
 pub children:Vec<usize>,
 #[serde(default,skip_serializing_if="Option::is_none")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub variant_selector:Option<usize>,
 #[serde(default,skip_serializing_if="Option::is_none")]
 #[value(default,skip_serializing_if="Option::is_none")]
 pub singleton:Option<DslValue>,
}
/// 🔀️ A variant selector names every actual group; ordinary section groups remain ordinary sections.
pub fn action_arg_variant_selector(fields:&[ActionArgDef])->Option<&ActionArgDef>{
 if !fields.iter().any(|field|field.group.is_some()){return None}
 fields.iter().filter(|field|field.group.is_none()).find(|field|matches!(&field.schema,ArgSchema::String{options,..}if fields.iter().filter_map(|field|field.group.as_deref()).all(|group|options.iter().any(|option|option.value==group))))
}
/// 🎁️ Explicit whole-input defaults take precedence over its schema-derived draft.
pub fn action_arg_shape_seed(input:&ActionArgDef)->DslValue{input.default.clone().unwrap_or_else(||arg_schema_shape_seed(&input.schema))}
/// 🌱️ Cold compilation chooses only the selected required variant and preserves existing number/reference defaults.
pub fn arg_schema_shape_seed(schema:&ArgSchema)->DslValue{
 let number=|min:Option<f64>,max:Option<f64>,exclusive:bool,step:Option<f64>,integer:bool|{
  let low=min.map_or_else(||max.map_or(0.0,|max|max.min(0.0)),|min|if exclusive{min+step.filter(|step|*step>0.0).unwrap_or(1.0)}else{min});
  DslValue::json_number(if integer{low.ceil()}else{low})
 };
 match schema{
  ArgSchema::Object{fields}=>{
   let selector=action_arg_variant_selector(fields);
   let selected=selector.and_then(|selector|selector.default.as_ref().and_then(DslValue::as_str).or_else(||match &selector.schema{ArgSchema::String{options,..}=>options.first().map(|option|option.value.as_str()),_=>None}));
   DslValue::Object(fields.iter().filter(|input|input.required&&(selector.is_none()||input.group.is_none()||input.group.as_deref()==selected)).map(|input|(input.key(),action_arg_shape_seed(input))).collect())
  },
  ArgSchema::Array{items,min_items,..}=>DslValue::Array((0..min_items.unwrap_or(0)).map(|_|arg_schema_shape_seed(items)).collect()),
  ArgSchema::Number{min,min_exclusive,max,step,integer,soft_min,..}=>number(soft_min.or(*min),*max,*min_exclusive&&soft_min.is_none(),*step,*integer),
  ArgSchema::String{options,..}=>DslValue::String(options.first().map(|option|option.value.clone()).unwrap_or_default()),
  ArgSchema::Boolean=>DslValue::Bool(false),
  ArgSchema::Vector{dims,min,max,..}=>DslValue::Array((0..*dims).map(|_|number(*min,*max,false,None,false)).collect()),
  ArgSchema::Reference{many:true,..}=>DslValue::Array(Vec::new()),
  ArgSchema::Reference{..}=>DslValue::String(String::new()),
  ArgSchema::Any=>DslValue::Null,
 }
}
/// 🗺️ Descriptor input indices, object field indices and array item zeroes bind every static seed to its schema node.
pub fn action_arg_shape_seeds(inputs:&[ActionArgDef])->Vec<ActionArgShapeSeed>{
 fn node(schema:&ArgSchema,path:&mut Vec<usize>,key:Option<String>,value:DslValue,output:&mut Vec<ActionArgShapeSeed>){
  let variant_selector=match schema{ArgSchema::Object{fields}=>action_arg_variant_selector(fields).and_then(|selector|fields.iter().position(|field|std::ptr::eq(field,selector))),_=>None};
  let singleton=match schema{ArgSchema::Array{items,..}=>Some(DslValue::Array(vec![arg_schema_shape_seed(items)])),ArgSchema::Reference{many:true,..}=>Some(DslValue::Array(vec![DslValue::String(String::new())])),_=>None};
  let own=output.len();output.push(ActionArgShapeSeed{schema_path:path.clone(),value,key,children:Vec::new(),variant_selector,singleton});
  match schema{
   ArgSchema::Object{fields}=>for(index,input)in fields.iter().enumerate(){let child=output.len();output[own].children.push(child);path.push(index);node(&input.schema,path,Some(input.key()),action_arg_shape_seed(input),output);path.pop();},
   ArgSchema::Array{items,..}=>{let child=output.len();output[own].children.push(child);path.push(0);node(items,path,None,arg_schema_shape_seed(items),output);path.pop();},
   _=>{},
  }
 }
 let mut output=Vec::new();
 for(index,input)in inputs.iter().enumerate(){node(&input.schema,&mut vec![index],Some(input.key()),action_arg_shape_seed(input),&mut output)}
 output
}
/// 🧾️ Build emission admits only complete, acyclic descriptor-ordered edges and exact decoded key/selector facts.
pub fn validate_action_arg_shape_seeds(inputs:&[ActionArgDef],rows:&[ActionArgShapeSeed])->Result<(),&'static str>{
 fn node(schema:&ArgSchema,path:&mut Vec<usize>,key:Option<&str>,rows:&[ActionArgShapeSeed],cursor:&mut usize)->Result<(),&'static str>{
  let own=*cursor;let row=rows.get(own).ok_or("seed node absent")?;
  if row.schema_path!=*path||row.key.as_deref()!=key{return Err("seed path or decoded key differs")}
  let selector=match schema{ArgSchema::Object{fields}=>action_arg_variant_selector(fields).and_then(|selector|fields.iter().position(|field|std::ptr::eq(field,selector))),_=>None};
  if row.variant_selector!=selector{return Err("seed variant selector differs")}
  let children=match schema{ArgSchema::Object{fields}=>fields.len(),ArgSchema::Array{..}=>1,_=>0};
  if row.children.len()!=children{return Err("seed child count differs")}
  match schema{
   ArgSchema::Array{..}=>{},
   ArgSchema::Reference{many:true,..}=>if !matches!(&row.singleton,Some(DslValue::Array(values))if values.len()==1&&matches!(&values[0],DslValue::String(value)if value.is_empty())){return Err("reference singleton differs")},
   _=>if row.singleton.is_some(){return Err("non-collection singleton present")},
  }
  *cursor+=1;
  match schema{
   ArgSchema::Object{fields}=>for(index,input)in fields.iter().enumerate(){if row.children[index]!=*cursor||*cursor<=own{return Err("seed child edge differs")}path.push(index);node(&input.schema,path,Some(&input.key()),rows,cursor)?;path.pop();},
   ArgSchema::Array{items,..}=>{if row.children[0]!=*cursor||*cursor<=own{return Err("seed item edge differs")}
    if !matches!(&row.singleton,Some(DslValue::Array(values))if values.len()==1&&rows.get(*cursor).is_some_and(|child|values[0]==child.value)){return Err("array singleton differs")}
    path.push(0);node(items,path,None,rows,cursor)?;path.pop();},
   _=>{},
  }Ok(())
 }
 let mut cursor=0;
 for(index,input)in inputs.iter().enumerate(){node(&input.schema,&mut vec![index],Some(&input.key()),rows,&mut cursor)?}
 if cursor!=rows.len(){return Err("seed roster has extra nodes")}Ok(())
}
#[cfg(test)]
#[path="🧪️tests.rs"]
mod tests;
