//! 📚️ Providers lend actual source values only inside their originating paid callback.
use semio_framework_job::{OriginalNativeLease,OriginalTurnFailure,StepContext};
use semio_framework_pack_json::JsonWriteSource;
use semio_framework_value::{ValueError,ValueRefusalKind};
use super::GuestRelayNativeHome;
use crate::GuestRelayOutput;
/// 🔤️ A producer and the native parent lend their values under one actual Source callback.
pub trait GuestRelayJsonSource{fn with_home_json(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&dyn JsonWriteSource,&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>;}
/// 📎️ The caller retains this immutable view for the complete live future.
pub struct GuestRelayBorrowedJson<'a,S:JsonWriteSource>(pub &'a S);
impl<S:JsonWriteSource> GuestRelayJsonSource for GuestRelayBorrowedJson<'_,S>{
 fn with_home_json(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&dyn JsonWriteSource,&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{home.with_native(cx,|home,cx|receive(self.0,home,cx))}
}
/// 📄️ Byte producers lend their exact immutable backing together with the native parent.
pub trait GuestRelayByteSource{fn with_home_bytes(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&[u8],&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>;}
impl GuestRelayByteSource for GuestRelayOutput{
 fn with_home_bytes(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&[u8],&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{self.native.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"output lease is absent"))?.with_native_pair(home,cx,|output,home,cx|receive(output.bytes()?,home,cx))}
}
/// 📎️ A caller-owned immutable byte span stays borrowed for the full live future.
pub struct GuestRelayBorrowedBytes<'a>(pub &'a[u8]);
impl GuestRelayByteSource for GuestRelayBorrowedBytes<'_>{
 fn with_home_bytes(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&[u8],&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{home.with_native(cx,|home,cx|receive(self.0,home,cx))}
}

/// 🔄️ A controlled transformation lends a JSON view of an existing output without releasing its owner.
pub trait GuestRelayJsonTransform{fn with_json(&mut self,bytes:&[u8],cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&dyn JsonWriteSource,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>;}
/// 🔗️ Previous output and new native parent are borrowed together from the same Source bank.
pub struct GuestRelayTransformedOutput<'a,T:GuestRelayJsonTransform+?Sized>{pub output:&'a mut GuestRelayOutput,pub transform:&'a mut T}
impl<T:GuestRelayJsonTransform+?Sized> GuestRelayJsonSource for GuestRelayTransformedOutput<'_,T>{
 fn with_home_json(&mut self,home:&mut OriginalNativeLease<GuestRelayNativeHome>,cx:&mut StepContext<'_>,receive:&mut dyn FnMut(&dyn JsonWriteSource,&mut GuestRelayNativeHome,&mut StepContext<'_>)->Result<(),OriginalTurnFailure>)->Result<bool,OriginalTurnFailure>{
  let mut produced=false;let transform=&mut*self.transform;let borrowed=self.output.native.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"output lease is absent"))?.with_native_pair(home,cx,|output,home,cx|{transform.with_json(output.bytes()?,cx,&mut |source,cx|{produced=true;receive(source,home,cx)})?;Ok(())})?;Ok(borrowed&&produced)
 }
}
