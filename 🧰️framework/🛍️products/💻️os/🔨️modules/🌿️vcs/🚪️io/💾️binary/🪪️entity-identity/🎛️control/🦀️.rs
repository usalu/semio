//! 🎛️ Caller-owned identity admission retained across asynchronous authoring hops.
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,native_encoding::{NativeEncodeProgress,NativeEncodeContinuation}};

#[cfg(not(target_arch="wasm32"))]
type Observer<'a>=dyn FnMut(NativeEncodeProgress)->bool+Send+'a;
#[cfg(target_arch="wasm32")]
type Observer<'a>=dyn FnMut(NativeEncodeProgress)->bool+'a;

/// 🧵️ One caller observer and consuming ownership receipt fund every physical authoring stage.
pub struct EntityIdentityAuthority<'a>{continuation:Option<NativeEncodeContinuation>,observer:&'a mut Observer<'a>}
impl<'a> EntityIdentityAuthority<'a>{
    /// 🚦️ Starts admission with the caller's explicit ownership ceiling and cancellation authority.
    pub fn new(maximum_bytes:usize,observer:&'a mut Observer<'a>)->Result<Self,ValueError>{let continuation=NativeEncodeControl::new(maximum_bytes,observer).pause()?;Ok(Self{continuation:Some(continuation),observer})}
    /// ▶️ Continues the actual previously admitted operation with the current caller observer.
    pub fn resume(continuation:NativeEncodeContinuation,observer:&'a mut Observer<'a>)->Self{Self{continuation:Some(continuation),observer}}
    /// ⏸️ Returns the original cumulative receipt to its next publication or authoring owner.
    pub fn pause(mut self)->Result<NativeEncodeContinuation,ValueError>{self.continuation.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"identity authority lacks its cumulative receipt"))}
    /// 🪆️ Resumes only for synchronous encoding and retains all admitted charges on refusal.
    pub fn encode<T,E:From<ValueError>>(&mut self,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let receipt=self.continuation.take().ok_or_else(||E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"identity authority lacks its cumulative receipt")))?;
        let mut control=NativeEncodeControl::resume(receipt,self.observer).map_err(E::from)?;
        let result=operation(&mut control);
        self.continuation=Some(control.pause().map_err(E::from)?);
        result
    }
}
