//! 📝️ Shared immutable UTF8 adopts the original byte allocation and exposes borrowed projections.
use crate::{paged::Utf8Text,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,RetirementCursor,shared::{SharedControlledRetirement,shared_retirement_allocation_bytes}},ValueError,ValueRefusalKind};
use std::{sync::Arc,mem::size_of};

#[derive(Clone,Debug,Default)]
pub struct SharedUtf8(Option<Arc<String>>);
impl SharedUtf8 {
    /// ♻️ Releases one original lease under independent full authority and returns its final unchanged String.
    pub fn close_original_lease(mut self,grant:RetainedCloneGrant)->Result<(Option<String>,RetainedCloneProgress),(ValueError,Self)> {
        if self.0.is_none(){return Ok((None,RetainedCloneProgress::default()));}
        let copied=size_of::<Option<String>>();let released=shared_retirement_allocation_bytes::<String>();
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"original UTF8 lease requires one admitted item"))}else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"original UTF8 lease requires admitted depth"))}else if copied>grant.maximum_copy_bytes{Some((ValueRefusalKind::WorkLimit,"original UTF8 lease transfer exceeds copy grant"))}else if released>grant.maximum_release_bytes{Some((ValueRefusalKind::OwnershipLimit,"original UTF8 lease backing exceeds release grant"))}else if Arc::weak_count(self.0.as_ref().unwrap())!=0{Some((ValueRefusalKind::OwnershipLimit,"original UTF8 lease retains weak backing"))}else{None};
        if let Some((kind,message))=refusal{return Err((ValueError::literal(kind,message),self));}
        let original=Arc::into_inner(self.0.take().unwrap());let final_lease=original.is_some();
        Ok((original,RetainedCloneProgress{copied_items:1,copied_bytes:if final_lease{copied}else{0},retained_capacity_bytes:0,released_bytes:if final_lease{released}else{0}}))
    }
    pub fn admission_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<String>()+size_of::<Self>(),capacity_bytes:shared_retirement_allocation_bytes::<String>(),depth:1,..Default::default()}}
    pub fn lease_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
    pub fn admit(original:String,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,String)> {
        let copied_bytes=size_of::<String>()+size_of::<Self>();
        let retained_capacity_bytes=shared_retirement_allocation_bytes::<String>();
        let refusal=if grant.maximum_items==0{Some((ValueRefusalKind::WorkLimit,"shared UTF8 requires one admitted item"))}else if grant.maximum_depth==0{Some((ValueRefusalKind::DepthLimit,"shared UTF8 requires admitted original depth"))}else if copied_bytes>grant.maximum_copy_bytes{Some((ValueRefusalKind::WorkLimit,"shared UTF8 original transfer exceeds admitted copy bytes"))}else if retained_capacity_bytes>grant.maximum_capacity_bytes{Some((ValueRefusalKind::OwnershipLimit,"shared UTF8 frame exceeds admitted capacity"))}else{None};
        if let Some((kind,message))=refusal{return Err((ValueError::literal(kind,message),original));}
        Ok((Self(Some(Arc::new(original))),RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes,released_bytes:0}))
    }
    pub fn admit_clone(&self,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError> {
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared UTF8 lease requires one admitted item"));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"shared UTF8 lease requires admitted original depth"));}
        if grant.maximum_copy_bytes<size_of::<Self>(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"shared UTF8 lease exceeds admitted copy bytes"));}
        Ok((Self(self.0.as_ref().map(Arc::clone)),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Self>(),..Default::default()}))
    }
    pub fn as_str(&self)->&str {self.0.as_ref().map_or("",|source|source.as_str())}
    pub fn as_bytes(&self)->&[u8] {self.as_str().as_bytes()}
    pub fn as_ptr(&self)->*const u8 {self.as_str().as_ptr()}
    pub fn len(&self)->usize {self.as_str().len()}
    pub fn is_empty(&self)->bool {self.as_str().is_empty()}
    pub fn has_owner(&self)->bool {self.0.is_some()}
    pub fn original_allocation_bytes(&self)->usize {self.0.as_ref().map_or(0,|source|shared_retirement_allocation_bytes::<String>()+source.capacity())}
}
impl PartialEq for SharedUtf8 {fn eq(&self,other:&Self)->bool{self.as_str()==other.as_str()}}
impl Eq for SharedUtf8 {}
impl PartialOrd for SharedUtf8 {fn partial_cmp(&self,other:&Self)->Option<std::cmp::Ordering>{Some(self.cmp(other))}}
impl Ord for SharedUtf8 {fn cmp(&self,other:&Self)->std::cmp::Ordering{self.as_str().cmp(other.as_str())}}
impl std::hash::Hash for SharedUtf8 {fn hash<H:std::hash::Hasher>(&self,state:&mut H){std::hash::Hash::hash(self.as_str(),state)}}
impl Utf8Text for SharedUtf8 {
    fn text_bytes(&self)->usize {self.len()}
    fn text_chunk_count(&self)->usize {self.as_str().text_chunk_count()}
    fn text_chunk(&self,index:usize)->Option<&str> {self.as_str().text_chunk(index)}
}
impl std::ops::Deref for SharedUtf8 {type Target=str;fn deref(&self)->&str{self.as_str()}}
impl AsRef<str> for SharedUtf8 {fn as_ref(&self)->&str{self.as_str()}}
impl std::fmt::Display for SharedUtf8 {fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.as_str().fmt(f)}}
impl From<String> for SharedUtf8 {fn from(original:String)->Self{Self(Some(Arc::new(original)))}}
impl From<&str> for SharedUtf8 {fn from(original:&str)->Self{Self::from(original.to_owned())}}
impl serde::Serialize for SharedUtf8 {fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serializer.serialize_str(self.as_str())}}
impl<'de> serde::Deserialize<'de> for SharedUtf8 {fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{<String as serde::Deserialize>::deserialize(deserializer).map(Self::from)}}
impl crate::ToValue for SharedUtf8 {fn to_value(&self)->crate::DslValue{crate::DslValue::String(self.as_str().to_owned())}}
impl crate::FromValue for SharedUtf8 {fn from_value(value:crate::DslValue)->Result<Self,ValueError>{<String as crate::FromValue>::from_value(value).map(Self::from)}}
impl RetireOwned for SharedUtf8 {
    fn retirement(self)->Box<dyn RetirementCursor>{self.0.map_or_else(||None::<Arc<String>>.retirement(),|source|Box::new(SharedControlledRetirement::lease(source)))}
    fn retirement_birth_bytes(&self)->Option<usize>{self.0.as_ref().map_or_else(||None::<Arc<String>>.retirement_birth_bytes(),|_|Some(size_of::<SharedControlledRetirement<String>>()))}
    fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
