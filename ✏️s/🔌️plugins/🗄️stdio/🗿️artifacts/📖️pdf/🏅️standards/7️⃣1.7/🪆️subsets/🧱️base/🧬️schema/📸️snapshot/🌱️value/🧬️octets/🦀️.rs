//! 🧬️ Explicit COS intrinsic byte payloads preserve their owner-tagged logical shape.
use super::*;
use pack::value::{DslValue as V,ToValue,FromValue,ValueError as E};
impl ToValue for PdfObject{
    fn to_value(&self)->V{
        let(tag,payload)=match self{
            Self::Null=>return V::Object(vec![("kind".into(),V::String("null".into()))]),
            Self::Bool(value)=>("bool",value.to_value()),Self::Int(value)=>("int",value.to_value()),Self::Real(value)=>("real",value.to_value()),
            Self::Str(bytes)=>("str",pack::value::bytes::to_value(bytes)),Self::Name(value)=>("name",value.to_value()),Self::Array(value)=>("array",value.to_value()),Self::Dict(value)=>("dict",value.to_value()),Self::Ref(value)=>("ref",value.to_value()),
            Self::Stream{dict,data,filters}=>return V::Object(vec![("kind".into(),V::String("stream".into())),("dict".into(),dict.to_value()),("data".into(),pack::value::bytes::to_value(data)),("filters".into(),filters.to_value())]),
        };
        let mut fields=vec![("kind".into(),V::String(tag.into()))];match payload{V::Object(values)=>fields.extend(values),value=>fields.push(("value".into(),value))};V::Object(fields)
    }
    fn to_value_controlled(&self,control:&mut pack::value::NativeEncodeControl<'_>)->Result<V,E>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            let(tag,count)=match self{Self::Null=>("null",1),Self::Real(_)=>("real",4),Self::Ref(_)=>("ref",3),Self::Stream{..}=>("stream",4),Self::Bool(_)=>("bool",2),Self::Int(_)=>("int",2),Self::Str(_)=>("str",2),Self::Name(_)=>("name",2),Self::Array(_)=>("array",2),Self::Dict(_)=>("dict",2)};
            control.begin_stage(count).map_err(E::new)?;let mut fields=V::object_encoding_controlled(count,control)?;cos_output(fields.get_mut(),"kind",&tag,control)?;
            match self{
                Self::Null=>{},
                Self::Bool(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Int(value)=>cos_output(fields.get_mut(),"value",value,control)?,
                Self::Name(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Array(value)=>cos_output(fields.get_mut(),"value",value,control)?,Self::Dict(value)=>cos_output(fields.get_mut(),"value",value,control)?,
                Self::Str(bytes)=>cos_output_bytes(fields.get_mut(),"value",bytes,control)?,
                Self::Real(value)=>{cos_output(fields.get_mut(),"negative",&value.negative,control)?;cos_output(fields.get_mut(),"coefficient",&value.coefficient,control)?;cos_output(fields.get_mut(),"scale",&value.scale,control)?;},
                Self::Ref(value)=>{cos_output(fields.get_mut(),"num",&value.num,control)?;cos_output(fields.get_mut(),"gen",&value.gen,control)?;},
                Self::Stream{dict,data,filters}=>{cos_output(fields.get_mut(),"dict",dict,control)?;cos_output_bytes(fields.get_mut(),"data",data,control)?;cos_output(fields.get_mut(),"filters",filters,control)?;},
            }
            Ok(V::Object(fields.take()))
        }))
    }

}
impl FromValue for PdfObject{
    fn from_value(value:V)->Result<Self,E>{
        let fields=value.into_object()?;let tag=fields.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str()).ok_or_else(||E::new("PDF COS object requires a kind"))?;
        let get=|key:&str|fields.iter().find(|(name,_)|name==key).map(|(_,value)|value.clone()).ok_or_else(||E::new(format!("PDF COS object requires {key}")));
        if tag=="stream"{if fields.iter().any(|(key,_)|!matches!(key.as_str(),"kind"|"dict"|"data"|"filters")){return Err(E::new("PDF COS stream has an undeclared field"));}return Ok(Self::Stream{dict:Vec::<PdfDictEntry>::from_value(get("dict")?)?,data:pack::value::bytes::from_value(get("data")?)?,filters:Vec::<PdfStreamFilter>::from_value(get("filters")?)?});}
        let payload=||get("value");
        Ok(match tag{
            "null"=>Self::Null,"bool"=>Self::Bool(bool::from_value(payload()?)?),"int"=>Self::Int(i64::from_value(payload()?)?),"real"=>Self::Real(PdfDecimal::from_value(V::Object(fields.iter().filter(|(key,_)|key!="kind").cloned().collect()))?),
            "str"=>Self::Str(pack::value::bytes::from_value(payload()?)?),"name"=>Self::Name(String::from_value(payload()?)?),"array"=>Self::Array(Vec::<PdfObject>::from_value(payload()?)?),"dict"=>Self::Dict(Vec::<PdfDictEntry>::from_value(payload()?)?),
            "ref"=>Self::Ref(ObjRef::from_value(V::Object(fields.iter().filter(|(key,_)|key!="kind").cloned().collect()))?),
            _=>return Err(E::new("unknown PDF COS object kind")),
        })
    }
    fn from_value_controlled(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Self,E>{
        control.scoped_depth(64,|control|control.scoped_stage(|control|{
            let V::Object(fields)=value else{return Err(E::new("PDF COS object requires a tagged object"))};
            let tag=fields.iter().find(|(key,_)|key=="kind").and_then(|(_,value)|value.as_str()).ok_or_else(||E::new("PDF COS object requires a kind"))?;
            let keys:&[&str]=match tag{"null"=>&["kind"],"real"=>&["kind","negative","coefficient","scale"],"ref"=>&["kind","num","gen"],"stream"=>&["kind","dict","data","filters"],"bool"|"int"|"str"|"name"|"array"|"dict"=>&["kind","value"],_=>return Err(E::new("unknown PDF COS object kind"))};
            if fields.len()!=keys.len()||keys.iter().any(|key|fields.iter().filter(|(name,_)|name==key).count()!=1){return Err(E::new("PDF COS object has missing, duplicate or undeclared fields"))}
            control.begin_stage(keys.len()).map_err(E::new)?;control.step().map_err(E::new)?;
            let get=|key:&str|fields.iter().find(|(name,_)|name==key).map(|(_,value)|value).ok_or_else(||E::new(format!("PDF COS object requires {key}")));
            Ok(match tag{
                "null"=>Self::Null,
                "bool"=>Self::Bool(cos_owned::<bool>(get("value")?,control)?.take()),
                "int"=>Self::Int(cos_owned::<i64>(get("value")?,control)?.take()),
                "str"=>{let data=cos_bytes(get("value")?,control)?;control.step().map_err(E::new)?;Self::Str(data)},
                "name"=>Self::Name(cos_owned::<String>(get("value")?,control)?.take()),
                "array"=>Self::Array(cos_owned::<Vec<Self>>(get("value")?,control)?.take()),
                "dict"=>Self::Dict(cos_owned::<Vec<PdfDictEntry>>(get("value")?,control)?.take()),
                "real"=>{let negative=cos_owned::<bool>(get("negative")?,control)?;let coefficient=cos_owned::<String>(get("coefficient")?,control)?;let scale=cos_owned::<u32>(get("scale")?,control)?;Self::Real(PdfDecimal{negative:negative.take(),coefficient:coefficient.take(),scale:scale.take()})},
                "ref"=>{let num=cos_owned::<u32>(get("num")?,control)?;let gen=cos_owned::<u16>(get("gen")?,control)?;Self::Ref(ObjRef{num:num.take(),gen:gen.take()})},
                "stream"=>{let dict=cos_owned::<Vec<PdfDictEntry>>(get("dict")?,control)?;let data=cos_bytes(get("data")?,control)?;control.step().map_err(E::new)?;let filters=cos_owned::<Vec<PdfStreamFilter>>(get("filters")?,control)?;Self::Stream{dict:dict.take(),data,filters:filters.take()}},
                _=>unreachable!(),
            })
        }))
    }
    fn retire_decoded(self){
        let mut pending=vec![self];while let Some(value)=pending.pop(){match value{
            Self::Array(mut values)=>pending.append(&mut values),Self::Dict(values)|Self::Stream{dict:values,..}=>pending.extend(values.into_iter().map(|entry|entry.value)),
            Self::Null|Self::Bool(_)|Self::Int(_)|Self::Real(_)|Self::Str(_)|Self::Name(_)|Self::Ref(_)=>{},
        }}
    }
    fn edit_value_at_path(&mut self,path:&[&str],edit:pack::value::ValueEdit)->Result<(),E>{pack::value::edit_through_value(self,path,edit)}
}
/// 🪪️ The document ID is exactly two independently owned intrinsic octet strings.
pub(crate)fn document_id_to_value(value:&Option<[Vec<u8>;2]>)->V{value.as_ref().map_or(V::Null,|value|V::Array(vec![pack::value::bytes::to_value(&value[0]),pack::value::bytes::to_value(&value[1])]))}
pub(crate)fn document_id_from_value(value:V)->Result<Option<[Vec<u8>;2]>,E>{match value{V::Null=>Ok(None),V::Array(mut values)if values.len()==2=>{let right=pack::value::bytes::from_value(values.pop().unwrap())?;let left=pack::value::bytes::from_value(values.pop().unwrap())?;Ok(Some([left,right]))},_=>Err(E::new("PDF document ID requires exactly two intrinsic octet strings"))}}

fn cos_owned<T:FromValue>(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,E>{let value=T::from_value_controlled(value,control)?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(E::new)?;Ok(owner)}
fn cos_bytes(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Vec<u8>,E>{match value{V::Bytes(bytes)=>control.copy_bytes(bytes).map_err(E::new),V::Array(_)=>Vec::<u8>::from_value_controlled(value,control),_=>Err(E::new("PDF intrinsic octet string requires bytes or canonical octets"))}}
/// 🪪️ Borrowed document ID admission copies its two explicit octet fields under one control.
pub(crate)fn document_id_from_value_controlled(value:&V,control:&mut pack::value::NativeDecodeControl<'_>)->Result<Option<[Vec<u8>;2]>,E>{control.scoped_stage(|control|match value{V::Null=>Ok(None),V::Array(values)if values.len()==2=>{control.begin_stage(2).map_err(E::new)?;let left=cos_bytes(&values[0],control)?;control.step().map_err(E::new)?;let right=cos_bytes(&values[1],control)?;control.step().map_err(E::new)?;Ok(Some([left,right]))},_=>Err(E::new("PDF document ID requires exactly two intrinsic octet strings"))})}

fn cos_output<T:ToValue+?Sized>(fields:&mut Vec<(String,V)>,key:&str,value:&T,control:&mut pack::value::NativeEncodeControl<'_>)->Result<(),E>{
    let key_owned=control.copy_text(key).map_err(E::new)?;let value=value.to_value_controlled(control).map_err(|error|error.under(key))?;fields.push((key_owned,value));control.step().map_err(E::new)
}
fn cos_output_bytes(fields:&mut Vec<(String,V)>,key:&str,bytes:&[u8],control:&mut pack::value::NativeEncodeControl<'_>)->Result<(),E>{
    let key_owned=control.copy_text(key).map_err(E::new)?;let value=pack::value::bytes::to_value_controlled(bytes,control).map_err(|error|error.under(key))?;fields.push((key_owned,value));control.step().map_err(E::new)
}
/// 🪪️ Projects the two intrinsic document-ID octets through one admitted native output frontier.
pub(crate)fn document_id_to_value_controlled(value:&Option<[Vec<u8>;2]>,control:&mut pack::value::NativeEncodeControl<'_>)->Result<V,E>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|match value{
        None=>().to_value_controlled(control),Some(values)=>{control.begin_stage(2).map_err(E::new)?;let mut output=Vec::<V>::guard_decoded(control.allocate_vec(2).map_err(E::new)?);for(index,value)in values.iter().enumerate(){output.get_mut().push(pack::value::bytes::to_value_controlled(value,control).map_err(|error|error.under(index))?);control.step().map_err(E::new)?;}Ok(V::Array(output.take()))}
    }))
}

#[cfg(test)]
#[path="🧪️tests/🛫️output/🦀️.rs"]
mod output_tests;
