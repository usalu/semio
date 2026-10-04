//! 🪶️ Semantic container refusals have no external transport variant.
use semio_framework_diagnostic::{FaultOrigin,TextError};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::list::{PagedListError,PagedListAllocationError,PagedListRefusalKind};
use super::{PackTransportCategory,PackRetryDisposition,TransportCaptureRefusal};
#[derive(Debug,Clone,PartialEq)]
pub enum PackRefusal{
 BadMagic,
 UnsupportedVersion{major:u16,minor:u16},
 UnknownRequiredFlags(u32),
 Truncated(u64),
 ChecksumMismatch{segment:&'static str,offset:u64},
 ContentHashMismatch,
 LimitExceeded{kind:ValueRefusalKind,limit:&'static str},
 RetainedMalformed{kind:ValueRefusalKind,what:&'static str,offset:u64,detail:&'static str},
 RetainedAllocation{kind:ValueRefusalKind,allocated_bytes:usize,what:&'static str,offset:u64,detail:&'static str},
 Malformed{kind:ValueRefusalKind,what:&'static str,offset:u64,detail:String},
 NonCanonical(&'static str),
 UnsupportedCodec(u8),
 ValueRefusal(ValueError),
 TextRefusal(TextError),
 Io{error:ValueError,retry:PackRetryDisposition},
 TransportAdmission{category:PackTransportCategory,refusal:TransportCaptureRefusal},
}
impl PackRefusal{
 /// 📋️ Preserves the lower borrowed source's kind and static reason.
 pub const fn from_paged_refusal(error:PagedListError,what:&'static str,offset:u64)->Self{
  Self::RetainedMalformed{kind:paged_kind(error.kind),what,offset,detail:error.reason}
 }
 /// 🧱️ Preserves the actual allocated byte witness alongside the borrowed cause.
 pub const fn from_paged_allocation(error:PagedListAllocationError,what:&'static str,offset:u64)->Self{
  Self::RetainedAllocation{kind:paged_kind(error.kind),allocated_bytes:error.allocated_bytes,what,offset,detail:error.reason}
 }
 /// 🧭️ Reads intrinsic syntax authority or the actual mandatory producer-authored kind.
 pub const fn kind(&self)->ValueRefusalKind{
  match self{
   Self::TransportAdmission{refusal,..}=>(refusal.kind),
   Self::BadMagic|Self::Truncated(_)|Self::ChecksumMismatch{..}|Self::ContentHashMismatch|Self::NonCanonical(_)=>(ValueRefusalKind::InvalidValue),
   Self::UnsupportedVersion{..}|Self::UnknownRequiredFlags(_)|Self::UnsupportedCodec(_)=>(ValueRefusalKind::UnsupportedOwner),
   Self::LimitExceeded{kind,..}|Self::RetainedMalformed{kind,..}|Self::RetainedAllocation{kind,..}|Self::Malformed{kind,..}=>(*kind),
   Self::ValueRefusal(error)|Self::Io{error,..}=>(error.kind),
   Self::TextRefusal(error)=>(error.kind),
  }
 }
 /// 🔎️ Returns a Value refusal only when the producer supplied semantic authority.
 pub fn into_value_error(self)->ValueError{let kind=self.kind();match self{Self::ValueRefusal(error)|Self::Io{error,..}=>error,Self::TextRefusal(error)=>ValueError::new(error.kind,error.message),error=>ValueError::new(kind,error.to_string())}}
}
const fn paged_kind(kind:PagedListRefusalKind)->ValueRefusalKind{
 match kind{PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated}
}
impl std::fmt::Display for PackRefusal{
 fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
  match self{
   Self::BadMagic=>formatter.write_str("bad magic"),
   Self::UnsupportedVersion{major,minor}=>write!(formatter,"unsupported version {major}.{minor}"),
   Self::UnknownRequiredFlags(flags)=>write!(formatter,"unknown required feature bits {flags:#x}"),
   Self::Truncated(offset)=>write!(formatter,"truncated at offset {offset}"),
   Self::ChecksumMismatch{segment,offset}=>write!(formatter,"checksum mismatch in {segment} at offset {offset}"),
   Self::ContentHashMismatch=>formatter.write_str("content hash mismatch"),
   Self::LimitExceeded{limit,..}=>write!(formatter,"limit exceeded: {limit}"),
   Self::RetainedMalformed{what,offset,detail,..}|Self::RetainedAllocation{what,offset,detail,..}=>write!(formatter,"malformed {what} at offset {offset}: {detail}"),
   Self::Malformed{what,offset,detail,..}=>write!(formatter,"malformed {what} at offset {offset}: {detail}"),
   Self::NonCanonical(detail)=>write!(formatter,"non-canonical encoding: {detail}"),
   Self::UnsupportedCodec(codec)=>write!(formatter,"unsupported codec {codec}"),
   Self::ValueRefusal(error)=>write!(formatter,"schema error: {error}"),
   Self::TextRefusal(error)=>write!(formatter,"schema error: {error}"),
   Self::Io{error,..}=>write!(formatter,"io error: {error}"),
   Self::TransportAdmission{refusal,..}=>write!(formatter,"transport admission: {refusal}"),
  }
 }
}
impl std::error::Error for PackRefusal{
 fn source(&self)->Option<&(dyn std::error::Error+'static)>{
  match self{Self::ValueRefusal(error)|Self::Io{error,..}=>Some(error),Self::TextRefusal(error)=>Some(error),Self::TransportAdmission{refusal,..}=>Some(refusal),_=>None}
 }
}
impl From<ValueError> for PackRefusal{fn from(error:ValueError)->Self{Self::ValueRefusal(error)}}
impl From<TextError> for PackRefusal{fn from(error:TextError)->Self{Self::TextRefusal(error)}}
semio_framework_diagnostic::fault_from_error!(PackRefusal,FaultOrigin::Module,"module.pack");
