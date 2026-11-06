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
 fn mapping_demand(_:&NativeLabelTerminologies)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{semio_framework_value_native_owned::owned::native_mapping_demand::<NativeLabelTerminologies,Self>(std::mem::size_of::<NativeLabelTerminologies>())}
 fn mapping_work_demand(_:&NativeLabelTerminologies)->Result<u64,semio_framework_value::ValueError>{semio_framework_value_native_owned::owned::native_mapping_work_demand(4)}
 fn map_step(input:&mut Option<NativeLabelTerminologies>,output:&mut Option<Self>,g:semio_framework_value::RetainedCloneGrant,maximum:u64,_:Option<&mut Option<usize>>)->semio_framework_value::RetainedWorkOutcome{let d=semio_framework_value_native_owned::owned::native_mapping_demand::<NativeLabelTerminologies,Self>(std::mem::size_of::<NativeLabelTerminologies>()).unwrap();let q=semio_framework_value_native_owned::owned::native_mapping_work_demand(4).unwrap();semio_framework_value_native_owned::owned::map_native_owner(input,output,g,maximum,d,q,|value|Self{cells:[[Cow::Owned(value.native.en),Cow::Owned(value.native.de)],[Cow::Owned(value.reuse.en),Cow::Owned(value.reuse.de)]]})}
}
impl<I:NativeValueSource+?Sized> NativeReceive<I> for LocalizedLabel{
 type Receiver=NativeMappedReceiver<I,NativeLabelTerminologies,Self>;
 fn receiver()->Self::Receiver{NativeMappedReceiver::new()}
}
