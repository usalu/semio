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
 fn from_native(value:NativeLabelTerminologies)->Self{Self{cells:[LabelRow([Cow::Owned(value.native.en),Cow::Owned(value.native.de)]),LabelRow([Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)])]}}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for LocalizedLabel{
 type Receiver=NativeMappedReceiver<I,NativeLabelTerminologies,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
