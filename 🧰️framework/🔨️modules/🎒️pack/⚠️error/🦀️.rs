//! ⚠️ Unmounted canonical Pack producer-authority draft; no native runtime verdict.
use semio_framework_diagnostic::{FaultOrigin,TextError};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};
#[cfg(test)]
#[path="../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR:test_allocation::RequestedAllocator=test_allocation::RequestedAllocator;
#[path="🔌️capture/🦀️.rs"]
mod transport_capture;
pub use transport_capture::{OwnedTransportError,TransportAdmission,TransportCaptureRefusal,TransportCaptureStage,TransportReservation,reserve_transport,PackTransportContext,PackTransportPolicy,PackTransportPhase,PackTransportProgress,TransportContextRefusal,TransportContextRefusalCause,SharedTransportReservation};

/// 🔁️ The transport owner explicitly chooses whether one failure may be retried.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackRetryDisposition{Never,Transient}

/// 🔌️ The actual provider operation owns the external failure category.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackTransportCategory{NativeIo,HttpRequest,HttpBody}

/// 🪪️ External transport never invents a semantic Value refusal.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PackCauseKind{Refusal(ValueRefusalKind),Transport(PackTransportCategory)}

#[path="🪶️refusal/🦀️.rs"]
mod semantic_refusal;
pub use semantic_refusal::PackRefusal;
/// 🚨️ Transport capability is separate from statically semantic codec refusal.
#[derive(Debug,Clone,PartialEq)]
pub enum PackError{Refusal(PackRefusal),TransportFailure(OwnedTransportError)}
impl PackError{
 /// 🧭️ Preserves the producer's semantic kind or actual external operation category.
 pub const fn cause_kind(&self)->PackCauseKind{match self{Self::Refusal(error)=>PackCauseKind::Refusal(error.kind()),Self::TransportFailure(error)=>PackCauseKind::Transport(error.category())}}
 /// 🔎️ External transport has no Value refusal kind.
 pub const fn refusal_kind(&self)->Option<ValueRefusalKind>{match self{Self::Refusal(error)=>Some(error.kind()),Self::TransportFailure(_)=>None}}
 /// 📤️ A transport cause returns the same owned failure without copying its source.
 pub fn into_value_error(self)->Result<ValueError,Self>{match self{Self::Refusal(error)=>Ok(error.into_value_error()),error@Self::TransportFailure(_)=>Err(error)}}
}
impl std::fmt::Display for PackError{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Refusal(error)=>std::fmt::Display::fmt(error,formatter),Self::TransportFailure(error)=>write!(formatter,"io error: {error}")}}}
impl std::error::Error for PackError{fn source(&self)->Option<&(dyn std::error::Error+'static)>{match self{Self::Refusal(error)=>std::error::Error::source(error),Self::TransportFailure(error)=>Some(error.source_error())}}}
impl From<PackRefusal> for PackError{fn from(error:PackRefusal)->Self{Self::Refusal(error)}}
impl From<ValueError> for PackError{fn from(error:ValueError)->Self{Self::Refusal(error.into())}}
impl From<TextError> for PackError{fn from(error:TextError)->Self{Self::Refusal(error.into())}}
semio_framework_diagnostic::fault_from_error!(PackError,FaultOrigin::Module,"module.pack");
#[cfg(test)]
#[path="🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;
#[cfg(test)]
#[path="🧪️tests/📍️text-refusal/🦀️.rs"]
mod text_refusal_tests;

#[cfg(test)]
#[path="🧪️tests/🔁️value-projection/🦀️.rs"]
mod value_projection_tests;
