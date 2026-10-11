//! 🪪️ Authored identity keys, typed commitment facts and explicit clone assignments.
use semio_framework_value::paged::PagedUtf8;
use semio_framework_value::FromValue;
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[cfg_attr(test,serde(try_from="DrawingIdentityWire"))]
pub struct DrawingIdentity {key:PagedUtf8<{usize::MAX}>}
impl DrawingIdentity {
 pub fn admit(key:PagedUtf8<{usize::MAX}>)->Result<Self,semio_framework_value::ValueError>{if key.is_empty(){return Err(invalid());}Ok(Self{key})}
 pub fn key(&self)->&PagedUtf8<{usize::MAX}>{&self.key}
 pub fn into_key(self)->PagedUtf8<{usize::MAX}>{self.key}
}
#[cfg(test)]
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DrawingIdentityWire {key:PagedUtf8<{usize::MAX}>}
#[cfg(test)]
impl TryFrom<DrawingIdentityWire> for DrawingIdentity {type Error=semio_framework_value::ValueError;fn try_from(value:DrawingIdentityWire)->Result<Self,Self::Error>{Self::admit(value.key)}}
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::ToValue,semio_framework_value::FromValue)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
#[value(rename_all="camelCase")]
#[cfg_attr(test,serde(rename_all="camelCase"))]
pub enum DrawingIdentityKind {Layer,Path,Group,Boolean,Trace,Shape,Text,Image,Svg,ImageAsset}
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
pub struct DrawingIdentityCommitment {pub kind:DrawingIdentityKind,pub digest:[u8;32]}
#[derive(Clone,Debug,PartialEq,semio_framework_value::ToValue,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned,semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(test,derive(serde::Serialize,serde::Deserialize))]
pub struct DrawingIdentityAssignment {pub source:PagedUtf8<{usize::MAX}>,pub target:PagedUtf8<{usize::MAX}>}

fn invalid()->semio_framework_value::ValueError{semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing identity record differs from its closed domain contract")}
fn fields<const N:usize>(value:semio_framework_value::DslValue,keys:[&str;N])->Result<[semio_framework_value::DslValue;N],semio_framework_value::ValueError>{
 let semio_framework_value::DslValue::Object(entries)=value else{return Err(invalid());};if entries.len()!=N{return Err(invalid());}let mut output=std::array::from_fn(|_|semio_framework_value::DslValue::Null);let mut seen=[false;N];
 for(key,value)in entries{let index=keys.iter().position(|candidate|*candidate==key).ok_or_else(invalid)?;if seen[index]{return Err(invalid());}seen[index]=true;output[index]=value;}if seen.iter().any(|seen|!*seen){return Err(invalid());}Ok(output)
}
impl semio_framework_value::FromValue for DrawingIdentity{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let[key]=fields(value,["key"])?;Self::admit(PagedUtf8::from_value(key)?)}
}
impl semio_framework_value::FromValue for DrawingIdentityCommitment{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let[kind,digest]=fields(value,["kind","digest"])?;Ok(Self{kind:DrawingIdentityKind::from_value(kind)?,digest:<[u8;32]>::from_value(digest)?})}
}
impl semio_framework_value::FromValue for DrawingIdentityAssignment{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let[source,target]=fields(value,["source","target"])?;let source=PagedUtf8::from_value(source)?;let target=PagedUtf8::from_value(target)?;if source.is_empty()||target.is_empty(){return Err(invalid());}Ok(Self{source,target})}
}
