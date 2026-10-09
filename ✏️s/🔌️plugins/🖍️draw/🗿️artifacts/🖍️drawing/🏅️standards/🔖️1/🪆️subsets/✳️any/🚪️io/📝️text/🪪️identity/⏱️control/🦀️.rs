//! ⏱️ Identity preparation remains bound to its original caller cancellation lifetime.
use semio_framework_async::CancelToken;
use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress};
pub fn with_identity_control<T,E>(cancel:&CancelToken,maximum_bytes:usize,observe:&mut dyn FnMut(NativeEncodeProgress),operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
 let mut checkpoint=|progress|{observe(progress);!cancel.is_cancelled_now()};
 let mut control=NativeEncodeControl::new(maximum_bytes,&mut checkpoint);
 operation(&mut control)
}
