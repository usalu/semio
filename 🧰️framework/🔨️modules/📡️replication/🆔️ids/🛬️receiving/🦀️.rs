//! 🧬️ Native schema identity takes its complete original text owner without another text allocation.
use super::{SchemaId,PayloadHash};
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource}};
impl NativeReceiveMap<String> for SchemaId{fn from_native(value:String)->Self{Self(value)}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for SchemaId{
 type Receiver=NativeMappedReceiver<I,String,Self>;
 fn receive_birth_demand()->semio_framework_value::RetirementDemand{Self::Receiver::birth().expect("schema identity has one fixed native text ancestor")}
 fn begin_receive(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::Receiver,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{Self::Receiver::begin(grant)}
}

impl NativeReceiveMap<[u8;32]> for PayloadHash{fn from_native(value:[u8;32])->Self{Self(value)}}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for PayloadHash{
 type Receiver=NativeMappedReceiver<I,[u8;32],Self>;
 fn receive_birth_demand()->semio_framework_value::RetirementDemand{Self::Receiver::birth().expect("payload hash has one fixed native octet ancestor")}
 fn begin_receive(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::Receiver,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{Self::Receiver::begin(grant)}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
