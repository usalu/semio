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
    fn edit_value_at_path(&mut self,path:&[&str],edit:pack::value::ValueEdit)->Result<(),E>{pack::value::edit_through_value(self,path,edit)}
}
/// 🪪️ The document ID is exactly two independently owned intrinsic octet strings.
pub(crate)fn document_id_to_value(value:&Option<[Vec<u8>;2]>)->V{value.as_ref().map_or(V::Null,|value|V::Array(vec![pack::value::bytes::to_value(&value[0]),pack::value::bytes::to_value(&value[1])]))}
pub(crate)fn document_id_from_value(value:V)->Result<Option<[Vec<u8>;2]>,E>{match value{V::Null=>Ok(None),V::Array(mut values)if values.len()==2=>{let right=pack::value::bytes::from_value(values.pop().unwrap())?;let left=pack::value::bytes::from_value(values.pop().unwrap())?;Ok(Some([left,right]))},_=>Err(E::new("PDF document ID requires exactly two intrinsic octet strings"))}}
