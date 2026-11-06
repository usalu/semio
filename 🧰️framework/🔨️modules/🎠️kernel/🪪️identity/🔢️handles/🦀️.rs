//! 🔢️ Artifact and window identities preserve their full canonical u128 decimal string across every codec.
use super::{ArtifactHandle,WindowHandle,CapabilityToken};
use semio_framework_value::{ValueError,ValueRefusalKind,native_decoding::{NativeReceive,owned::tree::{NativeValueSource,NativeScalarValue,NativeScalarReceiver}}};
pub(super) fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native handle requires canonical decimal u128 string")}
pub(super) fn parse(text:&str)->Result<u128,ValueError>{if text.is_empty()||text.len()>39||text.len()>1&&text.as_bytes()[0]==b'0'||!text.bytes().all(|byte|byte.is_ascii_digit()){return Err(invalid())}text.parse().map_err(|_|invalid())}
struct Decimal;
impl<'de> serde::de::Visitor<'de> for Decimal{type Value=u128;fn expecting(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("canonical decimal u128 string")}fn visit_str<E:serde::de::Error>(self,value:&str)->Result<u128,E>{parse(value).map_err(|_|E::custom("native handle requires canonical decimal u128 string"))}}
macro_rules! handle{($ty:ident)=>{
 impl std::str::FromStr for $ty{type Err=ValueError;fn from_str(text:&str)->Result<Self,Self::Err>{parse(text).map(Self)}}
 impl serde::Serialize for $ty{fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serializer.collect_str(&self.0)}}
 impl<'de> serde::Deserialize<'de> for $ty{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{deserializer.deserialize_str(Decimal).map(Self)}}
 impl<I:NativeValueSource+?Sized> NativeScalarValue<I> for $ty{fn read(input:&I)->Result<Self,ValueError>{parse(input.text()?).map(Self)}fn read_copy_bytes(_:&I)->Result<usize,ValueError>{Ok(128)}}
 impl<I:NativeValueSource+?Sized> NativeReceive<I> for $ty{type Receiver=NativeScalarReceiver<I,Self>;fn receiver()->Self::Receiver{Self::Receiver::new()}}
};}
handle!(ArtifactHandle);
handle!(WindowHandle);
handle!(CapabilityToken);
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
