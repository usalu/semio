//! 🧬️ Native schema identity takes its complete original text owner without another text allocation.
use super::{SchemaId,PayloadHash};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource}};
impl NativeReceiveMap<String> for SchemaId{fn from_native(value:String)->Self{Self(value)}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for SchemaId{
 type Receiver=NativeMappedReceiver<I,String,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}

impl NativeReceiveMap<[u8;32]> for PayloadHash{fn from_native(value:[u8;32])->Self{Self(value)}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PayloadHash{
 type Receiver=NativeMappedReceiver<I,[u8;32],Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
