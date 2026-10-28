//! 🌐️ Native labels receive every authored language and terminology before adopting their original text owners.
use super::*;
use semio_framework_value_native_owned::native_decoding::{NativeReceive,owned::{NativeReceiveMap,NativeMappedReceiver,tree::NativeValueSource}};
#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
pub struct NativeLabelLanguages{en:String,de:String}
#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned,semio_framework_value::NativeReceive)]
pub struct NativeLabelTerminologies{native:NativeLabelLanguages,reuse:NativeLabelLanguages}
impl NativeReceiveMap<NativeLabelTerminologies> for LocalizedLabel{
 fn from_native(value:NativeLabelTerminologies)->Self{Self{cells:[[Cow::Owned(value.native.en),Cow::Owned(value.native.de)],[Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)]]}}
}
impl<I:NativeValueSource> NativeReceive<I> for LocalizedLabel{
 type Receiver=NativeMappedReceiver<I,NativeLabelTerminologies,Self>;
 fn receive_birth_demand()->semio_framework_value::RetirementDemand{Self::Receiver::birth().expect("localized label has one fixed original terminology ancestor")}
 fn begin_receive(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::Receiver,semio_framework_value::RetainedCloneProgress),ValueError>{Self::Receiver::begin(grant)}
}
